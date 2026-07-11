pub mod models;
mod home;
mod onboarding;

use axum::Router;
use shared::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .merge(home::routes())
        .merge(onboarding::routes())
}
