mod controllers;
mod csrf_middleware;
mod errors;
mod models;
mod public_middleware;
mod routes;
mod services;
mod state;
mod tenant_db_middleware;
mod utils;

use axum::{extract::Request, Router};
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use std::{collections::HashMap, str::FromStr, sync::Arc};
use tokio::sync::RwLock;
use tower_http::{
    cors::{Any, CorsLayer},
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    services::ServeDir,
    trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

use state::AppState;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Ensure data dirs exist.
    std::fs::create_dir_all("data/tenant").ok();

    // Master DB (auto-create if missing).
    let master_opts = SqliteConnectOptions::from_str("sqlite://data/master.db")
        .expect("bad master db url")
        .create_if_missing(true);
    let master_db = SqlitePool::connect_with(master_opts)
        .await
        .expect("Failed to connect to master DB");

    // Run master DB migrations at boot (fail-fast if broken).
    sqlx::migrate!("./resources/migration/master")
        .run(&master_db)
        .await
        .expect("Failed to run master DB migrations");

    let state = AppState {
        master_db,
        tenant_pools: Arc::new(RwLock::new(HashMap::new())),
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app: Router = routes::create_routes(state)
        .nest_service("/public", ServeDir::new("public"))
        .layer(cors)
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
                        request_id = %request_id,
                    )
                })
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind");
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
