//! Thin HTTP layer.
//!
//! Routing is split into:
//! * [`routes`] for top-level assembly (`/web`, `/portal`, admin UI, probes)
//! * [`api_routes`] for `/api/*` URL wiring
//!
//! No tenant middleware; a single [`TenantScope`] extractor validates the
//! `{tenant}` path segment and resolves the per-tenant `AppServices`.

pub mod api_routes;
pub mod routes;

pub use routes::{build_router, AppState, ServiceHttpError, TenantScope};
