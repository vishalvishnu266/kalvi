//! # server — minimal Axum test harness for the `lit-ui` Rust DSL
//!
//! This crate has exactly one job: prove that a page composed with
//! [`lit_ui`] renders correctly in a real browser against the Lit
//! web components living in `../lit-components/`.
//!
//! ## Routes
//!
//! * `GET /`                    — a demo page built with the DSL.
//! * `GET /health`              — liveness probe (returns `"ok"`).
//! * `GET /lit-components/*`    — static files (CSS tokens + component JS).
//!
//! ## Design notes
//!
//! * The server does **not** import anything from the previous `school_erp`
//!   codebase; it stands alone.
//! * `rust-dsl` (`lit-ui`) is used as a plain library dependency — no
//!   modifications were made to that crate.
//! * Static file serving lets the DSL output actually run: the HTML uses
//!   `<ui-*>` custom elements that are defined by JS modules in
//!   `../lit-components/components/`.

use axum::{
    Router,
    response::Html,
    routing::get,
};
use lit_ui::prelude::*;
use std::net::SocketAddr;
use std::path::PathBuf;
use tower_http::{services::ServeDir, trace::TraceLayer};

/// Address the server binds to. Kept as a module-level constant so it is
/// easy to spot and change during development.
const BIND_ADDR: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main() {
    // Basic tracing subscriber; controlled by `RUST_LOG`.
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,tower_http=info".into()),
        )
        .init();

    // Path to the sibling `lit-components/` directory, resolved relative
    // to this crate's manifest so `cargo run -p server` works from any cwd.
    let lit_components_dir: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("server crate must have a parent (workspace root)")
        .join("lit-components");

    let app = Router::new()
        .route("/", get(index))
        .route("/health", get(health))
        // Serve the Lit component kit as static assets so the DSL's HTML
        // output can load `./components/index.js`, tokens.css, etc.
        .nest_service(
            "/lit-components",
            ServeDir::new(&lit_components_dir),
        )
        .layer(TraceLayer::new_for_http());

    let addr: SocketAddr = BIND_ADDR.parse().expect("valid bind address");
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind TCP listener");

    tracing::info!("🚀  server listening on http://{addr}");
    tracing::info!("📄  demo page   → http://{addr}/");
    tracing::info!("🧩  components  → http://{addr}/lit-components/components/index.js");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("server error");
}

/// Simple liveness probe. Kept as plain text so `curl` output is readable.
async fn health() -> &'static str {
    "ok"
}

/// The demo page — built entirely with the `lit-ui` DSL and returned as
/// HTML. Serves as the canonical smoke test that:
///
/// 1. The DSL compiles and links against this server.
/// 2. The generated markup mounts correctly against the Lit components
///    served from `/lit-components/`.
async fn index() -> Html<String> {
    // `page()` emits a full `<!doctype html>` document. It expects the
    // component bundle to be reachable at a base URL — we point it at the
    // `/lit-components/` route that ServeDir exposes above.
    let html = page()
        .title("lit-ui × Axum — DSL smoke test")
        .assets_base("/lit-components")
        .add(
            card()
                .title("It works 🎉")
                .subtitle("Rendered by lit-ui, served by Axum")
                .add(Node::text(
                    "This page was composed in Rust with the macro-free \
                     lit-ui DSL and served by a minimal Axum handler. \
                     The interactive bits below are real Lit web components \
                     loaded from /lit-components/.",
                )),
        )
        .add(
            card()
                .title("Try the DSL")
                .add(input()
                    .label("Full name")
                    .name("fullName")
                    .placeholder("e.g. Aarav Kumar")
                    .required())
                .add(input()
                    .label("Guardian email")
                    .name("email")
                    .kind(InputType::Email)
                    .required())
                .add(row_actions()
                    .add(button()
                        .label("Save")
                        .variant(Variant::Primary)
                        .icon(Icons::CHECK))
                    .add(button()
                        .label("Cancel")
                        .variant(Variant::Secondary))),
        )
        .render();

    Html(html)
}

/// Waits for Ctrl-C so `axum::serve(..).with_graceful_shutdown(..)` can
/// drain in-flight requests instead of dropping them on the floor.
async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
    tracing::info!("shutdown signal received — draining connections");
}
