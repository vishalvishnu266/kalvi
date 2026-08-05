//! `GET /dashboard` — a couple of stats + a card. Whole page is one
//! `main` fragment; the shell is untouched on nav.

use axum::{http::HeaderMap, response::Response};
use lit_ui::core::Node;
use lit_ui::prelude::*;
use ui_shell::{negotiate, Fragment, Fragments, Target};

use crate::shell::chrome;

pub async fn handler(headers: HeaderMap) -> Response {
    let stats = row()
        .gap(Gap::Md)
        .add(stat("Users",         "128"))
        .add(stat("Active today",  "42"))
        .add(stat("Errors (24h)",  "0"));

    let body = column()
        .gap(Gap::Lg)
        .add(stats)
        .add(card()
            .title("Dashboard")
            .add(Node::text("A rich page composed entirely with rust-dsl.")));

    let frags = Fragments::new().push(Fragment::replace(Target::Main, body));
    negotiate(&headers, "/dashboard", frags, chrome)
}
