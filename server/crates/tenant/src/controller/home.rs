use axum::{
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use shared::AppState;
use crate::view::home::home_page;

pub async fn show_home() -> Response {
    home_page().into_response()
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(show_home))
}
