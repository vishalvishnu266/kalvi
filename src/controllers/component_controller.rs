use axum::response::Html;
use crate::views::{layout, storybook};

pub async fn storybook_handler() -> Html<String> {
    Html(layout::base_layout("Component Storybook", &storybook::render()))
}
