//! Tiny form + button demo — writes an HTML file straight into the sibling
//! `lit-components/` folder so you can open it in a browser immediately.
//!
//! ```sh
//! cargo run -p lit-ui --example demo
//! # writes -> ../lit-components/dsl-demo.html
//! # open   -> http://localhost:3000/lit-components/dsl-demo.html
//! ```
//!
//! The output path is resolved relative to this file, so it works from
//! anywhere: `cargo run` from repo root, from `rust-dsl/`, or from anywhere
//! else inside the workspace.

use lit_ui::prelude::*;
use std::fs;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    // Build the page purely in Rust — no macros, no templates.
    let html = page()
        .title("lit-ui — DSL demo")

        // Header card with a slotted action button. `Icons::INFO` is a
        // compile-time-checked constant — try changing it to `Icons::INFOO`
        // and you'll get a clean Rust compile error instead of a silently
        // missing icon at runtime.
        .add(card()
            .title("Welcome to lit-ui")
            .subtitle("Macro-free Rust DSL over the Lit component kit")
            .action(button()
                .label("Docs")
                .variant(Variant::Ghost)
                .size(Size::Sm)
                .icon(Icons::INFO))
            .add(Node::text(
                "Everything on this page was built by chaining Rust methods. \
                 The output is plain HTML — no runtime, no framework hooks. \
                 Perfect for Axum / Askama handlers."
            )))

        // Form card demonstrating .add()
        .add(card()
            .title("Add student")
            .add(input().label("Full name").name("fullName")
                        .placeholder("e.g. Aarav Kumar").required())
            .add(input().label("Guardian email").name("email")
                        .kind(InputType::Email).required())
            .add(input().label("Notes").name("notes")
                        .kind(InputType::Textarea)
                        .hint("Optional")))

        // Actions row — use `row_actions()`, the standard preset for every
        // action bar. It's just `row().gap(Md).align(Center)` under the hood,
        // but centralising it guarantees every bar in the app looks identical.
        // Previously passing buttons straight to `card().children(...)` put
        // them into the card's block-flow slot — that's why they stacked and
        // the icon-vs-plain buttons had different vertical alignment.
        .add(card()
            .add(row_actions()
                .add(button().label("Save").variant(Variant::Primary).icon(Icons::CHECK))
                .add(button().label("Cancel").variant(Variant::Secondary))
                .add(divider().vertical()) // visual separator between safe and destructive actions
                .add(button().label("Delete").variant(Variant::Danger).icon(Icons::DELETE))
                .add(spacer())             // push the disabled example to the far right
                .add(button().label("Disabled").disabled())))
        .render();

    // Write next to the Lit components so `npx serve` from the repo root
    // finds it at `/lit-components/dsl-demo.html`.
    let out = output_path("dsl-demo.html");
    fs::create_dir_all(out.parent().unwrap())?;
    fs::write(&out, &html)?;

    eprintln!("wrote {} bytes → {}", html.len(), out.display());
    eprintln!("open  → http://localhost:3000/lit-components/dsl-demo.html");
    Ok(())
}

/// Resolves `<repo-root>/lit-components/<file>` relative to this source file
/// so the example always writes to the right place regardless of the
/// current working directory Cargo was invoked from.
fn output_path(filename: &str) -> PathBuf {
    // CARGO_MANIFEST_DIR = the crate that owns this example (`rust-dsl/`).
    // Its parent is the repo root, and the target folder is `lit-components/`.
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.parent().unwrap().join("lit-components").join(filename)
}
