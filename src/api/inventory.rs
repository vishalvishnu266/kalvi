use axum::{extract::{Path, Query}, http::StatusCode, Json};
use serde::Deserialize;

use crate::http::{ServiceHttpError, TenantScope};
use crate::repositories::inventory::{
    Item, NewMovement, NewPurchaseOrder, PurchaseOrder, StockMovement, Vendor,
};
use crate::services::perm;

pub async fn list_vendors(scope: TenantScope)
    -> Result<Json<Vec<Vendor>>, ServiceHttpError>
{
    scope.ctx.require_any(&[perm::INVENTORY_VIEW, perm::INVENTORY_MANAGE])?;
    Ok(Json(scope.services.repos.vendors.list().await?))
}

pub async fn create_vendor(scope: TenantScope, Json(b): Json<Vendor>)
    -> Result<Json<Vendor>, ServiceHttpError>
{
    scope.ctx.require(perm::INVENTORY_MANAGE)?;
    Ok(Json(scope.services.repos.vendors.create(&b).await?))
}

#[derive(Deserialize)] pub struct Page { #[serde(default = "d50")] limit: i64, #[serde(default)] offset: i64 }
fn d50() -> i64 { 50 }

pub async fn list_items(scope: TenantScope, Query(p): Query<Page>)
    -> Result<Json<Vec<Item>>, ServiceHttpError>
{
    scope.ctx.require_any(&[perm::INVENTORY_VIEW, perm::INVENTORY_MANAGE])?;
    Ok(Json(scope.services.repos.items.list(p.limit, p.offset).await?))
}

pub async fn create_item(scope: TenantScope, Json(b): Json<Item>)
    -> Result<Json<Item>, ServiceHttpError>
{
    scope.ctx.require(perm::INVENTORY_MANAGE)?;
    Ok(Json(scope.services.repos.items.create(&b).await?))
}

pub async fn low_stock(scope: TenantScope)
    -> Result<Json<Vec<i64>>, ServiceHttpError>
{ Ok(Json(scope.services.inventory.low_stock_alert_ids(&scope.ctx).await?)) }

pub async fn get_item(scope: TenantScope, Path((_t, id)): Path<(String, i64)>)
    -> Result<Json<Item>, ServiceHttpError>
{
    scope.ctx.require_any(&[perm::INVENTORY_VIEW, perm::INVENTORY_MANAGE])?;
    Ok(Json(scope.services.repos.items.get(id).await?))
}

pub async fn history(scope: TenantScope, Path((_t, id)): Path<(String, i64)>)
    -> Result<Json<Vec<StockMovement>>, ServiceHttpError>
{
    scope.ctx.require_any(&[perm::INVENTORY_VIEW, perm::INVENTORY_MANAGE])?;
    Ok(Json(scope.services.repos.stock.history(id, 100).await?))
}

pub async fn move_stock(scope: TenantScope, Json(b): Json<NewMovement>)
    -> Result<Json<StockMovement>, ServiceHttpError>
{
    scope.ctx.require(perm::INVENTORY_MANAGE)?;
    Ok(Json(scope.services.repos.stock.record(&b).await?))
}

pub async fn create_po(scope: TenantScope, Json(b): Json<NewPurchaseOrder>)
    -> Result<Json<PurchaseOrder>, ServiceHttpError>
{
    scope.ctx.require(perm::INVENTORY_MANAGE)?;
    Ok(Json(scope.services.repos.purchase_orders.create(&b).await?))
}

pub async fn get_po(scope: TenantScope, Path((_t, id)): Path<(String, i64)>)
    -> Result<Json<PurchaseOrder>, ServiceHttpError>
{
    scope.ctx.require_any(&[perm::INVENTORY_VIEW, perm::INVENTORY_MANAGE])?;
    Ok(Json(scope.services.repos.purchase_orders.get(id).await?))
}

#[derive(Deserialize)] pub struct Status { target: String }

pub async fn set_po_status(scope: TenantScope, Path((_t, id)): Path<(String, i64)>, Json(b): Json<Status>)
    -> Result<StatusCode, ServiceHttpError>
{ scope.services.inventory.set_po_status(&scope.ctx, id, &b.target).await?; Ok(StatusCode::NO_CONTENT) }
