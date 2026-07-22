//! `/api/{tenant}/guardians/*` handlers.

use axum::{extract::Path, http::StatusCode, Json};

use crate::http::{ServiceHttpError, TenantScope};
use crate::repositories::guardians::{Guardian, NewGuardian, StudentGuardianLink};

pub async fn list(scope: TenantScope)
    -> Result<Json<Vec<Guardian>>, ServiceHttpError>
{ Ok(Json(scope.services.repos.guardians.list(200, 0).await?)) }

pub async fn create(scope: TenantScope, Json(b): Json<NewGuardian>)
    -> Result<Json<Guardian>, ServiceHttpError>
{ Ok(Json(scope.services.repos.guardians.create(&b).await?)) }

pub async fn get_one(scope: TenantScope, Path((_t, id)): Path<(String, i64)>)
    -> Result<Json<Guardian>, ServiceHttpError>
{ Ok(Json(scope.services.repos.guardians.get(id).await?)) }

pub async fn remove(scope: TenantScope, Path((_t, id)): Path<(String, i64)>)
    -> Result<StatusCode, ServiceHttpError>
{ scope.services.repos.guardians.delete(id).await?; Ok(StatusCode::NO_CONTENT) }

pub async fn link(scope: TenantScope, Json(b): Json<StudentGuardianLink>)
    -> Result<StatusCode, ServiceHttpError>
{ scope.services.repos.guardians.link(&b).await?; Ok(StatusCode::NO_CONTENT) }

pub async fn unlink(scope: TenantScope, Path((_t, sid, gid)): Path<(String, i64, i64)>)
    -> Result<StatusCode, ServiceHttpError>
{ scope.services.repos.guardians.unlink(sid, gid).await?; Ok(StatusCode::NO_CONTENT) }

pub async fn of_student(scope: TenantScope, Path((_t, sid)): Path<(String, i64)>)
    -> Result<Json<Vec<Guardian>>, ServiceHttpError>
{ Ok(Json(scope.services.repos.guardians.guardians_of_student(sid).await?)) }
