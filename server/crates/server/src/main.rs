use axum::{Router, middleware as axum_middleware};
use shared::{TenantDatabaseManager, middleware::{AppState, tenant_middleware}};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let db_manager = Arc::new(TenantDatabaseManager::new("test"));
    let state = AppState { db_manager };

    let app = Router::new()
        .merge(student::routes())
        .layer(axum_middleware::from_fn_with_state(
            state.clone(),
            tenant_middleware,
        ))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Server listening on http://localhost:3000");
    println!("📚 Try: http://localhost:3000/student/1");
    axum::serve(listener, app).await.unwrap();
}
