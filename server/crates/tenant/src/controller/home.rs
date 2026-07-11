use axum::{
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use shared::AppState;
use crate::view::home::home_page;

pub async fn show_home() -> Response {
    Html(home_page().into_string()).into_response()
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(show_home))
}
