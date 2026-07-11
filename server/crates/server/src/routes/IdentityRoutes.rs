use axum::{
    routing::{get, post},
    Router,
};
use crate::config::AppState::AppState;
use crate::controllers::{LoginController, LogoutController, DashboardController};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/t/{tenant_slug}/login", get(LoginController::show_login))
        .route("/t/{tenant_slug}/login", post(LoginController::process_login))
        .route("/t/{tenant_slug}/logout", post(LogoutController::process_logout))
        .route("/t/{tenant_slug}/dashboard", get(DashboardController::show_dashboard))
}
