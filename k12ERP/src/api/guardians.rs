//! `/api/tenant/guardians/*`

use axum::{
    extract::Path,
    routing::{get, post, delete},
    Json, Router,
};

use crate::http::{ExtractServices, ServiceHttpError};
use crate::http::middleware::TenantScopeState;
use crate::repositories::guardians::{Guardian, NewGuardian, StudentGuardianLink};

pub fn routes() -> Router<TenantScopeState> {
    Router::new()
        .route("/",                    get(list).post(create))
        .route("/{id}",                 get(get_one).delete(remove))
        .route("/link",                 post(link))
        .route("/link/{sid}/{gid}",     delete(unlink))
        .route("/of-student/{sid}",     get(of_student))
}

async fn list(ExtractServices(a): ExtractServices) -> Result<Json<Vec<Guardian>>, ServiceHttpError> {
    Ok(Json(a.repos.guardians.list(200, 0).await.map_err(rerr)?))
}
async fn create(ExtractServices(a): ExtractServices, Json(b): Json<NewGuardian>)
    -> Result<Json<Guardian>, ServiceHttpError>
{ Ok(Json(a.repos.guardians.create(&b).await.map_err(rerr)?)) }

async fn get_one(ExtractServices(a): ExtractServices, Path(id): Path<i64>)
    -> Result<Json<Guardian>, ServiceHttpError>
{ Ok(Json(a.repos.guardians.get(id).await.map_err(rerr)?)) }

async fn remove(ExtractServices(a): ExtractServices, Path(id): Path<i64>)
    -> Result<axum::http::StatusCode, ServiceHttpError>
{ a.repos.guardians.delete(id).await.map_err(rerr)?; Ok(axum::http::StatusCode::NO_CONTENT) }

async fn link(ExtractServices(a): ExtractServices, Json(b): Json<StudentGuardianLink>)
    -> Result<axum::http::StatusCode, ServiceHttpError>
{ a.repos.guardians.link(&b).await.map_err(rerr)?; Ok(axum::http::StatusCode::NO_CONTENT) }

async fn unlink(ExtractServices(a): ExtractServices, Path((sid, gid)): Path<(i64,i64)>)
    -> Result<axum::http::StatusCode, ServiceHttpError>
{ a.repos.guardians.unlink(sid, gid).await.map_err(rerr)?; Ok(axum::http::StatusCode::NO_CONTENT) }

async fn of_student(ExtractServices(a): ExtractServices, Path(sid): Path<i64>)
    -> Result<Json<Vec<Guardian>>, ServiceHttpError>
{ Ok(Json(a.repos.guardians.guardians_of_student(sid).await.map_err(rerr)?)) }

fn rerr(e: crate::error::RepoError) -> ServiceHttpError { ServiceHttpError(e.into()) }
