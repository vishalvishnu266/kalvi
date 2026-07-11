use axum::{
    routing::{get, post},
    Router,
};
use shared::AppState;
use auth::controller::{login, logout, dashboard};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/t/{tenant_slug}/login", get(login::show_login))
        .route("/t/{tenant_slug}/login", post(login::process_login))
        .route("/t/{tenant_slug}/logout", post(logout::process_logout))
        .route("/t/{tenant_slug}/dashboard", get(dashboard::show_dashboard))
}
