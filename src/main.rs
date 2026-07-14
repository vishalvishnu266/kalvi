mod controllers;
mod errors;
mod views;
mod routes;
mod state;
mod middleware;

use axum::{extract::Request, Router};
use sqlx::SqlitePool;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use tower_http::{
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::{TraceLayer, DefaultOnRequest, DefaultOnResponse},
};
use tracing::Level;
use state::AppState;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Initialize master DB
    let master_db_url = "sqlite://data/master.db";
    // For demo purposes, we ignore error if db doesn't exist yet, 
    // but in real app we should handle it.
    let master_db = SqlitePool::connect(master_db_url).await.expect("Failed to connect to master DB");

    let state = AppState {
        master_db,
        tenant_pools: Arc::new(RwLock::new(HashMap::new())),
    };

    // Build our application with a route
    let app = routes::create_routes(state)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request| {
                    let request_id = request
                        .headers()
                        .get("x-request-id")
                        .and_then(|v| v.to_str().ok())
                        .unwrap_or("unknown");
                    tracing::info_span!(
                        "request",
                        method = %request.method(),
                        uri = %request.uri(),
                        version = ?request.version(),
                        request_id = %request_id,
                    )
                })
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid));

    // Run our application
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
