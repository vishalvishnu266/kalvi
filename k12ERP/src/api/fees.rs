//! `/api/tenant/fees/*`

use axum::{
    extract::{Path, Query},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;

use crate::http::{ExtractServices, ServiceHttpError};
use crate::http::middleware::TenantScopeState;
use crate::repositories::fees::{
    FeeCategory, FeeDiscount, FeeInvoice, FeeInvoiceLine, FeePayment, FeeStructure,
    FeeStructureItem, NewPayment, NewStructureItem,
};

pub fn routes() -> Router<TenantScopeState> {
    Router::new()
        .route("/categories",           get(list_categories).post(create_category))
        .route("/structures",           post(create_structure))
        .route("/structures/year/{yid}", get(list_structures))
        .route("/structures/{id}/items", post(add_item).get(list_items))
        .route("/invoices/generate",     post(generate_invoice))
        .route("/invoices/student/{sid}", get(for_student))
        .route("/invoices/{id}",         get(get_invoice))
        .route("/invoices/{id}/lines",   get(get_lines))
        .route("/invoices/{id}/cancel",  post(cancel))
        .route("/outstanding/{sid}",     get(outstanding))
        .route("/overdue",               get(overdue))
        .route("/aging",                 get(aging))
        .route("/payments",              post(record_payment))
        .route("/payments/invoice/{id}", get(payments_for_invoice))
        .route("/discounts",             post(grant_discount))
        .route("/discounts/student/{sid}", get(discounts_for_student))
        .route("/ledger/trial-balance",  get(trial_balance))
}

// --- Categories ---
async fn list_categories(ExtractServices(a): ExtractServices)
    -> Result<Json<Vec<FeeCategory>>, ServiceHttpError>
{ Ok(Json(a.repos.fee_categories.list().await.map_err(re)?)) }

#[derive(Deserialize)] struct NameBody { name: String }

async fn create_category(ExtractServices(a): ExtractServices, Json(b): Json<NameBody>)
    -> Result<Json<FeeCategory>, ServiceHttpError>
{ Ok(Json(a.repos.fee_categories.create(&b.name).await.map_err(re)?)) }

// --- Structures ---
#[derive(Deserialize)] struct NewStruct { academic_year_id: i64, grade_id: i64, name: String }

async fn create_structure(ExtractServices(a): ExtractServices, Json(b): Json<NewStruct>)
    -> Result<Json<FeeStructure>, ServiceHttpError>
{ Ok(Json(a.repos.fee_structures.create(b.academic_year_id, b.grade_id, &b.name).await.map_err(re)?)) }

async fn list_structures(ExtractServices(a): ExtractServices, Path(yid): Path<i64>)
    -> Result<Json<Vec<FeeStructure>>, ServiceHttpError>
{ Ok(Json(a.repos.fee_structures.list_for_year(yid).await.map_err(re)?)) }

async fn add_item(ExtractServices(a): ExtractServices, Path(id): Path<i64>, Json(b): Json<NewStructureItem>)
    -> Result<Json<FeeStructureItem>, ServiceHttpError>
{ Ok(Json(a.repos.fee_structures.add_item(id, &b).await.map_err(re)?)) }

async fn list_items(ExtractServices(a): ExtractServices, Path(id): Path<i64>)
    -> Result<Json<Vec<FeeStructureItem>>, ServiceHttpError>
{ Ok(Json(a.repos.fee_structures.items(id).await.map_err(re)?)) }

// --- Invoices ---
#[derive(Deserialize)]
struct GenerateInvoice {
    student_id: i64,
    fee_structure_id: i64,
    invoice_no: String,
    issue_date: chrono::NaiveDate,
    due_date: chrono::NaiveDate,
    tax_cents: i64,
}

async fn generate_invoice(ExtractServices(a): ExtractServices, Json(b): Json<GenerateInvoice>)
    -> Result<Json<FeeInvoice>, ServiceHttpError>
{
    Ok(Json(a.fees.generate_invoice_for_student(
        b.student_id, b.fee_structure_id, b.invoice_no,
        b.issue_date, b.due_date, b.tax_cents,
    ).await?))
}

async fn for_student(ExtractServices(a): ExtractServices, Path(sid): Path<i64>)
    -> Result<Json<Vec<FeeInvoice>>, ServiceHttpError>
{ Ok(Json(a.repos.invoices.for_student(sid).await.map_err(re)?)) }

async fn get_invoice(ExtractServices(a): ExtractServices, Path(id): Path<i64>)
    -> Result<Json<FeeInvoice>, ServiceHttpError>
{ Ok(Json(a.repos.invoices.get(id).await.map_err(re)?)) }

async fn get_lines(ExtractServices(a): ExtractServices, Path(id): Path<i64>)
    -> Result<Json<Vec<FeeInvoiceLine>>, ServiceHttpError>
{ Ok(Json(a.repos.invoices.lines(id).await.map_err(re)?)) }

async fn cancel(ExtractServices(a): ExtractServices, Path(id): Path<i64>)
    -> Result<axum::http::StatusCode, ServiceHttpError>
{ a.fees.cancel_invoice(id).await?; Ok(axum::http::StatusCode::NO_CONTENT) }

async fn outstanding(ExtractServices(a): ExtractServices, Path(sid): Path<i64>)
    -> Result<Json<serde_json::Value>, ServiceHttpError>
{
    let v = a.fees.outstanding(sid).await?;
    Ok(Json(serde_json::json!({ "outstanding_cents": v })))
}

#[derive(Deserialize)] struct Today { today: chrono::NaiveDate }

async fn overdue(ExtractServices(a): ExtractServices, Query(q): Query<Today>)
    -> Result<Json<Vec<FeeInvoice>>, ServiceHttpError>
{ Ok(Json(a.repos.invoices.overdue(q.today).await.map_err(re)?)) }

async fn aging(ExtractServices(a): ExtractServices, Query(q): Query<Today>)
    -> Result<Json<serde_json::Value>, ServiceHttpError>
{
    let (b0, b30, b60, b90) = a.fees.aging(q.today).await?;
    Ok(Json(serde_json::json!({
        "0_30": b0, "31_60": b30, "61_90": b60, "90_plus": b90,
    })))
}

// --- Payments ---
async fn record_payment(ExtractServices(a): ExtractServices, Json(b): Json<NewPayment>)
    -> Result<Json<FeePayment>, ServiceHttpError>
{ Ok(Json(a.fees.record_payment(b).await?)) }

async fn payments_for_invoice(ExtractServices(a): ExtractServices, Path(id): Path<i64>)
    -> Result<Json<Vec<FeePayment>>, ServiceHttpError>
{ Ok(Json(a.repos.payments.for_invoice(id).await.map_err(re)?)) }

// --- Discounts ---
async fn grant_discount(ExtractServices(a): ExtractServices, Json(b): Json<FeeDiscount>)
    -> Result<Json<serde_json::Value>, ServiceHttpError>
{
    let id = a.repos.discounts.grant(&b).await.map_err(re)?;
    Ok(Json(serde_json::json!({ "id": id })))
}

async fn discounts_for_student(ExtractServices(a): ExtractServices, Path(sid): Path<i64>)
    -> Result<Json<Vec<FeeDiscount>>, ServiceHttpError>
{ Ok(Json(a.repos.discounts.for_student(sid).await.map_err(re)?)) }

// --- Ledger ---
#[derive(Deserialize)] struct AsOf { as_of: chrono::NaiveDate }

async fn trial_balance(ExtractServices(a): ExtractServices, Query(q): Query<AsOf>)
    -> Result<Json<Vec<serde_json::Value>>, ServiceHttpError>
{
    let rows = a.repos.ledger.trial_balance(q.as_of).await.map_err(re)?;
    Ok(Json(rows.into_iter().map(|(code, dr, cr)|
        serde_json::json!({ "code": code, "debit_cents": dr, "credit_cents": cr })).collect()))
}

fn re(e: crate::error::RepoError) -> ServiceHttpError { ServiceHttpError(e.into()) }
