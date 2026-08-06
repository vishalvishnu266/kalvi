//! # server — a Rust-DSL-driven Storybook for the Lit primitives kit.
//!
//! Two routes:
//! * `GET /`                — the storybook page (rendered by `lit_ui::page()`).
//! * `GET /lit-components/*` — static assets (stylesheets + component modules).
//!
//! The storybook page composes every primitive through the `lit_ui` DSL.
//! There is no HTML string literal or custom wrapper struct in this file —
//! the ENTIRE page is a call to `page().add(...)` with primitives.

mod stories;

use axum::{
    body::Body,
    http::{header, HeaderValue, Request, Response},
    middleware::{self, Next},
    response::Html,
    routing::get,
    Router,
};
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

    let lit_components_dir: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("server crate must have a workspace parent")
        .join("lit-components");

    // Serve every non-`/` request from the workspace root, exposing the
    // whole tree at its real paths. That means the browser can request
    // `/lit-components/components/index.js` and get it directly, with
    // ServeDir handling MIME + range headers correctly. We use
    // `fallback_service` instead of `nest_service` because the latter
    // has surprising path-stripping behaviour that varies across
    // tower-http versions and can turn a valid request into a 404.
    let workspace_root: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("server crate must have a workspace parent")
        .to_path_buf();
    let _ = lit_components_dir; // keep the variable for future reference/logging

    let app = Router::new()
        .route("/", get(storybook))
        .fallback_service(ServeDir::new(&workspace_root))
        // Force the correct Content-Type for JS / CSS files. `ServeDir`
        // usually gets this right via `mime_guess`, but on some Linux
        // configurations (or when `mime_guess`'s DB is stale) `.js` can
        // come back as `text/plain`, which browsers refuse to execute
        // as an ES module — silently, with no console error. This
        // middleware normalizes the header after `ServeDir` has run.
        .layer(middleware::from_fn(force_correct_mime))
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

/// GET / — the storybook page. Pure `lit_ui` composition.
async fn storybook() -> Html<String> {
    // Body is one big vertical stack, centered in a readable column.
    let mut body_stack = stack().gap(Gap::Xl);

    // Page-level title + the theme toggle in a cluster so they share the row.
    body_stack = body_stack.add(
        cluster_row()
            .justify(Justify::Between)
            .add(heading("lit-ui storybook").h1().tone(HeadingTone::Brand))
            .add(theme_toggle())
    );

    // One section per grouping of stories.
    for section in stories::all_sections() {
        let mut section_stack = stack().gap(Gap::Md);
        section_stack = section_stack.add(heading(section.title).h2());

        for (name, comp) in section.stories {
            // Each story row: label on the left, rendered story on the right.
            let row = columns()
                .ratios("1 3")
                .gap(Gap::Md)
                .align(Align::Center)
                .add(heading(name).h4().tone(HeadingTone::Muted))
                .add_boxed(comp);
            section_stack = section_stack.add(row);
        }

        body_stack = body_stack.add(section_stack);
    }

    let centered = center().max_w("1024px").padded().add(body_stack);
    let html = page()
        .title("lit-ui storybook")
        .body_class("has-storybook")
        .add(centered)
        .render();
    Html(html)
}

/// Middleware: after the inner service (ServeDir) produces a response,
/// overwrite Content-Type based on the URL extension so browsers accept
/// `.js` as an ES module and `.css` as a stylesheet regardless of how
/// `mime_guess` classified the file.
async fn force_correct_mime(req: Request<Body>, next: Next) -> Response<Body> {
    let path = req.uri().path().to_string();
    let mut resp = next.run(req).await;
    let override_ct = if path.ends_with(".js") || path.ends_with(".mjs") {
        Some("application/javascript; charset=utf-8")
    } else if path.ends_with(".css") {
        Some("text/css; charset=utf-8")
    } else if path.ends_with(".html") {
        Some("text/html; charset=utf-8")
    } else if path.ends_with(".json") {
        Some("application/json; charset=utf-8")
    } else {
        None
    };
    if let Some(ct) = override_ct {
        resp.headers_mut()
            .insert(header::CONTENT_TYPE, HeaderValue::from_static(ct));
    }
    resp
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
    tracing::info!("shutdown signal received");
}
