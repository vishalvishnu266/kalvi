//! `GET /reports` — stub app so the Reports icon in the activity bar
//! lands somewhere. Real content ships with the first reports feature.

use axum::{http::HeaderMap, response::Response};
use lit_ui::core::Node;
use lit_ui::prelude::*;
use ui_shell::{negotiate, Fragment, Fragments, Target};

use crate::shell::chrome;

pub async fn handler(headers: HeaderMap) -> Response {
    let body = card()
        .title("Reports")
        .subtitle("Placeholder")
        .add(Node::text(
            "Reports app is registered in the activity bar / tab bar. \
             Real content lands when the first reports feature ships.",
        ));

    let frags = Fragments::new().push(Fragment::replace(Target::Main, body));
    negotiate(&headers, "/reports", frags, chrome)
}
