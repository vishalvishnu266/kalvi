use axum::{extract::Path, http::StatusCode, Json};
use serde::Deserialize;

use crate::http::{ServiceHttpError, TenantScope};
use crate::repositories::payroll::{NewStructureItem, Payslip, SalaryComponent};
use crate::services::perm;

pub async fn list_components(scope: TenantScope)
    -> Result<Json<Vec<SalaryComponent>>, ServiceHttpError>
{
    scope.ctx.require_any(&[perm::PAYROLL_VIEW, perm::PAYROLL_RUN])?;
    Ok(Json(scope.services.repos.salary_components.list().await?))
}

#[derive(Deserialize)]
pub struct SetSalary { staff_id: i64, effective_from: chrono::NaiveDate, items: Vec<NewStructureItem> }

pub async fn set_salary(scope: TenantScope, Json(b): Json<SetSalary>)
    -> Result<Json<serde_json::Value>, ServiceHttpError>
{
    let id = scope.services.payroll.set_salary(&scope.ctx, b.staff_id, b.effective_from, b.items).await?;
    Ok(Json(serde_json::json!({ "id": id })))
}

#[derive(Deserialize)] pub struct Generate { staff_id: i64, month: i64, year: i64 }

pub async fn generate(scope: TenantScope, Json(b): Json<Generate>)
    -> Result<Json<Payslip>, ServiceHttpError>
{ Ok(Json(scope.services.payroll.generate_payslip(&scope.ctx, b.staff_id, b.month, b.year).await?)) }

#[derive(Deserialize)] pub struct GenerateMonth { month: i64, year: i64 }

pub async fn generate_month(scope: TenantScope, Json(b): Json<GenerateMonth>)
    -> Result<Json<serde_json::Value>, ServiceHttpError>
{
    let n = scope.services.payroll.generate_month(&scope.ctx, b.month, b.year).await?;
    Ok(Json(serde_json::json!({ "generated": n })))
}

pub async fn approve(scope: TenantScope, Path((_t, id)): Path<(String, i64)>)
    -> Result<StatusCode, ServiceHttpError>
{ scope.services.payroll.approve(&scope.ctx, id).await?; Ok(StatusCode::NO_CONTENT) }

#[derive(Deserialize)] pub struct PayBody { paid_on: chrono::NaiveDate, #[serde(default)] from_bank: bool }

pub async fn pay(scope: TenantScope, Path((_t, id)): Path<(String, i64)>, Json(b): Json<PayBody>)
    -> Result<StatusCode, ServiceHttpError>
{ scope.services.payroll.pay(&scope.ctx, id, b.paid_on, b.from_bank).await?; Ok(StatusCode::NO_CONTENT) }

pub async fn list_for_staff(scope: TenantScope, Path((_t, sid, year)): Path<(String, i64, i64)>)
    -> Result<Json<Vec<Payslip>>, ServiceHttpError>
{
    scope.ctx.require_any(&[perm::PAYROLL_VIEW, perm::PAYROLL_VIEW_OWN, perm::PAYROLL_RUN])?;
    Ok(Json(scope.services.repos.payslips.for_staff(sid, year).await?))
}
