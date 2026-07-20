//! `/api/tenant/payroll/*`

use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;

use crate::http::{ExtractServices, ServiceHttpError};
use crate::http::middleware::TenantScopeState;
use crate::repositories::payroll::{NewStructureItem, Payslip, SalaryComponent};

pub fn routes() -> Router<TenantScopeState> {
    Router::new()
        .route("/components",              get(list_components))
        .route("/structures/set",          post(set_salary))
        .route("/payslips/generate",       post(generate))
        .route("/payslips/month",          post(generate_month))
        .route("/payslips/{id}/approve",    post(approve))
        .route("/payslips/{id}/pay",        post(pay))
        .route("/payslips/staff/{sid}/{year}", get(list_for_staff))
}

async fn list_components(ExtractServices(a): ExtractServices)
    -> Result<Json<Vec<SalaryComponent>>, ServiceHttpError>
{ Ok(Json(a.repos.salary_components.list().await.map_err(re)?)) }

#[derive(Deserialize)]
struct SetSalary { staff_id: i64, effective_from: chrono::NaiveDate, items: Vec<NewStructureItem> }

async fn set_salary(ExtractServices(a): ExtractServices, Json(b): Json<SetSalary>)
    -> Result<Json<serde_json::Value>, ServiceHttpError>
{
    let id = a.payroll.set_salary(b.staff_id, b.effective_from, b.items).await?;
    Ok(Json(serde_json::json!({ "id": id })))
}

#[derive(Deserialize)] struct Generate { staff_id: i64, month: i64, year: i64 }

async fn generate(ExtractServices(a): ExtractServices, Json(b): Json<Generate>)
    -> Result<Json<Payslip>, ServiceHttpError>
{ Ok(Json(a.payroll.generate_payslip(b.staff_id, b.month, b.year).await?)) }

#[derive(Deserialize)] struct GenerateMonth { month: i64, year: i64 }

async fn generate_month(ExtractServices(a): ExtractServices, Json(b): Json<GenerateMonth>)
    -> Result<Json<serde_json::Value>, ServiceHttpError>
{
    let n = a.payroll.generate_month(b.month, b.year).await?;
    Ok(Json(serde_json::json!({ "generated": n })))
}

async fn approve(ExtractServices(a): ExtractServices, Path(id): Path<i64>)
    -> Result<axum::http::StatusCode, ServiceHttpError>
{ a.payroll.approve(id).await?; Ok(axum::http::StatusCode::NO_CONTENT) }

#[derive(Deserialize)] struct PayBody { paid_on: chrono::NaiveDate, #[serde(default)] from_bank: bool }

async fn pay(ExtractServices(a): ExtractServices, Path(id): Path<i64>, Json(b): Json<PayBody>)
    -> Result<axum::http::StatusCode, ServiceHttpError>
{ a.payroll.pay(id, b.paid_on, b.from_bank).await?; Ok(axum::http::StatusCode::NO_CONTENT) }

async fn list_for_staff(ExtractServices(a): ExtractServices, Path((sid, year)): Path<(i64,i64)>)
    -> Result<Json<Vec<Payslip>>, ServiceHttpError>
{ Ok(Json(a.repos.payslips.for_staff(sid, year).await.map_err(re)?)) }

fn re(e: crate::error::RepoError) -> ServiceHttpError { ServiceHttpError(e.into()) }
