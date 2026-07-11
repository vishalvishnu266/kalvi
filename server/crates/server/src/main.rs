use axum::{Router, middleware as axum_middleware};
use shared::{TenantDatabaseManager, middleware::{AppState, tenant_middleware}};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Initialize database manager with master database
    let db_manager = Arc::new(
        TenantDatabaseManager::new()
            .await
            .expect("Failed to initialize database manager")
    );
    
    let state = AppState { db_manager: db_manager.clone() };

    let app = Router::new()
        .merge(tenant::routes())      // Tenant onboarding routes (control plane)
        .merge(auth::routes())        // Authentication routes (tenant-scoped)
        // Note: student::routes() currently empty - test endpoints removed
        // Add real student management features as needed
        .layer(axum_middleware::from_fn(auth::session_middleware))  // Custom session middleware
        .layer(axum_middleware::from_fn_with_state(
            state.clone(),
            tenant_middleware,
        ))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 School ERP Server listening on http://localhost:3000");
    println!();
    println!("📋 Onboard a tenant: http://localhost:3000/onboard");
    println!("🔐 Login (tenant-scoped): http://localhost:3000/t/{{tenant-slug}}/login");
    println!("📊 Dashboard: http://localhost:3000/t/{{tenant-slug}}/dashboard");
    println!();
    println!("ℹ️  Custom session system active!");
    println!("💡 Each tenant has isolated users, sessions, and data");
    axum::serve(listener, app).await.unwrap();
}
