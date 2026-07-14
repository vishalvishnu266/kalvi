use axum::{
    routing::{get, post},
    Router,
    middleware,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use kalvi::config::database_config::DatabaseConfig;
use kalvi::middleware::request_id_middleware::request_id_middleware;
use kalvi::middleware::tenant_middleware::tenant_middleware;
use kalvi::middleware::auth_middleware::auth_middleware;
use kalvi::controller::student_controller::StudentController;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize Tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "kalvi=debug,tower_http=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 2. Initialize Database Config (Master pool + Migrations)
    let db_config = Arc::new(DatabaseConfig::new("sqlite://data/master.db?mode=rwc").await?);

    // 3. Define Routes
    
    // Public / SaaS Routes (e.g., registration) would go here
    
    // Tenant-Specific Protected Routes
    let tenant_routes = Router::new()
        .route("/student/add", get(StudentController::get_form))
        .route("/student/add", post(StudentController::post_student))
        // Apply Auth Middleware (Session check)
        .layer(middleware::from_fn(auth_middleware))
        // Apply Tenant Middleware (Resolve pool from :slug)
        .layer(middleware::from_fn_with_state(db_config.clone(), tenant_middleware));

    let app = Router::new()
        .nest("/web/:slug", tenant_routes)
        // Root / Health check
        .route("/", get(|| async { "Kalvi ERP v2" }))
        // Global Request ID tracking
        .layer(middleware::from_fn(request_id_middleware))
        .with_state(db_config);

    // 4. Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Server listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
