use std::sync::Arc;

use chrono::NaiveDate;

use crate::repositories::Repositories;
use crate::repositories::transport::StudentTransport;
use crate::services::{ServiceError, ServiceResult};

#[derive(Clone)]
pub struct TransportService {
    repos: Arc<Repositories>,
}

impl TransportService {
    pub fn new(repos: Arc<Repositories>) -> Self { Self { repos } }

pub async fn assign_to_stop(
        &self, student_id: i64, route_stop_id: i64, valid_from: NaiveDate,
    ) -> ServiceResult<StudentTransport> {

        let stop = sqlx::query_as::<_, (i64,)>(
            "SELECT route_id FROM route_stop WHERE id = ?",
        )
        .bind(route_stop_id)
        .fetch_optional(&self.repos.pool).await?
        .ok_or(ServiceError::NotFound)?;

        let route = self.repos.routes.get(stop.0).await?;
        if let Some(vehicle_id) = route.vehicle_id {
            let vehicle = self.repos.vehicles.get(vehicle_id).await?;
            if let Some(cap) = vehicle.capacity {
                let riders = self.repos.student_transport
                    .students_on_route(route.id).await?
                    .len() as i64;
                if riders >= cap {
                    return Err(ServiceError::conflict("route vehicle is full"));
                }
            }
        }

        let year = self.repos.academic_years.current().await?;
        Ok(self.repos.student_transport
            .assign(student_id, route_stop_id, year.id, valid_from).await?)
    }

    pub async fn end_assignment(&self, id: i64, on: NaiveDate) -> ServiceResult<()> {
        self.repos.student_transport.end_assignment(id, on).await?;
        Ok(())
    }
}
