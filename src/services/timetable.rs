use std::sync::Arc;

use crate::repositories::Repositories;
use crate::repositories::timetable::{NewSlot, TimetableSlot};
use crate::services::{ServiceError, ServiceResult};

#[derive(Clone)]
pub struct TimetableService {
    repos: Arc<Repositories>,
}

impl TimetableService {
    pub fn new(repos: Arc<Repositories>) -> Self { Self { repos } }

    pub async fn set_slot(&self, s: NewSlot) -> ServiceResult<TimetableSlot> {
        if !(1..=7).contains(&s.day_of_week) {
            return Err(ServiceError::validation("day_of_week must be 1..7"));
        }

        if let Some(subj_id) = s.subject_id {
            let assigned = self.repos.class_subjects.list_for_class(s.class_section_id).await?;
            if !assigned.iter().any(|cs| cs.subject_id == subj_id) {
                return Err(ServiceError::validation(
                    "subject is not assigned to this class section",
                ));
            }
        }

Ok(self.repos.timetable.upsert(&s).await?)
    }

    pub async fn class_grid(&self, class_section_id: i64) -> ServiceResult<Vec<TimetableSlot>> {
        Ok(self.repos.timetable.for_class(class_section_id).await?)
    }

    pub async fn teacher_grid(&self, teacher_id: i64) -> ServiceResult<Vec<TimetableSlot>> {
        Ok(self.repos.timetable.for_teacher(teacher_id).await?)
    }

    pub async fn remove(&self, id: i64) -> ServiceResult<()> {
        self.repos.timetable.delete(id).await?;
        Ok(())
    }
}
