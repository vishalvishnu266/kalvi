# server — Axum test harness for `lit-ui`

A **minimal** Axum server whose only purpose is to exercise the `lit-ui`
Rust DSL (`../rust-dsl/`) end-to-end in a real browser.

It does **not** carry over anything from the previous `school_erp` app —
no database, no tenancy, no auth. Just:

1. Build a page in Rust using the DSL.
2. Return it as HTML.
3. Serve the Lit web components as static files so the page hydrates.

## Run

```bash
# From the repo root
cargo run -p server

# Then open
#   http://localhost:3000/           → DSL-rendered demo page
#   http://localhost:3000/health     → "ok"
```

## Layout

```
server/
├── Cargo.toml       # deps: axum, tokio, tower-http, lit-ui (path)
└── src/main.rs      # routes + shutdown handling
```

## Adding a new page

Add a handler in `src/main.rs` that builds a page with `lit_ui::prelude::*`
and returns `Html<String>`, then wire it into the `Router` in `main`.

```rust
async fn students() -> Html<String> {
    let html = page()
        .title("Students")
        .assets_base("/lit-components")
        .add(card().title("Roster").add(/* … */))
        .render();
    Html(html)
}
```

## Rules

* **Never modify `rust-dsl/`** from here. This crate consumes it as a
  library and nothing more. If the DSL is missing something, add it in
  `rust-dsl/` in a separate change.
* Keep dependencies tight — this is a test harness, not a framework.
