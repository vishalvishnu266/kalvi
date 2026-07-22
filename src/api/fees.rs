//! `/api/{tenant}/fees/*` handlers.

use axum::{extract::{Path, Query}, http::StatusCode, Json};
use serde::Deserialize;

use crate::http::{ServiceHttpError, TenantScope};
use crate::repositories::fees::{
    FeeCategory, FeeDiscount, FeeInvoice, FeeInvoiceLine, FeePayment, FeeStructure,
    FeeStructureItem, NewPayment, NewStructureItem,
};

// -------- categories --------

pub async fn list_categories(scope: TenantScope)
    -> Result<Json<Vec<FeeCategory>>, ServiceHttpError>
{ Ok(Json(scope.services.repos.fee_categories.list().await?)) }

#[derive(Deserialize)] pub struct NameBody { name: String }

pub async fn create_category(scope: TenantScope, Json(b): Json<NameBody>)
    -> Result<Json<FeeCategory>, ServiceHttpError>
{ Ok(Json(scope.services.repos.fee_categories.create(&b.name).await?)) }

// -------- structures --------

#[derive(Deserialize)] pub struct NewStruct { academic_year_id: i64, grade_id: i64, name: String }

pub async fn create_structure(scope: TenantScope, Json(b): Json<NewStruct>)
    -> Result<Json<FeeStructure>, ServiceHttpError>
{ Ok(Json(scope.services.repos.fee_structures.create(b.academic_year_id, b.grade_id, &b.name).await?)) }

pub async fn list_structures(scope: TenantScope, Path((_t, yid)): Path<(String, i64)>)
    -> Result<Json<Vec<FeeStructure>>, ServiceHttpError>
{ Ok(Json(scope.services.repos.fee_structures.list_for_year(yid).await?)) }

pub async fn add_item(scope: TenantScope, Path((_t, id)): Path<(String, i64)>, Json(b): Json<NewStructureItem>)
    -> Result<Json<FeeStructureItem>, ServiceHttpError>
{ Ok(Json(scope.services.repos.fee_structures.add_item(id, &b).await?)) }

pub async fn list_items(scope: TenantScope, Path((_t, id)): Path<(String, i64)>)
    -> Result<Json<Vec<FeeStructureItem>>, ServiceHttpError>
{ Ok(Json(scope.services.repos.fee_structures.items(id).await?)) }

// -------- invoices --------

#[derive(Deserialize)]
pub struct GenerateInvoice {
    student_id: i64,
    fee_structure_id: i64,
    invoice_no: String,
    issue_date: chrono::NaiveDate,
    due_date: chrono::NaiveDate,
    tax_cents: i64,
}

pub async fn generate_invoice(scope: TenantScope, Json(b): Json<GenerateInvoice>)
    -> Result<Json<FeeInvoice>, ServiceHttpError>
{
    Ok(Json(scope.services.fees.generate_invoice_for_student(
        b.student_id, b.fee_structure_id, b.invoice_no,
        b.issue_date, b.due_date, b.tax_cents,
    ).await?))
}

pub async fn for_student(scope: TenantScope, Path((_t, sid)): Path<(String, i64)>)
    -> Result<Json<Vec<FeeInvoice>>, ServiceHttpError>
{ Ok(Json(scope.services.repos.invoices.for_student(sid).await?)) }

pub async fn get_invoice(scope: TenantScope, Path((_t, id)): Path<(String, i64)>)
    -> Result<Json<FeeInvoice>, ServiceHttpError>
{ Ok(Json(scope.services.repos.invoices.get(id).await?)) }

pub async fn get_lines(scope: TenantScope, Path((_t, id)): Path<(String, i64)>)
    -> Result<Json<Vec<FeeInvoiceLine>>, ServiceHttpError>
{ Ok(Json(scope.services.repos.invoices.lines(id).await?)) }

pub async fn cancel(scope: TenantScope, Path((_t, id)): Path<(String, i64)>)
    -> Result<StatusCode, ServiceHttpError>
{ scope.services.fees.cancel_invoice(id).await?; Ok(StatusCode::NO_CONTENT) }

pub async fn outstanding(scope: TenantScope, Path((_t, sid)): Path<(String, i64)>)
    -> Result<Json<serde_json::Value>, ServiceHttpError>
{
    let v = scope.services.fees.outstanding(sid).await?;
    Ok(Json(serde_json::json!({ "outstanding_cents": v })))
}

#[derive(Deserialize)] pub struct Today { today: chrono::NaiveDate }

pub async fn overdue(scope: TenantScope, Query(q): Query<Today>)
    -> Result<Json<Vec<FeeInvoice>>, ServiceHttpError>
{ Ok(Json(scope.services.repos.invoices.overdue(q.today).await?)) }

pub async fn aging(scope: TenantScope, Query(q): Query<Today>)
    -> Result<Json<serde_json::Value>, ServiceHttpError>
{
    let (b0, b30, b60, b90) = scope.services.fees.aging(q.today).await?;
    Ok(Json(serde_json::json!({
        "0_30": b0, "31_60": b30, "61_90": b60, "90_plus": b90,
    })))
}

// -------- payments --------

pub async fn record_payment(scope: TenantScope, Json(b): Json<NewPayment>)
    -> Result<Json<FeePayment>, ServiceHttpError>
{ Ok(Json(scope.services.fees.record_payment(b).await?)) }

pub async fn payments_for_invoice(scope: TenantScope, Path((_t, id)): Path<(String, i64)>)
    -> Result<Json<Vec<FeePayment>>, ServiceHttpError>
{ Ok(Json(scope.services.repos.payments.for_invoice(id).await?)) }

// -------- discounts --------

pub async fn grant_discount(scope: TenantScope, Json(b): Json<FeeDiscount>)
    -> Result<Json<serde_json::Value>, ServiceHttpError>
{
    let id = scope.services.repos.discounts.grant(&b).await?;
    Ok(Json(serde_json::json!({ "id": id })))
}

pub async fn discounts_for_student(scope: TenantScope, Path((_t, sid)): Path<(String, i64)>)
    -> Result<Json<Vec<FeeDiscount>>, ServiceHttpError>
{ Ok(Json(scope.services.repos.discounts.for_student(sid).await?)) }

// -------- ledger --------

#[derive(Deserialize)] pub struct AsOf { as_of: chrono::NaiveDate }

pub async fn trial_balance(scope: TenantScope, Query(q): Query<AsOf>)
    -> Result<Json<Vec<serde_json::Value>>, ServiceHttpError>
{
    let rows = scope.services.repos.ledger.trial_balance(q.as_of).await?;
    Ok(Json(rows.into_iter().map(|(code, dr, cr)|
        serde_json::json!({ "code": code, "debit_cents": dr, "credit_cents": cr })).collect()))
}
