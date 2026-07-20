//! HTTP integration test helpers.
//!
//! Builds the full Axum router against:
//! * a fresh in-memory **system DB** (unique per test)
//! * a `TenantRegistry` backed by `InMemoryTenantResolver` — each tenant gets
//!   its own private `sqlite::memory:` database, isolated between tests
//! * `DbTenantGuard` so admin CRUD actually gates traffic
//!
//! Returns an `axum_test::TestServer` you can call like a real HTTP client.

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use axum_test::TestServer;
use school_erp::api::AppState;
use school_erp::build_router;
use school_erp::health_probes::Readiness;
use school_erp::http::middleware::TenantSource;
use school_erp::system::{connect_system, migrate_system, DbTenantGuard, SystemRegistry};
use school_erp::tenancy::{InMemoryTenantResolver, TenantRegistry, TenantRegistryConfig};

static COUNTER: AtomicU64 = AtomicU64::new(0);

fn unique_system_url() -> String {
    let n = COUNTER.fetch_add(1, Ordering::SeqCst);
    format!(
        "sqlite:file:sys_test_{}_{}?mode=memory&cache=shared",
        std::process::id(), n,
    )
}

/// Spin up a fully-wired test server. Each call gives you fresh isolated
/// state (both system + tenants).
pub async fn spawn() -> TestServer {
    let pool = connect_system(&unique_system_url()).await.expect("sys connect");
    migrate_system(&pool).await.expect("sys migrate");
    let system = SystemRegistry::new(pool);

    let tenants = TenantRegistry::new(TenantRegistryConfig {
        resolver: Arc::new(InMemoryTenantResolver),
        guard:    Arc::new(DbTenantGuard { system: system.clone() }),
        auto_migrate: true,
    });

    let state = AppState { system, tenants };
    let router = build_router(state, TenantSource::header_default(), Readiness::new_ready());

    TestServer::new(router).expect("test server")
}

/// Same as `spawn()` but also returns the readiness handle so a test can flip
/// it to simulate shutdown draining.
pub async fn spawn_with_readiness() -> (TestServer, Readiness) {
    use std::sync::atomic::Ordering;
    let _ = Ordering::SeqCst; // silence unused-import when the fn isn't used

    let pool = connect_system(&unique_system_url()).await.expect("sys connect");
    migrate_system(&pool).await.expect("sys migrate");
    let system = SystemRegistry::new(pool);

    let tenants = TenantRegistry::new(TenantRegistryConfig {
        resolver: Arc::new(InMemoryTenantResolver),
        guard:    Arc::new(DbTenantGuard { system: system.clone() }),
        auto_migrate: true,
    });

    let readiness = Readiness::new_ready();
    let state = AppState { system, tenants };
    let router = build_router(state, TenantSource::header_default(), readiness.clone());

    (TestServer::new(router).expect("test server"), readiness)
}
