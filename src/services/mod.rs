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

/// Permission codes referenced across the UI (nav, dashboard tiles,
/// stub pages) and services. Only a subset is enforced today — the
/// full catalogue is kept here so navigation, RBAC seeding, and
/// upcoming domain implementations line up.
pub mod perm {
    // ── People ──────────────────────────────────────────────────────
    pub const STUDENTS_VIEW: &str = "students.view";
    pub const STUDENTS_VIEW_OWN: &str = "students.view_own";
    pub const STUDENTS_EDIT: &str = "students.edit";
    pub const STUDENTS_ADMIT: &str = "students.admit";

    pub const STAFF_VIEW: &str = "staff.view";
    pub const STAFF_EDIT: &str = "staff.edit";
    pub const STAFF_HIRE: &str = "staff.hire";

    pub const GUARDIANS_VIEW: &str = "guardians.view";
    pub const GUARDIANS_MANAGE: &str = "guardians.manage";

    // ── Academic ────────────────────────────────────────────────────
    pub const ACADEMIC_VIEW: &str = "academic.view";
    pub const ACADEMIC_MANAGE: &str = "academic.manage";

    // ── Attendance ──────────────────────────────────────────────────
    pub const ATTENDANCE_VIEW: &str = "attendance.view";
    pub const ATTENDANCE_VIEW_OWN: &str = "attendance.view_own";
    pub const ATTENDANCE_MARK: &str = "attendance.mark";

    // ── Timetable ───────────────────────────────────────────────────
    pub const TIMETABLE_VIEW: &str = "timetable.view";
    pub const TIMETABLE_MANAGE: &str = "timetable.manage";

    // ── Fees ────────────────────────────────────────────────────────
    pub const FEES_VIEW: &str = "fees.view";
    pub const FEES_VIEW_OWN: &str = "fees.view_own";
    pub const FEES_COLLECT: &str = "fees.collect";
    pub const FEES_PAY: &str = "fees.pay";

    // ── Examinations ────────────────────────────────────────────────
    pub const EXAMINATIONS_VIEW: &str = "examinations.view";
    pub const EXAMINATIONS_VIEW_OWN: &str = "examinations.view_own";
    pub const EXAMINATIONS_MANAGE: &str = "examinations.manage";
    pub const EXAMINATIONS_ENTER_MARKS: &str = "examinations.enter_marks";

    // ── Payroll ─────────────────────────────────────────────────────
    pub const PAYROLL_VIEW: &str = "payroll.view";
    pub const PAYROLL_VIEW_OWN: &str = "payroll.view_own";
    pub const PAYROLL_RUN: &str = "payroll.run";

    // ── Communication ───────────────────────────────────────────────
    pub const COMMUNICATION_VIEW: &str = "communication.view";
    pub const COMMUNICATION_BROADCAST: &str = "communication.broadcast";

    // ── Ancillary modules ───────────────────────────────────────────
    pub const LIBRARY_VIEW: &str = "library.view";
    pub const LIBRARY_MANAGE: &str = "library.manage";
    pub const TRANSPORT_VIEW: &str = "transport.view";
    pub const TRANSPORT_MANAGE: &str = "transport.manage";
    pub const HOSTEL_VIEW: &str = "hostel.view";
    pub const HOSTEL_MANAGE: &str = "hostel.manage";
    pub const INVENTORY_VIEW: &str = "inventory.view";
    pub const INVENTORY_MANAGE: &str = "inventory.manage";
    pub const HEALTH_VIEW: &str = "health.view";
    pub const HEALTH_MANAGE: &str = "health.manage";
    pub const DISCIPLINE_VIEW: &str = "discipline.view";
    pub const DISCIPLINE_MANAGE: &str = "discipline.manage";
    pub const DOCUMENTS_VIEW: &str = "documents.view";
    pub const DOCUMENTS_MANAGE: &str = "documents.manage";

    // ── System ──────────────────────────────────────────────────────
    pub const AUDIT_VIEW: &str = "audit.view";
    pub const SETTINGS_VIEW: &str = "settings.view";
    pub const SETTINGS_MANAGE: &str = "settings.manage";
}
