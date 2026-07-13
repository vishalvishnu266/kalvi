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
    tracing_subscriber::fmt::init();

    let db_config: DatabaseConfig = DatabaseConfig::new().await.expect("Failed to initialize database");
    let state = AppState { db: db_config };

    let app = create_router(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
