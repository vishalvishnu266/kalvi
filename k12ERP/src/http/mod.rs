//! Optional Axum integration layer.
//!
//! Provides:
//! * [`tenant_scope`] — middleware that resolves the tenant, builds an
//!   [`crate::services::AppServices`] against the tenant's pool, and stores
//!   both in the request extensions.
//! * [`ExtractTenant`] — an Axum extractor for `TenantId`.
//! * [`ExtractServices`] — an Axum extractor for the per-request
//!   [`AppServices`].
//! * [`ServiceHttpError`] — helper that maps [`crate::ServiceError`] to
//!   an HTTP response.
//!
//! You can use these directly or copy the patterns into your own HTTP layer.

pub mod extractors;
pub mod middleware;
pub mod error;

pub use extractors::{ExtractServices, ExtractTenant};
pub use middleware::tenant_scope;
pub use error::ServiceHttpError;
