mod routes;
mod controllers;
mod models;
mod repositories;
mod views;
mod services;
mod middleware;
mod web_utils;
mod config;

use axum::middleware as axum_middleware;
use crate::config::DatabaseManager::TenantDatabaseManager;
use crate::config::AppState::AppState;
use crate::middleware::{TenantMiddleware, SessionMiddleware, AuthenticationMiddleware};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let db_manager = Arc::new(
        TenantDatabaseManager::new()
            .await
            .expect("Failed to initialize database manager"),
    );

    let state = AppState {
        db_manager: db_manager.clone(),
    };

    let app = routes::app_routes()
        // Auth check runs after session extraction
        .layer(axum_middleware::from_fn(AuthenticationMiddleware::require_auth_middleware))
        .layer(axum_middleware::from_fn_with_state(state.clone(), SessionMiddleware::session_middleware))
        .layer(axum_middleware::from_fn_with_state(state.clone(), TenantMiddleware::tenant_middleware))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
