use axum::{
    routing::{get, post},
    Router,
};
use crate::config::AppState::AppState;
use crate::controllers::{
    LoginController, LogoutController, DashboardController,
    HomeController, OnboardingController, SettingsController
};

pub fn app_routes() -> Router<AppState> {
    Router::new()
        // Public routes
        .route("/", get(HomeController::show_home))
        .route("/onboard", get(OnboardingController::show_form))
        .route("/onboard", post(OnboardingController::submit_form))

        // Identity and Auth routes
        .route("/t/{tenant_slug}/login", get(LoginController::show_login))
        .route("/t/{tenant_slug}/login", post(LoginController::process_login))
        .route("/t/{tenant_slug}/logout", post(LogoutController::process_logout))

        // Institution / Tenant routes
        .route("/t/{tenant_slug}/dashboard", get(DashboardController::show_dashboard))
        .route("/t/{tenant_slug}/settings", get(SettingsController::show_settings))
        .route("/t/{tenant_slug}/settings", post(SettingsController::update_settings))
}
