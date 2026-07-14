use axum::response::{Html, Redirect};
use axum::extract::Path;
use crate::views::{layout, storybook};

pub async fn storybook_root_handler() -> Redirect {
    Redirect::to("/storybook/1")
}

pub async fn storybook_handler(Path(page): Path<u32>) -> Html<String> {
    let title = match page {
        1 => "Dashboard Storybook",
        2 => "Form Elements Storybook",
        _ => "Component Storybook",
    };
    Html(layout::base_layout(title, &storybook::render(page)))
}
