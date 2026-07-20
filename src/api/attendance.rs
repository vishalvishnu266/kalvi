//! `/api/tenant/attendance/*`

use axum::{
    extract::{Path, Query},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;

use crate::http::{ExtractServices, ServiceHttpError};
use crate::http::middleware::TenantScopeState;
use crate::repositories::attendance::{MarkStaff, MarkStudent, StaffAttendance, StudentAttendance};
use crate::services::attendance::BulkMark;

pub fn routes() -> Router<TenantScopeState> {
    Router::new()
        .route("/students/mark",           post(mark_one))
        .route("/students/mark-class",     post(mark_class))
        .route("/students/for/{sid}",       get(for_student))
        .route("/students/percentage",     get(percentage))
        .route("/students/class/{id}",      get(for_class_on))
        .route("/staff/mark",              post(mark_staff))
        .route("/staff/for/{sid}",          get(for_staff))
}

async fn mark_one(ExtractServices(a): ExtractServices, Json(b): Json<MarkStudent>)
    -> Result<Json<StudentAttendance>, ServiceHttpError>
{ Ok(Json(a.attendance.mark_one(b).await?)) }

#[derive(Deserialize)]
struct MarkClassBody {
    class_section_id: i64,
    date: chrono::NaiveDate,
    marks: Vec<BulkMark>,
    marked_by_staff_id: Option<i64>,
}

async fn mark_class(ExtractServices(a): ExtractServices, Json(b): Json<MarkClassBody>)
    -> Result<Json<serde_json::Value>, ServiceHttpError>
{
    let n = a.attendance.mark_class(b.class_section_id, b.date, b.marks, b.marked_by_staff_id).await?;
    Ok(Json(serde_json::json!({ "marked": n })))
}

#[derive(Deserialize)] struct Range { from: chrono::NaiveDate, to: chrono::NaiveDate }

async fn for_student(
    ExtractServices(a): ExtractServices, Path(sid): Path<i64>, Query(r): Query<Range>,
) -> Result<Json<Vec<StudentAttendance>>, ServiceHttpError> {
    Ok(Json(a.repos.student_attendance
        .for_student_between(sid, r.from, r.to).await.map_err(|e| ServiceHttpError(e.into()))?))
}

#[derive(Deserialize)] struct PctQ { student_id: i64, from: chrono::NaiveDate, to: chrono::NaiveDate }

async fn percentage(ExtractServices(a): ExtractServices, Query(q): Query<PctQ>)
    -> Result<Json<serde_json::Value>, ServiceHttpError>
{
    let p = a.attendance.percentage(q.student_id, q.from, q.to).await?;
    Ok(Json(serde_json::json!({ "percentage": p })))
}

#[derive(Deserialize)] struct DateOnly { date: chrono::NaiveDate }

async fn for_class_on(
    ExtractServices(a): ExtractServices, Path(id): Path<i64>, Query(q): Query<DateOnly>,
) -> Result<Json<Vec<StudentAttendance>>, ServiceHttpError> {
    Ok(Json(a.repos.student_attendance.for_class_on(id, q.date).await.map_err(|e| ServiceHttpError(e.into()))?))
}

async fn mark_staff(ExtractServices(a): ExtractServices, Json(b): Json<MarkStaff>)
    -> Result<Json<StaffAttendance>, ServiceHttpError>
{ Ok(Json(a.repos.staff_attendance.mark(&b).await.map_err(|e| ServiceHttpError(e.into()))?)) }

async fn for_staff(
    ExtractServices(a): ExtractServices, Path(sid): Path<i64>, Query(r): Query<Range>,
) -> Result<Json<Vec<StaffAttendance>>, ServiceHttpError> {
    Ok(Json(a.repos.staff_attendance
        .for_staff_between(sid, r.from, r.to).await.map_err(|e| ServiceHttpError(e.into()))?))
}
