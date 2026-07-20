//! `/api/tenant/inventory/*`

use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;

use crate::http::{ExtractServices, ServiceHttpError};
use crate::http::middleware::TenantScopeState;
use crate::repositories::inventory::{
    Item, NewMovement, NewPurchaseOrder, PurchaseOrder, StockMovement, Vendor,
};

pub fn routes() -> Router<TenantScopeState> {
    Router::new()
        .route("/vendors",              get(list_vendors).post(create_vendor))
        .route("/items",                get(list_items).post(create_item))
        .route("/items/low-stock",      get(low_stock))
        .route("/items/{id}",            get(get_item))
        .route("/items/{id}/history",    get(history))
        .route("/movements",            post(move_stock))
        .route("/purchase-orders",      post(create_po))
        .route("/purchase-orders/{id}",  get(get_po))
        .route("/purchase-orders/{id}/status", post(set_po_status))
}

async fn list_vendors(ExtractServices(a): ExtractServices)
    -> Result<Json<Vec<Vendor>>, ServiceHttpError>
{ Ok(Json(a.repos.vendors.list().await.map_err(re)?)) }

async fn create_vendor(ExtractServices(a): ExtractServices, Json(b): Json<Vendor>)
    -> Result<Json<Vendor>, ServiceHttpError>
{ Ok(Json(a.repos.vendors.create(&b).await.map_err(re)?)) }

#[derive(Deserialize)] struct Page { #[serde(default="d50")] limit: i64, #[serde(default)] offset: i64 }
fn d50() -> i64 { 50 }

async fn list_items(ExtractServices(a): ExtractServices, axum::extract::Query(p): axum::extract::Query<Page>)
    -> Result<Json<Vec<Item>>, ServiceHttpError>
{ Ok(Json(a.repos.items.list(p.limit, p.offset).await.map_err(re)?)) }

async fn create_item(ExtractServices(a): ExtractServices, Json(b): Json<Item>)
    -> Result<Json<Item>, ServiceHttpError>
{ Ok(Json(a.repos.items.create(&b).await.map_err(re)?)) }

async fn low_stock(ExtractServices(a): ExtractServices)
    -> Result<Json<Vec<i64>>, ServiceHttpError>
{ Ok(Json(a.inventory.low_stock_alert_ids().await?)) }

async fn get_item(ExtractServices(a): ExtractServices, Path(id): Path<i64>)
    -> Result<Json<Item>, ServiceHttpError>
{ Ok(Json(a.repos.items.get(id).await.map_err(re)?)) }

async fn history(ExtractServices(a): ExtractServices, Path(id): Path<i64>)
    -> Result<Json<Vec<StockMovement>>, ServiceHttpError>
{ Ok(Json(a.repos.stock.history(id, 100).await.map_err(re)?)) }

async fn move_stock(ExtractServices(a): ExtractServices, Json(b): Json<NewMovement>)
    -> Result<Json<StockMovement>, ServiceHttpError>
{ Ok(Json(a.repos.stock.record(&b).await.map_err(re)?)) }

async fn create_po(ExtractServices(a): ExtractServices, Json(b): Json<NewPurchaseOrder>)
    -> Result<Json<PurchaseOrder>, ServiceHttpError>
{ Ok(Json(a.repos.purchase_orders.create(&b).await.map_err(re)?)) }

async fn get_po(ExtractServices(a): ExtractServices, Path(id): Path<i64>)
    -> Result<Json<PurchaseOrder>, ServiceHttpError>
{ Ok(Json(a.repos.purchase_orders.get(id).await.map_err(re)?)) }

#[derive(Deserialize)] struct Status { target: String }

async fn set_po_status(ExtractServices(a): ExtractServices, Path(id): Path<i64>, Json(b): Json<Status>)
    -> Result<axum::http::StatusCode, ServiceHttpError>
{ a.inventory.set_po_status(id, &b.target).await?; Ok(axum::http::StatusCode::NO_CONTENT) }

fn re(e: crate::error::RepoError) -> ServiceHttpError { ServiceHttpError(e.into()) }
