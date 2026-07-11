pub mod login;
pub mod logout;
pub mod dashboard;

use axum::Router;
use shared::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .merge(login::routes())
        .merge(logout::routes())
        .merge(dashboard::routes())
}
