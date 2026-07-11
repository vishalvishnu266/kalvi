use axum::{
    routing::{get, post},
    Router,
};
use shared::AppState;
use tenant::controller::{home, onboarding};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(home::show_home))
        .route("/onboard", get(onboarding::show_form))
        .route("/onboard", post(onboarding::submit_form))
}
