//! Attendance workflows: mark a whole class in one call, compute % for a
//! student or class.

use std::sync::Arc;

use chrono::NaiveDate;

use crate::repositories::Repositories;
use crate::repositories::attendance::{MarkStudent, StudentAttendance};
use crate::services::{ServiceError, ServiceResult};

#[derive(Debug, Clone)]
pub struct BulkMark {
    pub student_id: i64,
    pub status: String,
    pub remarks: Option<String>,
}

#[derive(Clone)]
pub struct AttendanceService {
    repos: Arc<Repositories>,
}

impl AttendanceService {
    pub fn new(repos: Arc<Repositories>) -> Self { Self { repos } }

    /// Mark attendance for a whole class in one transaction.
    /// Any student in `marks` not in the class roster is rejected up-front
    /// (so partial data doesn't slip in).
    pub async fn mark_class(
        &self, class_section_id: i64, date: NaiveDate,
        marks: Vec<BulkMark>, marked_by_staff_id: Option<i64>,
    ) -> ServiceResult<usize> {
        let roster = self.repos.enrollments.roster(class_section_id).await?;
        let valid_ids: std::collections::HashSet<i64> =
            roster.iter().map(|e| e.student_id).collect();

        for m in &marks {
            if !valid_ids.contains(&m.student_id) {
                return Err(ServiceError::validation(
                    format!("student {} not in class {}", m.student_id, class_section_id),
                ));
            }
            if !matches!(m.status.as_str(), "present"|"absent"|"late"|"excused"|"half_day") {
                return Err(ServiceError::validation("invalid status"));
            }
        }

        let payload: Vec<MarkStudent> = marks.into_iter().map(|m| MarkStudent {
            student_id: m.student_id,
            class_section_id,
            date,
            status: m.status,
            remarks: m.remarks,
            marked_by_staff_id,
        }).collect();

        Ok(self.repos.student_attendance.mark_bulk(&payload).await?)
    }

    /// Single-student mark (upsert).
    pub async fn mark_one(&self, m: MarkStudent) -> ServiceResult<StudentAttendance> {
        Ok(self.repos.student_attendance.mark(&m).await?)
    }

    pub async fn percentage(
        &self, student_id: i64, from: NaiveDate, to: NaiveDate,
    ) -> ServiceResult<f64> {
        Ok(self.repos.student_attendance.percentage(student_id, from, to).await?)
    }

    pub async fn class_absentees(
        &self, class_section_id: i64, date: NaiveDate,
    ) -> ServiceResult<Vec<StudentAttendance>> {
        let all = self.repos.student_attendance
            .for_class_on(class_section_id, date).await?;
        Ok(all.into_iter().filter(|a| a.status == "absent").collect())
    }
}
