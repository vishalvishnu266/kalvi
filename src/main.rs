//! Reference binary that wires the whole thing together with **graceful
//! shutdown**.
//!
//! Environment variables:
//! * `SYSTEM_DB_URL`      (default: `sqlite://data/system.db?mode=rwc`)
//! * `TENANT_DB_ROOT`     (default: `data/tenants`)
//! * `BIND`               (default: `0.0.0.0:3000`)
//! * `SHUTDOWN_TIMEOUT_S` (default: `30`) — per-step timeout for pool close
//!
//! Shutdown lifecycle:
//! 1. Wait for SIGINT (Ctrl+C) **or** SIGTERM (Docker/K8s).
//! 2. `axum::serve(...).with_graceful_shutdown(...)` stops accepting new
//!    connections and drains in-flight requests.
//! 3. Close every per-tenant pool (checkpoints each tenant's WAL).
//! 4. Close the system pool (checkpoints `system.db`'s WAL).
//! 5. Exit 0.

use std::sync::Arc;
use std::time::Duration;

use school_erp::health_probes::Readiness;
use school_erp::http::middleware::TenantSource;
use school_erp::shutdown::{close_pools, wait_for_signal};
use school_erp::system::{connect_system, migrate_system, DbTenantGuard};
use school_erp::tenancy::{FileTenantResolver, TenantRegistry, TenantRegistryConfig};
use school_erp::{build_router, AppState, SystemRegistry};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();

    // --- Config ---
    let system_url = std::env::var("SYSTEM_DB_URL")
        .unwrap_or_else(|_| "sqlite://data/system.db?mode=rwc".to_string());
    let tenant_root = std::env::var("TENANT_DB_ROOT")
        .unwrap_or_else(|_| "data/tenants".to_string());
    let bind = std::env::var("BIND").unwrap_or_else(|_| "0.0.0.0:3000".to_string());
    let shutdown_timeout = std::env::var("SHUTDOWN_TIMEOUT_S")
        .ok().and_then(|v| v.parse::<u64>().ok())
        .map(Duration::from_secs)
        .unwrap_or_else(|| Duration::from_secs(30));

    // Ensure data directories exist.
    std::fs::create_dir_all("data").ok();
    std::fs::create_dir_all(&tenant_root).ok();

    // --- 1. Central DB ---
    let sys_pool = connect_system(&system_url).await?;
    migrate_system(&sys_pool).await?;
    let system = SystemRegistry::new(sys_pool);
    let system_pool_for_shutdown = system.pool_clone();

    // --- 2. Tenant registry ---
    let cfg = TenantRegistryConfig {
        resolver: Arc::new(FileTenantResolver::new(&tenant_root)),
        guard:    Arc::new(DbTenantGuard { system: system.clone() }),
        auto_migrate: true,
    };
    let tenants = TenantRegistry::new(cfg);
    let tenants_for_shutdown = tenants.clone();

    // --- 3. Router ---
    let readiness = Readiness::new_ready();
    let readiness_for_shutdown = readiness.clone();
    let state = AppState { system, tenants };
    let app = build_router(state, TenantSource::header_default(), readiness);

    // --- 4. Serve with graceful shutdown ---
    let listener = tokio::net::TcpListener::bind(&bind).await?;
    tracing::info!(addr = %bind, "listening");
    println!("listening on {bind}");

    // Combine: on signal → flip readiness=false (so LB stops sending traffic)
    // → then let axum drain in-flight requests.
    let shutdown_signal = async move {
        wait_for_signal().await;
        readiness_for_shutdown.set_ready(false);
        tracing::info!("readiness flipped to false; draining in-flight requests");
    };

    // `app` is a `NormalizePath<Router>`. `axum::serve` accepts anything
    // implementing `IntoMakeService`, so we convert via `tower`'s
    // `ServiceExt::into_make_service` — the same shape axum uses internally
    // for a bare `Router`, but this version preserves the outer
    // `NormalizePathLayer` that rewrites trailing slashes before routing.
    use tower::ServiceExt;
    use axum::extract::Request;
    axum::serve(
        listener,
        ServiceExt::<Request>::into_make_service(app),
    )
    .with_graceful_shutdown(shutdown_signal)
    .await?;

    // --- 5. Requests have drained; close pools in order ---
    tracing::info!("HTTP server stopped, closing pools");
    close_pools(&tenants_for_shutdown, &system_pool_for_shutdown, shutdown_timeout).await;

    tracing::info!("shutdown complete");
    Ok(())
}

fn init_tracing() {
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    // Respect RUST_LOG (e.g. `RUST_LOG=school_erp=info,tower_http=debug`).
    // Default to `info` if nothing is set.
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    // Ignore error if a subscriber was already installed (e.g. by tests).
    let _ = tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().with_target(true).with_level(true))
        .try_init();
}
