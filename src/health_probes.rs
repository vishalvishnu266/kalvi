//! Cheap, cloneable readiness flag used by:
//!
//! * `GET /api/live`  — always 200 while the process runs. Kubernetes uses
//!   this to decide whether to **restart** the pod.
//! * `GET /api/ready` — 200 while accepting traffic, **503** during shutdown.
//!   Kubernetes uses this to decide whether to **route traffic** to the pod.
//!
//! Flip readiness to `false` at the start of shutdown; the load balancer will
//! stop routing new requests within one probe interval, and in-flight
//! requests continue to drain via [`crate::shutdown`]. The actual HTTP
//! handlers live in [`crate::http::routes`].

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

/// Cheap, cloneable readiness flag shared with handlers.
///
/// Wrap `AtomicBool` in `Arc` so both the router state and the shutdown code
/// mutate the same value.
#[derive(Clone, Default)]
pub struct Readiness(pub Arc<AtomicBool>);

impl Readiness {
    /// Start out ready (the default is `false`, so bump it here).
    pub fn new_ready() -> Self {
        tracing::debug!("Readiness::new_ready: initializing as ready");
        let r = Self::default();
        r.set_ready(true);
        r
    }

    pub fn is_ready(&self) -> bool { self.0.load(Ordering::SeqCst) }
    pub fn set_ready(&self, v: bool) {
        tracing::debug!("Readiness::set_ready: v={}", v);
        self.0.store(v, Ordering::SeqCst);
    }
}
