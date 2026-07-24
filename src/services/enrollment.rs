use std::sync::Arc;

use chrono::NaiveDate;

use crate::repositories::Repositories;
use crate::repositories::class_enrollment::{Enrollment, NewEnrollment};
use crate::services::{RequestCtx, ServiceError, ServiceResult};
use crate::services::perm;

#[derive(Clone)]
pub struct EnrollmentService {
    repos: Arc<Repositories>,
}

impl EnrollmentService {
    pub fn new(repos: Arc<Repositories>) -> Self { Self { repos } }

pub async fn enroll(&self, ctx: &RequestCtx, e: NewEnrollment) -> ServiceResult<Enrollment> {
        ctx.require(perm::ACADEMIC_MANAGE)?;
        let cs = self.repos.class_sections.get(e.class_section_id).await?;
        if let Some(cap) = cs.capacity {
            let filled = self.repos.class_sections.student_count(cs.id).await?;
            if filled >= cap {
                return Err(ServiceError::conflict("class section is full"));
            }
        }
        Ok(self.repos.enrollments.enroll(&e).await?)
    }

pub async fn transfer(
        &self, ctx: &RequestCtx, student_id: i64, to_class_section_id: i64, effective: NaiveDate,
    ) -> ServiceResult<Enrollment> {
        ctx.require(perm::ACADEMIC_MANAGE)?;
        let year = self.repos.academic_years.current().await?;
        let current = self.repos.enrollments
            .current_for_student(student_id, year.id).await?
            .ok_or(ServiceError::NotFound)?;

        if current.class_section_id == to_class_section_id {
            return Err(ServiceError::validation("already in target class"));
        }

sqlx::query(
            "UPDATE enrollment SET left_on = NULL, result = NULL, class_section_id = ? WHERE id = ?",
        )
        .bind(to_class_section_id).bind(current.id)
        .execute(&self.repos.pool).await?;

        self.repos.audit_log.record(&crate::repositories::audit::NewAudit {
            user_id: None,
            entity: "enrollment".into(),
            entity_id: current.id,
            action: "update".into(),
            diff_json: Some(serde_json::json!({
                "transfer": {
                    "from_class_section_id": current.class_section_id,
                    "to_class_section_id": to_class_section_id,
                    "effective": effective,
                }
            })),
            ip_address: None,
            user_agent: None,
        }).await?;

        Ok(self.repos.enrollments.get(current.id).await?)
    }

    pub async fn close_current(
        &self, ctx: &RequestCtx, student_id: i64, on: NaiveDate, result: &str,
    ) -> ServiceResult<()> {
        ctx.require(perm::ACADEMIC_MANAGE)?;
        let year = self.repos.academic_years.current().await?;
        let current = self.repos.enrollments
            .current_for_student(student_id, year.id).await?
            .ok_or(ServiceError::NotFound)?;
        self.repos.enrollments.close(current.id, on, Some(result)).await?;
        Ok(())
    }

    pub async fn roster(&self, ctx: &RequestCtx, class_section_id: i64) -> ServiceResult<Vec<Enrollment>> {
        ctx.require(perm::ACADEMIC_VIEW)?;
        Ok(self.repos.enrollments.roster(class_section_id).await?)
    }
}
