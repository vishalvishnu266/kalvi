//! # server — a Rust-DSL-driven Storybook for the Lit primitives kit.
//!
//! Two routes:
//! * `GET /`                — the storybook page (rendered by `lit_ui::page()`).
//! * `GET /lit-components/*` — static assets (stylesheets + component modules).
//!
//! The storybook composes every primitive purely through the `lit_ui`
//! DSL — there are no `<html>` string literals in the request handler.
//! Even the outer envelope is `page().title(...).add(...).render()`.

mod stories;

use axum::{response::Html, routing::get, Router};
use lit_ui::core::Component;
use lit_ui::prelude::*;
use std::net::SocketAddr;
use std::path::PathBuf;
use tower_http::{services::ServeDir, trace::TraceLayer};

const BIND_ADDR: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,tower_http=info".into()),
        )
        .init();

    // Resolve `../lit-components/` relative to the server crate so
    // `cargo run -p server` works from any working directory.
    let lit_components_dir: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("server crate must have a workspace parent")
        .join("lit-components");

    let app = Router::new()
        .route("/", get(storybook))
        // Serve the whole lit-components tree at /lit-components/*. This
        // matches the default asset base of `lit_ui::page()`, so no
        // `.assets_base(...)` override is needed.
        .nest_service("/lit-components", ServeDir::new(&lit_components_dir))
        .layer(TraceLayer::new_for_http());

    let addr: SocketAddr = BIND_ADDR.parse().expect("valid bind address");
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind TCP listener");

    tracing::info!("📚  Storybook up at http://{addr}/");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("server error");
}

/// GET / — the storybook page. Every UI element (including labels and
/// separators) is composed from the `lit_ui` DSL.
async fn storybook() -> Html<String> {
    // Build one big page: the outer envelope from `page()`, plus a
    // vertical `<ui-stack>` of sections, each containing a heading
    // (rendered via `<ui-badge>` since we don't have a heading primitive
    // yet) followed by its list of named stories inside a two-column
    // `<ui-columns>` (label on the left, rendered story on the right).
    let mut root_stack = StackBoxed::default();

    // Page title as a big brand badge — no `<h1>` primitive yet, so this
    // keeps the "DSL only" rule.
    root_stack = root_stack.push(Box::new(
        badge("lit-ui storybook").tone(Tone::Brand),
    ));

    for section in stories::all_sections() {
        // Section heading.
        root_stack = root_stack.push(Box::new(
            badge(section.title).tone(Tone::Info),
        ));

        // Rows of (label, story) built as a two-column `<ui-columns>`.
        for (name, comp) in section.stories {
            root_stack = root_stack.push(Box::new(StoryRow {
                label: name.to_string(),
                comp,
            }));
        }
    }

    // Center it in a readable column and pad the edges so the storybook
    // doesn't run right up against the viewport.
    let centered = Centered { max_w: "960px", padded: true, inner: Box::new(root_stack) };

    let html = page()
        .title("lit-ui storybook")
        .add(centered)
        .render();
    Html(html)
}

// ── Local DSL wrappers ─────────────────────────────────────────────────
// These are the same layout wrappers we use in stories.rs — kept here
// because `main.rs` composes the page itself and needs them too. Each
// implements `Component` so it plugs into `page().add(...)`.

/// Vertical stack that accepts boxed children.
#[derive(Default)]
struct StackBoxed { children: Vec<Box<dyn Component>> }
impl StackBoxed {
    fn push(mut self, c: Box<dyn Component>) -> Self { self.children.push(c); self }
}
impl Component for StackBoxed {
    fn render(&self) -> String {
        let body: String = self.children.iter().map(|c| c.render()).collect();
        format!(r#"<ui-stack gap="md">{body}</ui-stack>"#)
    }
}

/// Centered readable column.
struct Centered { max_w: &'static str, padded: bool, inner: Box<dyn Component> }
impl Component for Centered {
    fn render(&self) -> String {
        let pad = if self.padded { " padded" } else { "" };
        format!(
            r#"<ui-center max-w="{}"{}>{}</ui-center>"#,
            self.max_w, pad, self.inner.render(),
        )
    }
}

/// One "row" of the storybook: a small label to the left, the story to
/// the right, separated with `<ui-columns ratios="1 3">`. Composed
/// entirely of primitives — the label uses `<ui-badge tone="neutral">`
/// so we don't need a text primitive yet.
struct StoryRow { label: String, comp: Box<dyn Component> }
impl Component for StoryRow {
    fn render(&self) -> String {
        format!(
            r#"<ui-columns ratios="1 3" gap="md" align="center">{}{}</ui-columns>"#,
            badge(self.label.clone()).render(),
            self.comp.render(),
        )
    }
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
    tracing::info!("shutdown signal received");
}
