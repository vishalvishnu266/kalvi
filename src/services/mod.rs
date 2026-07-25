//! Service layer: **free functions** operating on a tenant `SqlitePool`
//! (and, for auth, the shared `SessionStore`). There is no repository
//! layer; SQL for each domain lives in its matching submodule here.

use thiserror::Error;

use crate::error::RepoError;

pub use context::{Actor, RequestCtx};

pub mod context;

pub mod academic;
pub mod auth;
pub mod guardians;
pub mod people;
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
    pub fn validation(m: impl Into<String>) -> Self { Self::Validation(m.into()) }
    pub fn conflict(m: impl Into<String>) -> Self { Self::Conflict(m.into()) }
    pub fn forbidden(perm: impl Into<String>) -> Self { Self::Forbidden(perm.into()) }
}

pub type ServiceResult<T> = Result<T, ServiceError>;

pub mod perm {
    pub const STUDENTS_VIEW:      &str = "students.view";
    pub const STUDENTS_VIEW_OWN:  &str = "students.view_own";
    pub const STUDENTS_EDIT:      &str = "students.edit";
    pub const STUDENTS_ADMIT:     &str = "students.admit";

    pub const STAFF_VIEW:         &str = "staff.view";
    pub const STAFF_EDIT:         &str = "staff.edit";
    pub const STAFF_HIRE:         &str = "staff.hire";

    pub const ACADEMIC_VIEW:      &str = "academic.view";
    pub const ACADEMIC_MANAGE:    &str = "academic.manage";

    pub const GUARDIANS_VIEW:    &str = "guardians.view";
    pub const GUARDIANS_MANAGE:  &str = "guardians.manage";

    pub const AUDIT_VIEW:         &str = "audit.view";
    pub const SETTINGS_VIEW:      &str = "settings.view";
    pub const SETTINGS_MANAGE:    &str = "settings.manage";
}
