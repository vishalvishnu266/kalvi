use axum::response::Html;
use crate::view::HomeView;

pub async fn show_home() -> Html<String> {
    Html(HomeView::render())
}
