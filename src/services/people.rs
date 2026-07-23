//! People workflows: admit a student (optionally in one call with guardian +
//! current-year enrollment), hire staff, link a login user.
//!
//! ## Row-level scoping ([`Scope`])
//!
//! RBAC decides *which screens* a caller can visit. `Scope` decides *which
//! rows* those screens are allowed to return. Handlers derive a `Scope` from
//! the caller's [`crate::middleware::auth::SessionUser`] (via
//! [`Scope::from_session`]) and hand it to service methods like
//! [`PeopleService::list_students_for`]. This keeps the router thin and lets
//! us evolve the scoping rules in one place per resource.

use std::sync::Arc;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::middleware::auth::SessionUser;
use crate::repositories::Repositories;
use crate::repositories::class_enrollment::NewEnrollment;
use crate::repositories::guardians::{NewGuardian, StudentGuardianLink};
use crate::repositories::staff::{NewStaff, Staff};
use crate::repositories::students::{NewStudent, Student};
use crate::services::{ServiceError, ServiceResult};

/// Row-level visibility for people-related list/get calls.
///
/// The variants are intentionally coarse — one per real-world "who is asking"
/// pattern — and each one maps to a well-defined DB query. Add new variants
/// (e.g. `Teacher { class_section_ids }`) as we build out more portals.
#[derive(Debug, Clone)]
pub enum Scope {
    /// See every row (admin / principal / staff with full read).
    Global,
    /// See only students linked to the given guardian *user_id* (parent portal).
    GuardianOfUser(i64),
    /// See only the student row linked to the given login *user_id* (student
    /// self-service portal).
    SelfStudent(i64),
}

impl Scope {
    /// Derive the appropriate scope from a session.
    ///
    /// Precedence (widest wins): if a caller has the plain `students.view`
    /// permission they're `Global`. Otherwise, if they're a `guardian`, they
    /// get `GuardianOfUser`. Otherwise, if they're a `student`, they get
    /// `SelfStudent`. Anything else falls through to `GuardianOfUser` with
    /// their user_id (which will resolve to an empty list) — safer than
    /// defaulting to `Global`.
    pub fn from_session(session: &SessionUser) -> Self {
        use crate::services::perm::STUDENTS_VIEW;
        if session.has(STUDENTS_VIEW) {
            Scope::Global
        } else if session.is_role("guardian") {
            Scope::GuardianOfUser(session.user_id)
        } else if session.is_role("student") {
            Scope::SelfStudent(session.user_id)
        } else {
            Scope::GuardianOfUser(session.user_id)
        }
    }
}

/// One-shot student admission input.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Admission {
    pub student: NewStudent,
    pub guardian: Option<(NewGuardian, String /* relationship */, bool /* primary */)>,
    /// If set, immediately enroll the student in this class section for the
    /// current academic year.
    pub enroll_into_class_section: Option<i64>,
    pub roll_no: Option<i64>,
    pub enrolled_on: Option<NaiveDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdmissionResult {
    pub student: Student,
    pub guardian_id: Option<i64>,
    pub enrollment_id: Option<i64>,
}

#[derive(Clone)]
pub struct PeopleService {
    repos: Arc<Repositories>,
}

impl PeopleService {
    pub fn new(repos: Arc<Repositories>) -> Self { Self { repos } }

    pub async fn admit(&self, a: Admission) -> ServiceResult<AdmissionResult> {
        // Reject duplicate admission_no early.
        if self.repos.students.find_by_admission_no(&a.student.admission_no).await?.is_some() {
            return Err(ServiceError::conflict("admission_no already exists"));
        }

        let student = self.repos.students.create(&a.student).await?;

        let mut guardian_id = None;
        if let Some((g, relationship, is_primary)) = a.guardian {
            let guardian = self.repos.guardians.create(&g).await?;
            self.repos.guardians.link(&StudentGuardianLink {
                student_id: student.id,
                guardian_id: guardian.id,
                relationship,
                is_primary,
                is_emergency: is_primary,
                can_pickup: true,
            }).await?;
            guardian_id = Some(guardian.id);
        }

        let mut enrollment_id = None;
        if let Some(cs_id) = a.enroll_into_class_section {
            let year = self.repos.academic_years.current().await?;
            let e = self.repos.enrollments.enroll(&NewEnrollment {
                student_id: student.id,
                class_section_id: cs_id,
                academic_year_id: year.id,
                roll_no: a.roll_no,
                enrolled_on: a.enrolled_on.unwrap_or_else(|| chrono::Local::now().date_naive()),
            }).await?;
            enrollment_id = Some(e.id);
        }

        Ok(AdmissionResult { student, guardian_id, enrollment_id })
    }

    pub async fn withdraw(&self, student_id: i64) -> ServiceResult<()> {
        self.repos.students.set_status(student_id, "withdrawn").await?;
        Ok(())
    }

    pub async fn graduate(&self, student_id: i64) -> ServiceResult<()> {
        self.repos.students.set_status(student_id, "graduated").await?;
        Ok(())
    }

    pub async fn hire_staff(&self, s: NewStaff) -> ServiceResult<Staff> {
        if self.repos.staff.find_by_employee_no(&s.employee_no).await?.is_some() {
            return Err(ServiceError::conflict("employee_no already exists"));
        }
        Ok(self.repos.staff.create(&s).await?)
    }

    pub async fn terminate_staff(&self, staff_id: i64, on: NaiveDate) -> ServiceResult<()> {
        self.repos.staff.update(staff_id, &crate::repositories::staff::UpdateStaff {
            status: Some("terminated".into()),
            date_of_leaving: Some(Some(on)),
            ..Default::default()
        }).await?;
        Ok(())
    }

    // -----------------------------------------------------------------
    // Row-scoped reads
    // -----------------------------------------------------------------

    /// Return the students the caller is allowed to see, according to
    /// `scope`. `limit` is applied only to the `Global` case — the scoped
    /// variants are naturally bounded by the caller's own relationships.
    pub async fn list_students_for(
        &self,
        scope: Scope,
        limit: i64,
    ) -> ServiceResult<Vec<Student>> {
        match scope {
            Scope::Global => Ok(self.repos.students.list(limit, 0).await?),
            Scope::GuardianOfUser(uid) => {
                let ids = self.repos.guardians.students_of_user(uid).await?;
                Ok(self.repos.students.list_by_ids(&ids).await?)
            }
            Scope::SelfStudent(uid) => {
                match self.repos.students.find_by_user_id(uid).await? {
                    Some(s) => Ok(vec![s]),
                    None    => Ok(Vec::new()),
                }
            }
        }
    }

    /// True if `scope` is allowed to open the profile of `student_id`.
    /// Detail-page handlers call this before rendering so a parent can't
    /// deep-link to another family's child.
    pub async fn can_view_student(
        &self,
        scope: &Scope,
        student_id: i64,
    ) -> ServiceResult<bool> {
        Ok(match scope {
            Scope::Global => true,
            Scope::GuardianOfUser(uid) => {
                self.repos.guardians.is_guardian_of(*uid, student_id).await?
            }
            Scope::SelfStudent(uid) => {
                self.repos.students.find_by_user_id(*uid).await?
                    .map(|s| s.id == student_id)
                    .unwrap_or(false)
            }
        })
    }
}
