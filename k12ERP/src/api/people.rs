//! `/api/tenant/people/*` — students & staff.

use axum::{
    extract::{Path, Query},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;

use crate::http::{ExtractServices, ServiceHttpError};
use crate::http::middleware::TenantScopeState;
use crate::repositories::staff::{NewStaff, Staff, UpdateStaff};
use crate::repositories::students::{NewStudent, Student, UpdateStudent};
use crate::services::people::{Admission, AdmissionResult};

pub fn routes() -> Router<TenantScopeState> {
    Router::new()
        // Students
        .route("/students",              get(list_students))
        .route("/students/search",       get(search_students))
        .route("/students/{id}",          get(get_student).put(update_student).delete(delete_student))
        .route("/students/admit",         post(admit))
        .route("/students/{id}/withdraw", post(withdraw))
        .route("/students/{id}/graduate", post(graduate))
        // Staff
        .route("/staff",           get(list_staff).post(hire))
        .route("/staff/{id}",       get(get_staff).put(update_staff))
        .route("/staff/{id}/terminate", post(terminate))
}

// --- Students ---
#[derive(Deserialize)] struct Page { #[serde(default = "d50")] limit: i64, #[serde(default)] offset: i64 }
fn d50() -> i64 { 50 }

async fn list_students(ExtractServices(a): ExtractServices, Query(p): Query<Page>)
    -> Result<Json<Vec<Student>>, ServiceHttpError>
{ Ok(Json(a.repos.students.list(p.limit, p.offset).await.map_err(rerr)?)) }

#[derive(Deserialize)] struct SearchQ { q: String, #[serde(default = "d50")] limit: i64 }

async fn search_students(ExtractServices(a): ExtractServices, Query(q): Query<SearchQ>)
    -> Result<Json<Vec<Student>>, ServiceHttpError>
{ Ok(Json(a.repos.students.search(&q.q, q.limit).await.map_err(rerr)?)) }

async fn get_student(ExtractServices(a): ExtractServices, Path(id): Path<i64>)
    -> Result<Json<Student>, ServiceHttpError>
{ Ok(Json(a.repos.students.get(id).await.map_err(rerr)?)) }

async fn update_student(
    ExtractServices(a): ExtractServices, Path(id): Path<i64>, Json(b): Json<UpdateStudent>,
) -> Result<Json<Student>, ServiceHttpError>
{ Ok(Json(a.repos.students.update(id, &b).await.map_err(rerr)?)) }

async fn delete_student(ExtractServices(a): ExtractServices, Path(id): Path<i64>)
    -> Result<axum::http::StatusCode, ServiceHttpError>
{
    a.repos.students.delete(id).await.map_err(rerr)?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}

async fn admit(ExtractServices(a): ExtractServices, Json(b): Json<Admission>)
    -> Result<Json<AdmissionResult>, ServiceHttpError>
{ Ok(Json(a.people.admit(b).await?)) }

async fn withdraw(ExtractServices(a): ExtractServices, Path(id): Path<i64>)
    -> Result<axum::http::StatusCode, ServiceHttpError>
{ a.people.withdraw(id).await?; Ok(axum::http::StatusCode::NO_CONTENT) }

async fn graduate(ExtractServices(a): ExtractServices, Path(id): Path<i64>)
    -> Result<axum::http::StatusCode, ServiceHttpError>
{ a.people.graduate(id).await?; Ok(axum::http::StatusCode::NO_CONTENT) }

// --- Staff ---
async fn list_staff(ExtractServices(a): ExtractServices, Query(p): Query<Page>)
    -> Result<Json<Vec<Staff>>, ServiceHttpError>
{ Ok(Json(a.repos.staff.list(p.limit, p.offset).await.map_err(rerr)?)) }

async fn hire(ExtractServices(a): ExtractServices, Json(b): Json<NewStaff>)
    -> Result<Json<Staff>, ServiceHttpError>
{ Ok(Json(a.people.hire_staff(b).await?)) }

async fn get_staff(ExtractServices(a): ExtractServices, Path(id): Path<i64>)
    -> Result<Json<Staff>, ServiceHttpError>
{ Ok(Json(a.repos.staff.get(id).await.map_err(rerr)?)) }

async fn update_staff(
    ExtractServices(a): ExtractServices, Path(id): Path<i64>, Json(b): Json<UpdateStaff>,
) -> Result<Json<Staff>, ServiceHttpError>
{ Ok(Json(a.repos.staff.update(id, &b).await.map_err(rerr)?)) }

#[derive(Deserialize)] struct Terminate { on: chrono::NaiveDate }

async fn terminate(
    ExtractServices(a): ExtractServices, Path(id): Path<i64>, Json(b): Json<Terminate>,
) -> Result<axum::http::StatusCode, ServiceHttpError>
{
    a.people.terminate_staff(id, b.on).await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}

fn rerr(e: crate::error::RepoError) -> ServiceHttpError {
    ServiceHttpError(crate::ServiceError::from(e))
}
