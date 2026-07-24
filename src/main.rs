use std::time::Duration;
use tower::Layer;

use school_erp::health_probes::Readiness;
use school_erp::shutdown::{close_pools, wait_for_signal};
use school_erp::system::{connect_system, migrate_system};
use school_erp::tenancy::new_tenant_registry;
use school_erp::{build_router, AppState, Config, SystemRegistry};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenvy::dotenv();
    let config = Config::from_env();

    init_tracing();
    tracing::debug!("main: application starting");

std::fs::create_dir_all(&config.db_dir).ok();
    std::fs::create_dir_all(config.tenant_db_root()).ok();

    let system_db_url = config.system_db_url();
    tracing::debug!("main: connecting to system db at {}", system_db_url);
    let sys_pool = connect_system(&system_db_url).await?;
    tracing::debug!("main: running system migrations");
    migrate_system(&sys_pool).await?;
    let system = SystemRegistry::new(sys_pool);
    let system_pool_for_shutdown = system.pool_clone();

    let session_db_url = config.session_db_url();
    tracing::debug!("main: initializing session store at {}", session_db_url);
    let sessions = school_erp::session::SessionStore::open(&session_db_url).await?;
    let session_pool_for_shutdown = sessions.pool_clone();

    let state = AppState::new(system, sessions, config.tenant_db_root());
    let state_for_shutdown = state.clone();

tracing::debug!("main: building router");
    let readiness = Readiness::new_ready();
    let readiness_for_shutdown = readiness.clone();
    let app = build_router(state, readiness);
    let app = tower_http::normalize_path::NormalizePathLayer::trim_trailing_slash().layer(app);

let listener = tokio::net::TcpListener::bind(&config.bind_addr).await?;
    tracing::info!(addr = %config.bind_addr, "listening");
    println!("listening on {}", config.bind_addr);

    let shutdown_signal = async move {
        tracing::debug!("shutdown_signal: waiting for signal");
        wait_for_signal().await;
        tracing::debug!("shutdown_signal: signal received, setting readiness to false");
        readiness_for_shutdown.set_ready(false);
        tracing::info!("readiness flipped to false; draining in-flight requests");
    };

    tracing::debug!("main: starting axum server");
    axum::serve(listener, tower::make::Shared::new(app))
    .with_graceful_shutdown(shutdown_signal)
    .await?;

tracing::info!("HTTP server stopped, closing pools");
    tracing::debug!("main: closing database pools");

for (tid, pool) in state_for_shutdown.active_tenant_pools().await {
        tracing::debug!("closing pool for tenant={}", tid);
        pool.close().await;
    }

close_pools(&system_pool_for_shutdown, config.shutdown_timeout).await;
    session_pool_for_shutdown.close().await;

    tracing::info!("shutdown complete");
    Ok(())
}

fn init_tracing() {
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("debug"));
    let layer = fmt::layer()
        .with_target(true)
        .with_level(true)
        .with_file(true)
        .with_line_number(true)

        .with_ansi(true)
        .pretty();
    let _ = tracing_subscriber::registry()
        .with(filter)

        .with(layer)

        .try_init();
}
