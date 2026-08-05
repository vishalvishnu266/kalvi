//! `GET /settings` — stub app for the settings icon. Real content
//! ships with the first user-preferences feature.

use axum::{http::HeaderMap, response::Response};
use lit_ui::core::Node;
use lit_ui::prelude::*;
use ui_shell::{negotiate, Fragment, Fragments, Target};

use crate::shell::chrome;

pub async fn handler(headers: HeaderMap) -> Response {
    let body = card()
        .title("Settings")
        .subtitle("Placeholder")
        .add(Node::text(
            "Settings will host theme, keyboard shortcuts, and \
             per-user preferences. Wire it up when the first preference \
             actually exists.",
        ));

    let frags = Fragments::new().push(Fragment::replace(Target::Main, body));
    negotiate(&headers, "/settings", frags, chrome)
}
