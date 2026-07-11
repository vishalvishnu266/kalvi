use axum::{
    routing::{get, post},
    Router,
};
use crate::config::AppState::AppState;
use crate::controllers::{HomeController, OnboardingController, SettingsController};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(HomeController::show_home))
        .route("/onboard", get(OnboardingController::show_form))
        .route("/onboard", post(OnboardingController::submit_form))
        .route("/t/{tenant_slug}/settings", get(SettingsController::show_settings))
        .route("/t/{tenant_slug}/settings", post(SettingsController::update_settings))
}
