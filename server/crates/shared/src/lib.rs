//! Shared infrastructure — the "spring-core" of this workspace.
//!
//! Layered as:
//!  * `config`  — application-wide state (DB manager, `AppState`).
//!  * `web`     — HTTP concerns: error type, tenant middleware, base layout
//!                and reusable Maud view components.

pub mod config;
pub mod web;
pub mod middleware;

// -------- Convenience re-exports (public API of this crate) --------

pub use config::app_state::AppState;
pub use config::db::TenantDatabaseManager;
pub use web::error::AppError;
pub use web::layout::base;
pub use web::html::{Html, IntoHtml, escape, e};
pub use web::styles::*;

pub use middleware::tenant::{tenant_middleware, TenantContext};
pub use middleware::session::session_middleware;
