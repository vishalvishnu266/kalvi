//! # server — minimal static-file server for the Lit component library.
//!
//! After the non-primitive purge, the server no longer owns page
//! rendering, chrome, launcher, commands, or the agent bridge — those all
//! composed higher-level components that no longer exist in `lit-ui`.
//!
//! What it does now:
//! * Serves the `lit-components/` folder at `/` so you can open the
//!   demos (`lit-components/demos/*.html`) directly in a browser.
//! * That's it.
//!
//! Reintroduce page routes here (or in a new crate) when you start
//! building on top of the primitives + JS layout kit again.

use axum::Router;
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

    // Single route: serve the whole lit-components tree at `/`.
    // The demo pages live at /demos/*.html.
    let app = Router::new()
        .nest_service("/", ServeDir::new(&lit_components_dir))
        .layer(TraceLayer::new_for_http());

    let addr: SocketAddr = BIND_ADDR.parse().expect("valid bind address");
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind TCP listener");

    tracing::info!("🚀  http://{addr}/demos/index.html");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("server error");
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
    tracing::info!("shutdown signal received");
}
