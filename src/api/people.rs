use axum::{
    extract::{Path, Query},
    http::StatusCode,
    Json,
};
use serde::Deserialize;

use crate::http::{ServiceHttpError, TenantScope};
use crate::models::people::{NewStaff, Staff, Student, UpdateStaff, UpdateStudent};
use crate::services::people::{self as people_svc, Admission, AdmissionResult};
use crate::services::perm;

#[derive(Deserialize)]
pub struct Page {
    #[serde(default = "d50")]
    pub limit: i64,
    #[serde(default)]
    pub offset: i64,
}
fn d50() -> i64 {
    50
}

pub async fn list_students(
    scope: TenantScope,
    Query(p): Query<Page>,
) -> Result<Json<Vec<Student>>, ServiceHttpError> {
    Ok(Json(
        people_svc::list_students_for(&scope.pool, &scope.ctx, p.limit, p.offset).await?,
    ))
}

#[derive(Deserialize)]
pub struct SearchQ {
    pub q: String,
    #[serde(default = "d50")]
    pub limit: i64,
}

pub async fn search_students(
    scope: TenantScope,
    Query(q): Query<SearchQ>,
) -> Result<Json<Vec<Student>>, ServiceHttpError> {
    scope
        .ctx
        .require_any(&[perm::STUDENTS_VIEW, perm::STUDENTS_VIEW_OWN])?;
    Ok(Json(
        people_svc::search_students(&scope.pool, &q.q, q.limit).await?,
    ))
}

pub async fn get_student(
    scope: TenantScope,
    Path((_t, id)): Path<(String, i64)>,
) -> Result<Json<Student>, ServiceHttpError> {
    scope
        .ctx
        .require_any(&[perm::STUDENTS_VIEW, perm::STUDENTS_VIEW_OWN])?;
    Ok(Json(people_svc::get_student(&scope.pool, id).await?))
}

pub async fn update_student(
    scope: TenantScope,
    Path((_t, id)): Path<(String, i64)>,
    Json(b): Json<UpdateStudent>,
) -> Result<Json<Student>, ServiceHttpError> {
    scope.ctx.require(perm::STUDENTS_EDIT)?;
    Ok(Json(people_svc::update_student(&scope.pool, id, &b).await?))
}

pub async fn delete_student(
    scope: TenantScope,
    Path((_t, id)): Path<(String, i64)>,
) -> Result<StatusCode, ServiceHttpError> {
    scope.ctx.require(perm::STUDENTS_EDIT)?;
    people_svc::delete_student(&scope.pool, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn admit(
    scope: TenantScope,
    Json(b): Json<Admission>,
) -> Result<Json<AdmissionResult>, ServiceHttpError> {
    Ok(Json(people_svc::admit(&scope.pool, &scope.ctx, b).await?))
}

pub async fn withdraw(
    scope: TenantScope,
    Path((_t, id)): Path<(String, i64)>,
) -> Result<StatusCode, ServiceHttpError> {
    people_svc::withdraw(&scope.pool, &scope.ctx, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn graduate(
    scope: TenantScope,
    Path((_t, id)): Path<(String, i64)>,
) -> Result<StatusCode, ServiceHttpError> {
    people_svc::graduate(&scope.pool, &scope.ctx, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn list_staff(
    scope: TenantScope,
    Query(p): Query<Page>,
) -> Result<Json<Vec<Staff>>, ServiceHttpError> {
    scope.ctx.require(perm::STAFF_VIEW)?;
    Ok(Json(
        people_svc::list_staff(&scope.pool, p.limit, p.offset).await?,
    ))
}

pub async fn hire(
    scope: TenantScope,
    Json(b): Json<NewStaff>,
) -> Result<Json<Staff>, ServiceHttpError> {
    Ok(Json(
        people_svc::hire_staff(&scope.pool, &scope.ctx, b).await?,
    ))
}

pub async fn get_staff(
    scope: TenantScope,
    Path((_t, id)): Path<(String, i64)>,
) -> Result<Json<Staff>, ServiceHttpError> {
    scope.ctx.require(perm::STAFF_VIEW)?;
    Ok(Json(people_svc::get_staff(&scope.pool, id).await?))
}

pub async fn update_staff(
    scope: TenantScope,
    Path((_t, id)): Path<(String, i64)>,
    Json(b): Json<UpdateStaff>,
) -> Result<Json<Staff>, ServiceHttpError> {
    scope.ctx.require(perm::STAFF_EDIT)?;
    Ok(Json(people_svc::update_staff(&scope.pool, id, &b).await?))
}

#[derive(Deserialize)]
pub struct Terminate {
    on: chrono::NaiveDate,
}

pub async fn terminate(
    scope: TenantScope,
    Path((_t, id)): Path<(String, i64)>,
    Json(b): Json<Terminate>,
) -> Result<StatusCode, ServiceHttpError> {
    people_svc::terminate_staff(&scope.pool, &scope.ctx, id, b.on).await?;
    Ok(StatusCode::NO_CONTENT)
}
