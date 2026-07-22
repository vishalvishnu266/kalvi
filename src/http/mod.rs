//! Thin HTTP layer.
//!
//! Everything about routing lives in [`routes`] — one file, one place. No
//! tenant middleware; a single [`TenantScope`] extractor validates the
//! `{tenant}` path segment and resolves the per-tenant `AppServices`.

pub mod routes;

pub use routes::{build_router, AppState, ServiceHttpError, TenantScope};
