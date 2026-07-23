//! Graceful shutdown primitives.
//!
//! Provides a cross-platform "wait for termination signal" future and an
//! ordered pool-cleanup helper. Used by `main.rs` and by any custom binary
//! that wants the same shutdown semantics.
//!
//! Order of operations we implement:
//!
//! 1. Wait for SIGINT or SIGTERM.
//! 2. Return from the future so `axum::serve(...).with_graceful_shutdown(...)`
//!    stops accepting new connections and drains in-flight requests.
//! 3. Close every per-tenant pool (checkpoints each tenant's WAL).
//! 4. Close the system pool (checkpoints `system.db`'s WAL).

use std::time::Duration;

use sqlx::SqlitePool;

use crate::tenancy::{tenant_shutdown, TenantRegistry};

/// Resolve when the process receives a termination signal.
///
/// * Unix: `SIGINT` (Ctrl+C) **or** `SIGTERM` (Docker/K8s stop).
/// * Windows: Ctrl+C.
pub async fn wait_for_signal() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{signal, SignalKind};
        let mut sigint  = signal(SignalKind::interrupt()).expect("install SIGINT");
        let mut sigterm = signal(SignalKind::terminate()).expect("install SIGTERM");
        tokio::select! {
            _ = sigint.recv()  => tracing::info!("received SIGINT, shutting down"),
            _ = sigterm.recv() => tracing::info!("received SIGTERM, shutting down"),
        }
    }
    #[cfg(not(unix))]
    {
        let _ = tokio::signal::ctrl_c().await;
        tracing::info!("received Ctrl+C, shutting down");
    }
}

/// Close per-tenant pools first, then the system pool, with a per-step
/// timeout so a stuck pool cannot hang shutdown forever.
pub async fn close_pools(
    tenants: &TenantRegistry,
    system_pool: &SqlitePool,
    step_timeout: Duration,
) {
    tracing::debug!("close_pools: starting shutdown sequence");
    // 1. Tenants — checkpoints each tenant's WAL.
    tracing::debug!("close_pools: closing tenant pools");
    let tenants_shutdown = tenant_shutdown(tenants);
    if tokio::time::timeout(step_timeout, tenants_shutdown).await.is_err() {
        tracing::warn!("tenant pools did not close within {:?}", step_timeout);
    } else {
        tracing::info!("tenant pools closed");
    }

    // 2. System pool.
    tracing::debug!("close_pools: closing system pool");
    let system_close = system_pool.close();
    if tokio::time::timeout(step_timeout, system_close).await.is_err() {
        tracing::warn!("system pool did not close within {:?}", step_timeout);
    } else {
        tracing::info!("system pool closed");
    }
    tracing::debug!("close_pools: shutdown sequence complete");
}
