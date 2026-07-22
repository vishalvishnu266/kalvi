//! Reference binary that wires the whole thing together with **graceful
//! shutdown**.
//!
//! Environment variables:
//! * `SYSTEM_DB_URL`      (default: `sqlite://data/system.db?mode=rwc`)
//! * `TENANT_DB_ROOT`     (default: `data/tenants`)
//! * `BIND`               (default: `0.0.0.0:3000`)
//! * `SHUTDOWN_TIMEOUT_S` (default: `30`) — per-step timeout for pool close

use std::sync::Arc;
use std::time::Duration;

use school_erp::health_probes::Readiness;
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

    // --- 3. Router (all routes live in src/http/routes.rs) ---
    let readiness = Readiness::new_ready();
    let readiness_for_shutdown = readiness.clone();
    let state = AppState { system, tenants };
    let app = build_router(state, readiness);

    // --- 4. Serve with graceful shutdown ---
    let listener = tokio::net::TcpListener::bind(&bind).await?;
    tracing::info!(addr = %bind, "listening");
    println!("listening on {bind}");

    let shutdown_signal = async move {
        wait_for_signal().await;
        readiness_for_shutdown.set_ready(false);
        tracing::info!("readiness flipped to false; draining in-flight requests");
    };

    // `app` is `NormalizePath<Router>`. Convert via tower's ServiceExt so
    // `axum::serve` accepts it while preserving the outer NormalizePathLayer.
    use tower::ServiceExt;
    use axum::extract::Request;
    axum::serve(
        listener,
        ServiceExt::<Request>::into_make_service(app),
    )
    .with_graceful_shutdown(shutdown_signal)
    .await?;

    // --- 5. Requests drained; close pools in order ---
    tracing::info!("HTTP server stopped, closing pools");
    close_pools(&tenants_for_shutdown, &system_pool_for_shutdown, shutdown_timeout).await;

    tracing::info!("shutdown complete");
    Ok(())
}

fn init_tracing() {
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    let _ = tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().with_target(true).with_level(true))
        .try_init();
}
