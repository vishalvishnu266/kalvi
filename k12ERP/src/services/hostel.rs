//! Hostel workflows: allocate, transfer between rooms, vacate.

use std::sync::Arc;

use chrono::NaiveDate;

use crate::repositories::Repositories;
use crate::repositories::hostel::HostelAllocation;
use crate::services::{ServiceError, ServiceResult};

#[derive(Clone)]
pub struct HostelService {
    repos: Arc<Repositories>,
}

impl HostelService {
    pub fn new(repos: Arc<Repositories>) -> Self { Self { repos } }

    pub async fn allocate(
        &self, student_id: i64, hostel_room_id: i64, from_date: NaiveDate,
    ) -> ServiceResult<HostelAllocation> {
        // Repo already checks capacity + closes prior allocation.
        Ok(self.repos.hostel_allocations
            .allocate(student_id, hostel_room_id, from_date).await?)
    }

    pub async fn transfer_room(
        &self, student_id: i64, to_room_id: i64, from_date: NaiveDate,
    ) -> ServiceResult<HostelAllocation> {
        let active = self.repos.hostel_allocations.active_for_student(student_id).await?;
        if let Some(a) = active {
            if a.hostel_room_id == to_room_id {
                return Err(ServiceError::validation("already in target room"));
            }
        }
        self.allocate(student_id, to_room_id, from_date).await
    }

    pub async fn vacate(&self, student_id: i64, to_date: NaiveDate) -> ServiceResult<()> {
        let active = self.repos.hostel_allocations.active_for_student(student_id).await?
            .ok_or(ServiceError::NotFound)?;
        self.repos.hostel_allocations.vacate(active.id, to_date).await?;
        Ok(())
    }
}
