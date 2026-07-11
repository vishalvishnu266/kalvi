use axum::{middleware as axum_middleware, Router};
use shared::{tenant_middleware, AppState, TenantDatabaseManager};
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

    let app = Router::new()
        .merge(tenant::routes())
        .merge(auth::routes())
        // Session middleware needs the tenant pool, so it runs *after*
        // the tenant middleware (middleware layers are applied bottom-up).
        .layer(axum_middleware::from_fn(auth::session_middleware))
        .layer(axum_middleware::from_fn_with_state(
            state.clone(),
            tenant_middleware,
        ))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind");

    println!("🚀 School ERP running at http://localhost:3000");
    println!();
    println!("  Home:      http://localhost:3000/");
    println!("  Onboard:   http://localhost:3000/onboard");
    println!("  Login:     http://localhost:3000/t/<slug>/login");
    println!("  Dashboard: http://localhost:3000/t/<slug>/dashboard");
    println!();

    axum::serve(listener, app).await.expect("server error");
}
