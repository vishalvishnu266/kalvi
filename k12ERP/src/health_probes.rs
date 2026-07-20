//! Kubernetes-style health probes.
//!
//! * `GET /api/live`  — always 200 while the process runs. Kubernetes uses
//!   this to decide whether to **restart** the pod.
//! * `GET /api/ready` — 200 while accepting traffic, **503** during shutdown.
//!   Kubernetes uses this to decide whether to **route traffic** to the pod.
//!
//! Flip readiness to `false` at the start of shutdown; the load balancer will
//! stop routing new requests within one probe interval, and in-flight
//! requests continue to drain via [`crate::shutdown`].

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};

/// Cheap, cloneable readiness flag shared with handlers.
///
/// Wrap `AtomicBool` in `Arc` so both the router state and the shutdown code
/// mutate the same value.
#[derive(Clone, Default)]
pub struct Readiness(pub Arc<AtomicBool>);

impl Readiness {
    /// Start out ready (the default is `false`, so bump it here).
    pub fn new_ready() -> Self {
        let r = Self::default();
        r.set_ready(true);
        r
    }

    pub fn is_ready(&self) -> bool { self.0.load(Ordering::SeqCst) }
    pub fn set_ready(&self, v: bool) { self.0.store(v, Ordering::SeqCst); }
}

/// Router exposing `/api/live` and `/api/ready`.
pub fn router(readiness: Readiness) -> Router {
    Router::new()
        .route("/api/live",  get(live))
        .route("/api/ready", get(ready))
        .with_state(readiness)
}

async fn live() -> impl IntoResponse {
    (StatusCode::OK, Json(serde_json::json!({ "status": "alive" })))
}

async fn ready(State(r): State<Readiness>) -> impl IntoResponse {
    if r.is_ready() {
        (StatusCode::OK, Json(serde_json::json!({ "status": "ready" })))
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, Json(serde_json::json!({ "status": "draining" })))
    }
}
