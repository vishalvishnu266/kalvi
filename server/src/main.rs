//! # server — the demo entry point for the framework.
//!
//! Boots Axum, registers page routes, and serves the `lit-components/`
//! folder as static assets. All page rendering is delegated to
//! [`crate::pages`]; layout chrome (topbar + sidebar + shell) lives in
//! [`crate::shell`].
//!
//! Every route is *content-negotiated*: if the client sends
//! `Accept: text/vnd.ui-fragments+html` (which the JS runtime does on
//! intercepted navigation) the handler returns fragments only;
//! otherwise it returns a full HTML document.

mod agent_route;
mod pages;
mod shell;

use axum::{routing::{get, post}, Router};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tower_http::{services::ServeDir, trace::TraceLayer};

use agent::StubAgent;
use crate::agent_route::AgentState;

/// Address the server binds to. Kept as a constant so it's easy to spot.
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

    // The copilot's brain. Rule-based today; swap in an LlmAgent later
    // without touching the route or the client.
    let agent_state = AgentState { agent: Arc::new(StubAgent::new()) };

    let app = Router::new()
        .route("/",          get(pages::landing::handler))
        .route("/dashboard", get(pages::dashboard::handler))
        .route("/admin",     get(pages::admin::handler))
        .route("/users",     get(pages::users::handler))
        .route("/agent",     post(agent_route::handler).with_state(agent_state))
        .nest_service("/lit-components", ServeDir::new(&lit_components_dir))
        .layer(TraceLayer::new_for_http());

    let addr: SocketAddr = BIND_ADDR.parse().expect("valid bind address");
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind TCP listener");

    tracing::info!("🚀  http://{addr}/");
    tracing::info!("     • island nav: click sidebar links");
    tracing::info!("     • fragments : curl -H 'Accept: text/vnd.ui-fragments+html' http://{addr}/dashboard");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("server error");
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
    tracing::info!("shutdown signal received");
}
