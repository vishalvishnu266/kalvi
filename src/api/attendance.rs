//! `/api/{tenant}/attendance/*` handlers.

use axum::{extract::{Path, Query}, Json};
use serde::Deserialize;

use crate::http::{ServiceHttpError, TenantScope};
use crate::repositories::attendance::{MarkStaff, MarkStudent, StaffAttendance, StudentAttendance};
use crate::services::attendance::BulkMark;

pub async fn mark_one(scope: TenantScope, Json(b): Json<MarkStudent>)
    -> Result<Json<StudentAttendance>, ServiceHttpError>
{ Ok(Json(scope.services.attendance.mark_one(b).await?)) }

#[derive(Deserialize)]
pub struct MarkClassBody {
    class_section_id: i64,
    date: chrono::NaiveDate,
    marks: Vec<BulkMark>,
    marked_by_staff_id: Option<i64>,
}

pub async fn mark_class(scope: TenantScope, Json(b): Json<MarkClassBody>)
    -> Result<Json<serde_json::Value>, ServiceHttpError>
{
    let n = scope.services.attendance
        .mark_class(b.class_section_id, b.date, b.marks, b.marked_by_staff_id).await?;
    Ok(Json(serde_json::json!({ "marked": n })))
}

#[derive(Deserialize)] pub struct Range { from: chrono::NaiveDate, to: chrono::NaiveDate }

pub async fn for_student(
    scope: TenantScope, Path((_t, sid)): Path<(String, i64)>, Query(r): Query<Range>,
) -> Result<Json<Vec<StudentAttendance>>, ServiceHttpError> {
    Ok(Json(scope.services.repos.student_attendance
        .for_student_between(sid, r.from, r.to).await?))
}

#[derive(Deserialize)] pub struct PctQ { student_id: i64, from: chrono::NaiveDate, to: chrono::NaiveDate }

pub async fn percentage(scope: TenantScope, Query(q): Query<PctQ>)
    -> Result<Json<serde_json::Value>, ServiceHttpError>
{
    let p = scope.services.attendance.percentage(q.student_id, q.from, q.to).await?;
    Ok(Json(serde_json::json!({ "percentage": p })))
}

#[derive(Deserialize)] pub struct DateOnly { date: chrono::NaiveDate }

pub async fn for_class_on(
    scope: TenantScope, Path((_t, id)): Path<(String, i64)>, Query(q): Query<DateOnly>,
) -> Result<Json<Vec<StudentAttendance>>, ServiceHttpError> {
    Ok(Json(scope.services.repos.student_attendance.for_class_on(id, q.date).await?))
}

pub async fn mark_staff(scope: TenantScope, Json(b): Json<MarkStaff>)
    -> Result<Json<StaffAttendance>, ServiceHttpError>
{ Ok(Json(scope.services.repos.staff_attendance.mark(&b).await?)) }

pub async fn for_staff(
    scope: TenantScope, Path((_t, sid)): Path<(String, i64)>, Query(r): Query<Range>,
) -> Result<Json<Vec<StaffAttendance>>, ServiceHttpError> {
    Ok(Json(scope.services.repos.staff_attendance
        .for_staff_between(sid, r.from, r.to).await?))
}
