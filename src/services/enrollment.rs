//! Student ↔ class movements: enroll, transfer, close.

use std::sync::Arc;

use chrono::NaiveDate;

use crate::repositories::Repositories;
use crate::repositories::class_enrollment::{Enrollment, NewEnrollment};
use crate::services::{ServiceError, ServiceResult};

#[derive(Clone)]
pub struct EnrollmentService {
    repos: Arc<Repositories>,
}

impl EnrollmentService {
    pub fn new(repos: Arc<Repositories>) -> Self { Self { repos } }

    /// Enroll a student into a class section, honouring capacity.
    pub async fn enroll(&self, e: NewEnrollment) -> ServiceResult<Enrollment> {
        // Capacity guard (unique(student, year) is enforced by DB, but capacity is app-level).
        let cs = self.repos.class_sections.get(e.class_section_id).await?;
        if let Some(cap) = cs.capacity {
            let filled = self.repos.class_sections.student_count(cs.id).await?;
            if filled >= cap {
                return Err(ServiceError::conflict("class section is full"));
            }
        }
        Ok(self.repos.enrollments.enroll(&e).await?)
    }

    /// Move a student mid-year from one class section to another
    /// (closes the current enrollment, opens a new one).
    pub async fn transfer(
        &self, student_id: i64, to_class_section_id: i64, effective: NaiveDate,
    ) -> ServiceResult<Enrollment> {
        let year = self.repos.academic_years.current().await?;
        let current = self.repos.enrollments
            .current_for_student(student_id, year.id).await?
            .ok_or(ServiceError::NotFound)?;

        if current.class_section_id == to_class_section_id {
            return Err(ServiceError::validation("already in target class"));
        }

        // The DB enforces UNIQUE(student, year), so we cannot open a *second*
        // row for the same year. Instead we log the transfer via the audit
        // trail (below) and switch the class on the same enrollment row,
        // clearing the close we just did.
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
        &self, student_id: i64, on: NaiveDate, result: &str,
    ) -> ServiceResult<()> {
        let year = self.repos.academic_years.current().await?;
        let current = self.repos.enrollments
            .current_for_student(student_id, year.id).await?
            .ok_or(ServiceError::NotFound)?;
        self.repos.enrollments.close(current.id, on, Some(result)).await?;
        Ok(())
    }

    pub async fn roster(&self, class_section_id: i64) -> ServiceResult<Vec<Enrollment>> {
        Ok(self.repos.enrollments.roster(class_section_id).await?)
    }
}
