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

        // Form card demonstrating .add()
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
