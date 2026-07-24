use axum::{extract::{Path, Query}, http::StatusCode, Json};
use serde::Deserialize;

use crate::http::{ServiceHttpError, TenantScope};
use crate::repositories::staff::{NewStaff, Staff, UpdateStaff};
use crate::repositories::students::{Student, UpdateStudent};
use crate::services::people::{Admission, AdmissionResult};

#[derive(Deserialize)] pub struct Page { #[serde(default = "d50")] pub limit: i64, #[serde(default)] pub offset: i64 }
fn d50() -> i64 { 50 }

pub async fn list_students(scope: TenantScope, Query(p): Query<Page>)
    -> Result<Json<Vec<Student>>, ServiceHttpError>
{ Ok(Json(scope.services.repos.students.list(p.limit, p.offset).await?)) }

#[derive(Deserialize)] pub struct SearchQ { pub q: String, #[serde(default = "d50")] pub limit: i64 }

pub async fn search_students(scope: TenantScope, Query(q): Query<SearchQ>)
    -> Result<Json<Vec<Student>>, ServiceHttpError>
{ Ok(Json(scope.services.repos.students.search(&q.q, q.limit).await?)) }

pub async fn get_student(scope: TenantScope, Path((_t, id)): Path<(String, i64)>)
    -> Result<Json<Student>, ServiceHttpError>
{ Ok(Json(scope.services.repos.students.get(id).await?)) }

pub async fn update_student(
    scope: TenantScope, Path((_t, id)): Path<(String, i64)>, Json(b): Json<UpdateStudent>,
) -> Result<Json<Student>, ServiceHttpError>
{ Ok(Json(scope.services.repos.students.update(id, &b).await?)) }

pub async fn delete_student(scope: TenantScope, Path((_t, id)): Path<(String, i64)>)
    -> Result<StatusCode, ServiceHttpError>
{
    scope.services.repos.students.delete(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn admit(scope: TenantScope, Json(b): Json<Admission>)
    -> Result<Json<AdmissionResult>, ServiceHttpError>
{ Ok(Json(scope.services.people.admit(b).await?)) }

pub async fn withdraw(scope: TenantScope, Path((_t, id)): Path<(String, i64)>)
    -> Result<StatusCode, ServiceHttpError>
{ scope.services.people.withdraw(id).await?; Ok(StatusCode::NO_CONTENT) }

pub async fn graduate(scope: TenantScope, Path((_t, id)): Path<(String, i64)>)
    -> Result<StatusCode, ServiceHttpError>
{ scope.services.people.graduate(id).await?; Ok(StatusCode::NO_CONTENT) }

pub async fn list_staff(scope: TenantScope, Query(p): Query<Page>)
    -> Result<Json<Vec<Staff>>, ServiceHttpError>
{ Ok(Json(scope.services.repos.staff.list(p.limit, p.offset).await?)) }

pub async fn hire(scope: TenantScope, Json(b): Json<NewStaff>)
    -> Result<Json<Staff>, ServiceHttpError>
{ Ok(Json(scope.services.people.hire_staff(b).await?)) }

pub async fn get_staff(scope: TenantScope, Path((_t, id)): Path<(String, i64)>)
    -> Result<Json<Staff>, ServiceHttpError>
{ Ok(Json(scope.services.repos.staff.get(id).await?)) }

pub async fn update_staff(
    scope: TenantScope, Path((_t, id)): Path<(String, i64)>, Json(b): Json<UpdateStaff>,
) -> Result<Json<Staff>, ServiceHttpError>
{ Ok(Json(scope.services.repos.staff.update(id, &b).await?)) }

#[derive(Deserialize)] pub struct Terminate { on: chrono::NaiveDate }

pub async fn terminate(
    scope: TenantScope, Path((_t, id)): Path<(String, i64)>, Json(b): Json<Terminate>,
) -> Result<StatusCode, ServiceHttpError>
{
    scope.services.people.terminate_staff(id, b.on).await?;
    Ok(StatusCode::NO_CONTENT)
}
