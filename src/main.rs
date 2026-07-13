mod config;
mod controller;
mod middleware;
mod model;
mod repository;
mod service;
mod util;
mod routes;
mod view;

use std::net::SocketAddr;
use crate::config::{AppState, DatabaseConfig};
use crate::routes::create_router;

#[tokio::main]
async fn main() {
    // Advanced Logging Setup
    let is_production = std::env::var("APP_ENV").unwrap_or_else(|_| "development".into()) == "production";
    
    if is_production {
        tracing_subscriber::fmt()
            .json() // Machine-readable for Datadog/ELK
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .init();
    }

    let db_config: DatabaseConfig = DatabaseConfig::new().await.expect("Failed to initialize database");
    let limiter = crate::middleware::rate_limit_middleware::RateLimiter::new(
        10, // 10 requests
        std::time::Duration::from_secs(60) // per minute
    );
    let state = AppState { db: db_config, limiter };

    let app = create_router(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
