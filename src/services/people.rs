use std::sync::Arc;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::repositories::Repositories;
use crate::repositories::class_enrollment::NewEnrollment;
use crate::repositories::guardians::{NewGuardian, StudentGuardianLink};
use crate::repositories::staff::{NewStaff, Staff};
use crate::repositories::students::{NewStudent, Student};
use crate::services::{RequestCtx, ServiceError, ServiceResult};
use crate::services::perm;

#[derive(Debug, Clone)]
pub enum Scope {

    Global,

    GuardianOfUser(i64),

SelfStudent(i64),
}

impl Scope {

    pub fn from_ctx(ctx: &RequestCtx) -> Self {
        if ctx.has_permission(perm::STUDENTS_VIEW) {
            Scope::Global
        } else if let Some(uid) = ctx.user_id() {
            // Guardian or self-student: determined by role assignment at login;
            // we fall back to guardian-scoped view unless explicitly a student.
            Scope::GuardianOfUser(uid)
        } else {
            Scope::GuardianOfUser(0)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Admission {
    pub student: NewStudent,
    pub guardian: Option<(NewGuardian, String , bool )>,

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

    pub async fn admit(&self, ctx: &RequestCtx, a: Admission) -> ServiceResult<AdmissionResult> {
        ctx.require(perm::STUDENTS_ADMIT)?;

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

    pub async fn withdraw(&self, ctx: &RequestCtx, student_id: i64) -> ServiceResult<()> {
        ctx.require(perm::STUDENTS_EDIT)?;
        self.repos.students.set_status(student_id, "withdrawn").await?;
        Ok(())
    }

    pub async fn graduate(&self, ctx: &RequestCtx, student_id: i64) -> ServiceResult<()> {
        ctx.require(perm::STUDENTS_EDIT)?;
        self.repos.students.set_status(student_id, "graduated").await?;
        Ok(())
    }

    pub async fn hire_staff(&self, ctx: &RequestCtx, s: NewStaff) -> ServiceResult<Staff> {
        ctx.require(perm::STAFF_HIRE)?;
        if self.repos.staff.find_by_employee_no(&s.employee_no).await?.is_some() {
            return Err(ServiceError::conflict("employee_no already exists"));
        }
        Ok(self.repos.staff.create(&s).await?)
    }

    pub async fn terminate_staff(&self, ctx: &RequestCtx, staff_id: i64, on: NaiveDate) -> ServiceResult<()> {
        ctx.require(perm::STAFF_EDIT)?;
        self.repos.staff.update(staff_id, &crate::repositories::staff::UpdateStaff {
            status: Some("terminated".into()),
            date_of_leaving: Some(Some(on)),
            ..Default::default()
        }).await?;
        Ok(())
    }

pub async fn list_students_for(
        &self,
        ctx: &RequestCtx,
        limit: i64,
        offset: i64,
    ) -> ServiceResult<Vec<Student>> {
        ctx.require_any(&[perm::STUDENTS_VIEW, perm::STUDENTS_VIEW_OWN])?;
        let scope = Scope::from_ctx(ctx);
        match scope {
            Scope::Global => Ok(self.repos.students.list(limit, offset).await?),
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

pub async fn can_view_student(
        &self,
        ctx: &RequestCtx,
        student_id: i64,
    ) -> ServiceResult<bool> {
        let scope = Scope::from_ctx(ctx);
        Ok(match scope {
            Scope::Global => true,
            Scope::GuardianOfUser(uid) => {
                self.repos.guardians.is_guardian_of(uid, student_id).await?
            }
            Scope::SelfStudent(uid) => {
                self.repos.students.find_by_user_id(uid).await?
                    .map(|s| s.id == student_id)
                    .unwrap_or(false)
            }
        })
    }
}
