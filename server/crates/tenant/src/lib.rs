pub mod controller;
pub mod model;
pub mod repository;
pub mod view;

use axum::Router;
use shared::AppState;

pub fn routes() -> Router<AppState> {
    controller::routes()
}
