//! Service layer: **free functions** operating on a tenant `SqlitePool`
//! (and, for auth, the shared `SessionStore`). There is no repository
//! layer; SQL for each domain lives in its matching submodule here.
//!
//! Real business services (academic, people, fees, ...) have been
//! stripped — only `auth`, `system`, and a tiny `demo` module remain
//! as reference wiring for the modules you're about to build.

use thiserror::Error;

use crate::error::RepoError;

pub use context::{Actor, RequestCtx};

pub mod context;

pub mod auth;
pub mod demo;
pub mod system;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("not found")]
    NotFound,

    #[error("validation error: {0}")]
    Validation(String),

    #[error("conflict: {0}")]
    Conflict(String),

    #[error("unauthorized")]
    Unauthorized,

    #[error("forbidden: missing permission {0}")]
    Forbidden(String),

    #[error("password hashing failed: {0}")]
    Hash(String),

    #[error("repository error: {0}")]
    Repo(#[from] RepoError),

    #[error("database error: {0}")]
    Sqlx(#[from] sqlx::Error),
}

impl ServiceError {
    pub fn validation(m: impl Into<String>) -> Self {
        Self::Validation(m.into())
    }
    pub fn conflict(m: impl Into<String>) -> Self {
        Self::Conflict(m.into())
    }
    pub fn forbidden(perm: impl Into<String>) -> Self {
        Self::Forbidden(perm.into())
    }
}

pub type ServiceResult<T> = Result<T, ServiceError>;

/// Permission codes referenced across the UI and services.
///
/// Business modules (attendance, fees, timetable, ...) will each add
/// their own codes here as they land. For now the framework ships
/// with only the codes needed by the auth flow + the demo module.
pub mod perm {
    /// Bound to the seeded `admin` role in
    /// `migrations/20260720120002_seed_rbac_stub.sql`. Delete once
    /// real modules and their permissions replace it.
    pub const DEMO_VIEW: &str = "demo.view";
}
