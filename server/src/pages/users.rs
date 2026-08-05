//! `GET /users` — small demo table so we can see a non-trivial DSL
//! page swapped into `main`.

use axum::{http::HeaderMap, response::Response};
use lit_ui::core::Node;
use lit_ui::prelude::*;
use ui_shell::{Fragment, Fragments, Target, negotiate};

use crate::shell::chrome;

pub async fn handler(headers: HeaderMap) -> Response {
    let list = column()
        .gap(Gap::Sm)
        .add(list_item("Aarav Kumar").subtitle("aarav@example.com"))
        .add(list_item("Bilal Ahmed").subtitle("bilal@example.com"))
        .add(list_item("Chitra Rao").subtitle("chitra@example.com"));

    let body = card()
        .title("Users")
        .action(button().label("Add user").variant(Variant::Primary))
        .add(list)
        .add(Node::text(
            "In milestone 8, saying \"add user\" in the copilot will \
             return a form fragment targeting `main` — no different \
             from clicking the button above.",
        ));

    let frags = Fragments::new().push(Fragment::replace(Target::Main, body));
    negotiate(&headers, "/users", frags, chrome)
}
