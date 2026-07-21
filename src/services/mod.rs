//! Service layer.
//!
//! Services sit on top of the repository layer and encapsulate **business
//! workflows** — anything that touches more than one repo, needs domain
//! validation beyond FK/CHECK constraints, or turns raw persistence into a
//! user-facing operation (e.g. "admit student", "receive payment", "generate
//! payslip", "receive purchase order").
//!
//! Repositories are still exposed via [`AppServices::repos`] so callers can
//! do simple reads directly.
//!
//! ## Request context
//!
//! [`AppServices`] is **tenant-scoped and cached** by the tenant registry;
//! per-request state (current user, roles, request id, trace id, actor kind)
//! lives in a separate [`RequestCtx`] value that is built per request by
//! HTTP middleware (or manually by background jobs / CLI / tests) and passed
//! as a parameter to any service method that needs to know *who* is doing
//! *what* and *from where*.
//!
//! This keeps `AppServices` immutable and shareable across concurrent
//! requests while still giving service methods a first-class way to see the
//! caller. Adoption is incremental — start by threading `&RequestCtx` into
//! new methods and into ones that need auditing, authorization or tracing.

use std::sync::Arc;

use sqlx::SqlitePool;
use thiserror::Error;

use crate::error::RepoError;
use crate::repositories::Repositories;

pub use context::{Actor, RequestCtx};

pub mod context;

pub mod academic;
pub mod attendance;
pub mod auth;
pub mod communication;
pub mod discipline;
pub mod documents;
pub mod enrollment;
pub mod examinations;
pub mod fees;
pub mod health;
pub mod hostel;
pub mod inventory;
pub mod library;
pub mod payroll;
pub mod people;
pub mod timetable;
pub mod transport;

/// Service-layer error. Wraps [`RepoError`] and adds
/// service-specific failure modes (auth failed, business rule violated, etc.).
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

/// Aggregator that holds every service. Cheap to clone (all state is behind
/// `Arc`), so wire this into your HTTP/CLI layer as shared state.
#[derive(Clone)]
pub struct AppServices {
    pub repos: Arc<Repositories>,

    pub academic:      academic::AcademicService,
    pub auth:          auth::AuthService,
    pub people:        people::PeopleService,
    pub enrollment:    enrollment::EnrollmentService,
    pub attendance:    attendance::AttendanceService,
    pub timetable:     timetable::TimetableService,
    pub examinations:  examinations::ExaminationService,
    pub fees:          fees::FeeService,
    pub payroll:       payroll::PayrollService,
    pub library:       library::LibraryService,
    pub transport:     transport::TransportService,
    pub hostel:        hostel::HostelService,
    pub inventory:     inventory::InventoryService,
    pub communication: communication::CommunicationService,
    pub health:        health::HealthService,
    pub discipline:    discipline::DisciplineService,
    pub documents:     documents::DocumentService,
}

impl AppServices {
    pub fn new(pool: SqlitePool) -> Self {
        let repos = Arc::new(Repositories::new(pool));
        Self {
            academic:      academic::AcademicService::new(repos.clone()),
            auth:          auth::AuthService::new(repos.clone()),
            people:        people::PeopleService::new(repos.clone()),
            enrollment:    enrollment::EnrollmentService::new(repos.clone()),
            attendance:    attendance::AttendanceService::new(repos.clone()),
            timetable:     timetable::TimetableService::new(repos.clone()),
            examinations:  examinations::ExaminationService::new(repos.clone()),
            fees:          fees::FeeService::new(repos.clone()),
            payroll:       payroll::PayrollService::new(repos.clone()),
            library:       library::LibraryService::new(repos.clone()),
            transport:     transport::TransportService::new(repos.clone()),
            hostel:        hostel::HostelService::new(repos.clone()),
            inventory:     inventory::InventoryService::new(repos.clone()),
            communication: communication::CommunicationService::new(repos.clone()),
            health:        health::HealthService::new(repos.clone()),
            discipline:    discipline::DisciplineService::new(repos.clone()),
            documents:     documents::DocumentService::new(repos.clone()),
            repos,
        }
    }
}

/// Convention: services that need chart-of-accounts codes look them up by
/// these constants; they match the seed migration `017_seed_reference_data.sql`.
pub mod ledger_codes {
    pub const CASH: &str            = "1000";
    pub const BANK: &str            = "1010";
    pub const RECEIVABLE: &str      = "1200";
    pub const PAYABLE: &str         = "2000";
    pub const TUITION_INCOME: &str  = "4000";
    pub const TRANSPORT_INCOME: &str = "4100";
    pub const HOSTEL_INCOME: &str   = "4200";
    pub const SALARY_EXPENSE: &str  = "5000";
    pub const SUPPLIES_EXPENSE: &str = "5200";
}
