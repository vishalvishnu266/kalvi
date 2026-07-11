//! Shared infrastructure — the "spring-core" of this workspace.
//!
//! Layered as:
//!  * `config`  — application-wide state (DB manager, `AppState`).
//!  * `web`     — HTTP concerns: error type, tenant middleware, base layout
//!                and reusable Maud view components.

pub mod config;
pub mod web;

// -------- Convenience re-exports (public API of this crate) --------

pub use config::app_state::AppState;
pub use config::db::TenantDatabaseManager;
pub use web::error::AppError;
pub use web::middleware::{tenant_middleware, TenantContext};
pub use web::layout::base;
