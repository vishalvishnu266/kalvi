use std::time::Duration;

use sqlx::SqlitePool;

pub async fn wait_for_signal() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{signal, SignalKind};
        let mut sigint = signal(SignalKind::interrupt()).expect("install SIGINT");
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

pub async fn close_pools(system_pool: &SqlitePool, step_timeout: Duration) {
    tracing::debug!("shutdown_hook::close_pools");
    let system_close = system_pool.close();
    if tokio::time::timeout(step_timeout, system_close)
        .await
        .is_err()
    {
        tracing::warn!("system pool did not close within {:?}", step_timeout);
    } else {
        tracing::info!("system pool closed");
    }
}
