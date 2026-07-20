//! School ERP core library.
//!
//! This crate exposes:
//! * [`db`]     — pool construction + migrations
//! * [`error`]  — a single [`error::RepoError`] used by all repositories
//! * [`repositories`] — one repository per domain module
//!
//! Typical usage:
//! ```no_run
//! # async fn demo() -> Result<(), school_erp::error::RepoError> {
//! let pool = school_erp::db::connect("sqlite://data/erp.db?mode=rwc").await?;
//! school_erp::db::migrate(&pool).await?;
//! let repos = school_erp::repositories::Repositories::new(pool);
//! let _ = repos.students.list(50, 0).await?;
//! # Ok(()) }
//! ```

pub mod api;
pub mod db;
pub mod error;
pub mod health_probes;
pub mod http;
pub mod repositories;
pub mod services;
pub mod shutdown;
pub mod system;
pub mod tenancy;
pub mod tracing_layer;

pub use api::{build_router, AppState};
pub use error::RepoError;
pub use services::{AppServices, ServiceError, ServiceResult};
pub use system::{connect_system, migrate_system, DbTenantGuard, SystemRegistry};
pub use tenancy::{TenantId, TenantRegistry, TenantRegistryConfig};
