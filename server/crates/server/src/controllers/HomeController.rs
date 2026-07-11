use axum::response::{IntoResponse, Response};
use crate::views::HomeView;

pub async fn show_home() -> Response {
    HomeView::render_home().into_response()
}
