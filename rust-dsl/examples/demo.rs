//! Print a full HTML page to stdout so you can preview the DSL output.
//!
//! ```sh
//! # from repo root, generate a page and open it in a browser
//! cargo run -p lit-ui --example demo > lit-components/dsl-demo.html
//! start http://localhost:3000/lit-components/dsl-demo.html
//! ```
//!
//! The generated page links to `/lit-components/{assets,components}/…`, so
//! serve the repo root with `npx serve` (or `python -m http.server`) and open
//! it via that URL.

use lit_ui::prelude::*;

fn main() {
    // Build the page purely in Rust — no macros, no templates.
    let html = page()
        .title("lit-ui — DSL demo")

        // Header card with a slotted action button
        .add(card()
            .title("Welcome to lit-ui")
            .subtitle("Macro-free Rust DSL over the Lit component kit")
            .padded()
            .action(button()
                .label("Docs")
                .variant(Variant::Ghost)
                .size(Size::Sm)
                .icon("info"))
            .add(Node::text(
                "Everything on this page was built by chaining Rust methods. \
                 The output is plain HTML — no runtime, no framework hooks. \
                 Perfect for Axum / Askama / Hotwire handlers."
            )))

        // Form card demonstrating .add() + .children()
        .add(card()
            .title("Add student")
            .padded()
            .add(input().label("Full name").name("fullName")
                        .placeholder("e.g. Aarav Kumar").required())
            .add(input().label("Guardian email").name("email")
                        .kind(InputType::Email).required())
            .add(input().label("Notes").name("notes")
                        .kind(InputType::Textarea)
                        .hint("Optional")))

        // Actions row — .children() to add several buttons at once
        .add(card()
            .padded()
            .children(vec![
                button().label("Save").variant(Variant::Primary).icon("check"),
                button().label("Cancel").variant(Variant::Secondary),
                button().label("Delete").variant(Variant::Danger).icon("x"),
                button().label("Disabled").disabled(),
            ]))

        .render();

    println!("{html}");
}
