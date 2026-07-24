use std::sync::Arc;

use chrono::NaiveDate;

use crate::repositories::Repositories;
use crate::repositories::hostel::HostelAllocation;
use crate::services::{RequestCtx, ServiceError, ServiceResult};
use crate::services::perm;

#[derive(Clone)]
pub struct HostelService {
    repos: Arc<Repositories>,
}

impl HostelService {
    pub fn new(repos: Arc<Repositories>) -> Self { Self { repos } }

    pub async fn allocate(
        &self, ctx: &RequestCtx, student_id: i64, hostel_room_id: i64, from_date: NaiveDate,
    ) -> ServiceResult<HostelAllocation> {
        ctx.require(perm::HOSTEL_MANAGE)?;
        Ok(self.repos.hostel_allocations
            .allocate(student_id, hostel_room_id, from_date).await?)
    }

    pub async fn transfer_room(
        &self, ctx: &RequestCtx, student_id: i64, to_room_id: i64, from_date: NaiveDate,
    ) -> ServiceResult<HostelAllocation> {
        ctx.require(perm::HOSTEL_MANAGE)?;
        let active = self.repos.hostel_allocations.active_for_student(student_id).await?;
        if let Some(a) = active {
            if a.hostel_room_id == to_room_id {
                return Err(ServiceError::validation("already in target room"));
            }
        }
        self.allocate(ctx, student_id, to_room_id, from_date).await
    }

    pub async fn vacate(&self, ctx: &RequestCtx, student_id: i64, to_date: NaiveDate) -> ServiceResult<()> {
        ctx.require(perm::HOSTEL_MANAGE)?;
        let active = self.repos.hostel_allocations.active_for_student(student_id).await?
            .ok_or(ServiceError::NotFound)?;
        self.repos.hostel_allocations.vacate(active.id, to_date).await?;
        Ok(())
    }
}
