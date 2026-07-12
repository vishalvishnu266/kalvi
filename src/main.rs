mod config;
mod controller;
mod middleware;
mod model;
mod repository;
mod util;
mod view;

use axum::{
    routing::{get, post},
    Router,
    middleware as axum_middleware,
};
use std::net::SocketAddr;
use crate::config::AppState::AppState;
use crate::config::DatabaseConfig::DatabaseConfig;
use crate::controller::*;
use crate::middleware::TenantMiddleware::tenant_middleware;
use crate::middleware::AuthMiddleware::auth_middleware;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let db_config: DatabaseConfig = DatabaseConfig::new().await.expect("Failed to initialize database");
    let state = AppState { db: db_config };

    let tenant_api_routes = Router::new()
        .route("/health", get(ApiController::health));

    let tenant_web_routes = Router::new()
        .route("/dashboard", get(DashboardController::show_dashboard))
        .route("/settings", get(SettingsController::show_settings).post(SettingsController::process_settings))
        .route("/logout", post(LogoutController::process_tenant_logout))
        .layer(axum_middleware::from_fn(auth_middleware));

    let app: Router = Router::new()
        // Public Routes
        .route("/", get(HomeController::show_home))
        .route("/login", get(LoginController::show_common_login).post(LoginController::process_common_login))
        .route("/logout", post(LogoutController::process_logout))
        .route("/registration", get(OnboardingController::show_form).post(OnboardingController::submit_form))
        .route("/pages/{*path}", get(|| async { axum::response::Html("<h1>Public Page Placeholder</h1>") }))
        
        // SaaS Control Plane
        .route("/saas/onboard", get(SaasController::show_onboard).post(SaasController::process_onboard))
        .route("/saas/login", get(SaasController::show_login).post(SaasController::process_login))
        
        // Tenant Routes (/{slug})
        .nest("/{slug}/api", tenant_api_routes)
        .nest("/{slug}", tenant_web_routes)
        .route("/{slug}/login", get(LoginController::show_login).post(LoginController::process_login))
        
        // Global Middlewares
        .layer(axum_middleware::from_fn_with_state(state.clone(), tenant_middleware))
        .fallback(|| async { axum::response::Redirect::to("/") })
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
