mod onboarding;
mod home;
mod shared;

use axum::Router;
use ::shared::middleware::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .merge(home::routes())
        .merge(onboarding::routes())
}
