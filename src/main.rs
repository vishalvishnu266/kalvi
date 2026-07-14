mod config;
mod controller;
mod middleware;
mod model;
mod repository;
mod service;
mod util;
mod view;
mod routes;

use tracing::info;
use crate::config::AppState;
use crate::routes::create_router;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Kalvi ERP Rewrite - Initializing State...");
    let state = AppState::new().await?;
    
    let app = create_router(state);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    info!("Server listening on http://localhost:3000");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}
