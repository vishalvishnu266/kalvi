use axum::{extract::Path, http::StatusCode, Json};

use crate::http::{ServiceHttpError, TenantScope};
use crate::models::guardians::{Guardian, NewGuardian, StudentGuardianLink};
use crate::services::guardians as g_svc;
use crate::services::perm;

pub async fn list(scope: TenantScope) -> Result<Json<Vec<Guardian>>, ServiceHttpError> {
    scope
        .ctx
        .require_any(&[perm::GUARDIANS_VIEW, perm::GUARDIANS_MANAGE])?;
    Ok(Json(g_svc::list(&scope.pool, 200, 0).await?))
}

pub async fn create(
    scope: TenantScope,
    Json(b): Json<NewGuardian>,
) -> Result<Json<Guardian>, ServiceHttpError> {
    scope.ctx.require(perm::GUARDIANS_MANAGE)?;
    Ok(Json(g_svc::create(&scope.pool, &b).await?))
}

pub async fn get_one(
    scope: TenantScope,
    Path((_t, id)): Path<(String, i64)>,
) -> Result<Json<Guardian>, ServiceHttpError> {
    scope
        .ctx
        .require_any(&[perm::GUARDIANS_VIEW, perm::GUARDIANS_MANAGE])?;
    Ok(Json(g_svc::get(&scope.pool, id).await?))
}

pub async fn remove(
    scope: TenantScope,
    Path((_t, id)): Path<(String, i64)>,
) -> Result<StatusCode, ServiceHttpError> {
    scope.ctx.require(perm::GUARDIANS_MANAGE)?;
    g_svc::delete(&scope.pool, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn link(
    scope: TenantScope,
    Json(b): Json<StudentGuardianLink>,
) -> Result<StatusCode, ServiceHttpError> {
    scope.ctx.require(perm::GUARDIANS_MANAGE)?;
    g_svc::link(&scope.pool, &b).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn unlink(
    scope: TenantScope,
    Path((_t, sid, gid)): Path<(String, i64, i64)>,
) -> Result<StatusCode, ServiceHttpError> {
    scope.ctx.require(perm::GUARDIANS_MANAGE)?;
    g_svc::unlink(&scope.pool, sid, gid).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn of_student(
    scope: TenantScope,
    Path((_t, sid)): Path<(String, i64)>,
) -> Result<Json<Vec<Guardian>>, ServiceHttpError> {
    scope
        .ctx
        .require_any(&[perm::GUARDIANS_VIEW, perm::GUARDIANS_MANAGE])?;
    Ok(Json(g_svc::guardians_of_student(&scope.pool, sid).await?))
}
