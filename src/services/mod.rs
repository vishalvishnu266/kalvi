use std::sync::Arc;
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

pub fn from_repos_with_auth(
        repos: Arc<Repositories>,
        auth: auth::AuthService,
    ) -> Self {
        Self {
            academic:      academic::AcademicService::new(repos.clone()),
            auth,
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

    pub const ATTENDANCE_VIEW:      &str = "attendance.view";
    pub const ATTENDANCE_VIEW_OWN:  &str = "attendance.view_own";
    pub const ATTENDANCE_MARK:      &str = "attendance.mark";

    pub const TIMETABLE_VIEW:     &str = "timetable.view";
    pub const TIMETABLE_MANAGE:   &str = "timetable.manage";

    pub const FEES_VIEW:      &str = "fees.view";
    pub const FEES_VIEW_OWN:  &str = "fees.view_own";
    pub const FEES_COLLECT:   &str = "fees.collect";
    pub const FEES_PAY:       &str = "fees.pay";

    pub const EXAMINATIONS_VIEW:         &str = "examinations.view";
    pub const EXAMINATIONS_VIEW_OWN:     &str = "examinations.view_own";
    pub const EXAMINATIONS_MANAGE:       &str = "examinations.manage";
    pub const EXAMINATIONS_ENTER_MARKS:  &str = "examinations.enter_marks";

    pub const PAYROLL_VIEW:      &str = "payroll.view";
    pub const PAYROLL_VIEW_OWN:  &str = "payroll.view_own";
    pub const PAYROLL_RUN:       &str = "payroll.run";

    pub const GUARDIANS_VIEW:    &str = "guardians.view";
    pub const GUARDIANS_MANAGE:  &str = "guardians.manage";

    pub const COMMUNICATION_VIEW:      &str = "communication.view";
    pub const COMMUNICATION_BROADCAST: &str = "communication.broadcast";

    pub const LIBRARY_VIEW:       &str = "library.view";
    pub const LIBRARY_MANAGE:     &str = "library.manage";
    pub const TRANSPORT_VIEW:     &str = "transport.view";
    pub const TRANSPORT_MANAGE:   &str = "transport.manage";
    pub const HOSTEL_VIEW:        &str = "hostel.view";
    pub const HOSTEL_MANAGE:      &str = "hostel.manage";
    pub const INVENTORY_VIEW:     &str = "inventory.view";
    pub const INVENTORY_MANAGE:   &str = "inventory.manage";
    pub const HEALTH_VIEW:        &str = "health.view";
    pub const HEALTH_MANAGE:      &str = "health.manage";
    pub const DISCIPLINE_VIEW:    &str = "discipline.view";
    pub const DISCIPLINE_MANAGE:  &str = "discipline.manage";
    pub const DOCUMENTS_VIEW:     &str = "documents.view";
    pub const DOCUMENTS_MANAGE:   &str = "documents.manage";

    pub const AUDIT_VIEW:         &str = "audit.view";
    pub const SETTINGS_VIEW:      &str = "settings.view";
    pub const SETTINGS_MANAGE:    &str = "settings.manage";
}

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
