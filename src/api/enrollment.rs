use axum::{extract::Path, http::StatusCode, Json};
use serde::Deserialize;

use crate::http::{ServiceHttpError, TenantScope};
use crate::repositories::class_enrollment::{Enrollment, NewEnrollment};

pub async fn enroll(scope: TenantScope, Json(b): Json<NewEnrollment>)
    -> Result<Json<Enrollment>, ServiceHttpError>
{ Ok(Json(scope.services.enrollment.enroll(b).await?)) }

#[derive(Deserialize)]
pub struct Transfer { student_id: i64, to_class_section_id: i64, effective: chrono::NaiveDate }

pub async fn transfer(scope: TenantScope, Json(b): Json<Transfer>)
    -> Result<Json<Enrollment>, ServiceHttpError>
{ Ok(Json(scope.services.enrollment.transfer(b.student_id, b.to_class_section_id, b.effective).await?)) }

#[derive(Deserialize)]
pub struct Close { student_id: i64, on: chrono::NaiveDate, result: String }

pub async fn close_current(scope: TenantScope, Json(b): Json<Close>)
    -> Result<StatusCode, ServiceHttpError>
{
    scope.services.enrollment.close_current(b.student_id, b.on, &b.result).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn roster(scope: TenantScope, Path((_t, id)): Path<(String, i64)>)
    -> Result<Json<Vec<Enrollment>>, ServiceHttpError>
{ Ok(Json(scope.services.enrollment.roster(id).await?)) }

pub async fn history(scope: TenantScope, Path((_t, id)): Path<(String, i64)>)
    -> Result<Json<Vec<Enrollment>>, ServiceHttpError>
{ Ok(Json(scope.services.repos.enrollments.history_for_student(id).await?)) }

#[derive(Deserialize)]
pub struct Promote { from_class_id: i64, to_class_id: i64, to_year_id: i64, enrolled_on: chrono::NaiveDate }

pub async fn promote_class(scope: TenantScope, Json(b): Json<Promote>)
    -> Result<Json<serde_json::Value>, ServiceHttpError>
{
    let n = scope.services.academic.promote_class(b.from_class_id, b.to_class_id, b.to_year_id, b.enrolled_on).await?;
    Ok(Json(serde_json::json!({ "promoted": n })))
}
