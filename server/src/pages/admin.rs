//! `GET /admin` — admin landing. Same pattern as every other page.

use axum::{http::HeaderMap, response::Response};
use lit_ui::core::Node;
use lit_ui::prelude::*;
use ui_shell::{Fragment, Fragments, Target, negotiate};

use crate::shell::chrome;

pub async fn handler(headers: HeaderMap) -> Response {
    let body = card()
        .title("Admin")
        .subtitle("System-wide settings")
        .add(Node::text(
            "You navigated here via an island swap. Notice the topbar and \
             sidebar never re-rendered — only the main island did. \
             Refresh the page to confirm this URL is directly loadable \
             (the server returns the full shell + this fragment).",
        ));

    let frags = Fragments::new().push(Fragment::replace(Target::Main, body));
    negotiate(&headers, frags, chrome)
}
