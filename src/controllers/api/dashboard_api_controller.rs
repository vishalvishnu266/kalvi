use axum::{response::IntoResponse, Extension, Json};
use sqlx::SqlitePool;

use crate::errors::AppError;
use crate::services::student_service::StudentService;
use serde::Serialize;
use crate::models::student::Student;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct DashboardStatsResponse {
    pub total_students: i64,
    pub active_students: i64,
    pub pending_students: i64,
    pub inactive_students: i64,
    pub recent_students: Vec<Student>,
}

/// Get dashboard statistics for a tenant.
#[utoipa::path(
    get,
    path = "/api/{tenant_id}/dashboard-stats",
    responses(
        (status = 200, description = "Dashboard stats retrieved successfully", body = DashboardStatsResponse),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("tenant_id" = String, Path, description = "Tenant identifier")
    )
)]
pub async fn get_dashboard_stats(
    Extension(pool): Extension<SqlitePool>,
) -> Result<impl IntoResponse, AppError> {
    let (total, active, pending, inactive, recent) =
        StudentService::get_dashboard_stats(&pool).await?;

    Ok(Json(DashboardStatsResponse {
        total_students: total,
        active_students: active,
        pending_students: pending,
        inactive_students: inactive,
        recent_students: recent,
    }))
}
