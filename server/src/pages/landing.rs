//! `GET /` — the landing page. Explains the framework and gives the
//! user something to click on so they can see island navigation work.

use axum::{http::HeaderMap, response::Response};
use lit_ui::core::Node;
use lit_ui::prelude::*;
use ui_shell::{Fragment, Fragments, Target, negotiate};

use crate::shell::chrome;

pub async fn handler(headers: HeaderMap) -> Response {
    let body = card()
        .title("Welcome 👋")
        .subtitle("Backend-driven, fragment-based, AI-friendly")
        .add(Node::text(
            "This page lives inside <ui-app-shell>'s `main` slot. \
             Click any sidebar link — only the main island will \
             swap and the URL bar will update. No full-page reload.",
        ))
        .add(row_actions()
            .add(button().label("Go to dashboard").variant(Variant::Primary))
            .add(button().label("Open admin").variant(Variant::Secondary)));

    let frags = Fragments::new().push(Fragment::replace(Target::Main, body));
    negotiate(&headers, "/", frags, chrome)
}
