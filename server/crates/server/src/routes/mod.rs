pub mod auth;
pub mod tenant;

use axum::Router;
use shared::AppState;

pub fn app_routes() -> Router<AppState> {
    Router::new()
        .merge(auth::routes())
        .merge(tenant::routes())
}
