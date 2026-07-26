use axum::{
    extract::{Path, Query},
    http::StatusCode,
    Json,
};
use serde::Deserialize;

use crate::http::{ServiceHttpError, TenantScope};
use crate::models::people::{NewStaff, Staff, UpdateStaff};
use crate::services::people as people_svc;
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
