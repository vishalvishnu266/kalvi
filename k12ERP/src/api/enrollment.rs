//! `/api/tenant/enrollment/*`

use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;

use crate::http::{ExtractServices, ServiceHttpError};
use crate::http::middleware::TenantScopeState;
use crate::repositories::class_enrollment::{Enrollment, NewEnrollment};

pub fn routes() -> Router<TenantScopeState> {
    Router::new()
        .route("/",                       post(enroll))
        .route("/transfer",               post(transfer))
        .route("/close-current",          post(close_current))
        .route("/roster/{class_section_id}", get(roster))
        .route("/history/{student_id}",    get(history))
        .route("/promote",                post(promote_class))
}

async fn enroll(ExtractServices(a): ExtractServices, Json(b): Json<NewEnrollment>)
    -> Result<Json<Enrollment>, ServiceHttpError>
{ Ok(Json(a.enrollment.enroll(b).await?)) }

#[derive(Deserialize)]
struct Transfer { student_id: i64, to_class_section_id: i64, effective: chrono::NaiveDate }

async fn transfer(ExtractServices(a): ExtractServices, Json(b): Json<Transfer>)
    -> Result<Json<Enrollment>, ServiceHttpError>
{ Ok(Json(a.enrollment.transfer(b.student_id, b.to_class_section_id, b.effective).await?)) }

#[derive(Deserialize)]
struct Close { student_id: i64, on: chrono::NaiveDate, result: String }

async fn close_current(ExtractServices(a): ExtractServices, Json(b): Json<Close>)
    -> Result<axum::http::StatusCode, ServiceHttpError>
{
    a.enrollment.close_current(b.student_id, b.on, &b.result).await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}

async fn roster(ExtractServices(a): ExtractServices, Path(id): Path<i64>)
    -> Result<Json<Vec<Enrollment>>, ServiceHttpError>
{ Ok(Json(a.enrollment.roster(id).await?)) }

async fn history(ExtractServices(a): ExtractServices, Path(id): Path<i64>)
    -> Result<Json<Vec<Enrollment>>, ServiceHttpError>
{ Ok(Json(a.repos.enrollments.history_for_student(id).await.map_err(|e| ServiceHttpError(e.into()))?)) }

#[derive(Deserialize)]
struct Promote { from_class_id: i64, to_class_id: i64, to_year_id: i64, enrolled_on: chrono::NaiveDate }

async fn promote_class(ExtractServices(a): ExtractServices, Json(b): Json<Promote>)
    -> Result<Json<serde_json::Value>, ServiceHttpError>
{
    let n = a.academic.promote_class(b.from_class_id, b.to_class_id, b.to_year_id, b.enrolled_on).await?;
    Ok(Json(serde_json::json!({ "promoted": n })))
}
