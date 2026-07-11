use axum::{
    response::{IntoResponse, Response},
};
use shared::AppState;
use crate::view::home::home_page;

pub async fn show_home() -> Response {
    home_page().into_response()
}

