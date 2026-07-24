//! Control-plane handlers: `/admin/api/tenants/*`.
//!
//! These operate on the **system DB** only, and also provision the per-tenant
//! DB (create + migrate) via [`crate::tenancy::TenantRegistry`].
//!
//! Routing lives in [`crate::http::routes`] — this file only contains the
//! handler functions.

use axum::{extract::{Path, State}, http::StatusCode, Json};

use crate::http::AppState;
use crate::system::{NewTenant, Tenant, UpdateTenant};
use crate::tenancy::TenantId;

fn err_500<E: std::fmt::Display>(e: E) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
}
fn err_400<E: std::fmt::Display>(e: E) -> (StatusCode, String) {
    (StatusCode::BAD_REQUEST, e.to_string())
}

pub async fn list(State(s): State<AppState>) -> Result<Json<Vec<Tenant>>, (StatusCode, String)> {
    s.system.list().await.map(Json).map_err(err_500)
}

pub async fn create(
    State(s): State<AppState>,
    Json(body): Json<NewTenant>,
) -> Result<Json<Tenant>, (StatusCode, String)> {
    let tenant = s.system.create(&body).await.map_err(err_400)?;
    let tid = TenantId::new(&tenant.tenant_id)
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    s.provision(tid).await.map_err(err_500)?;
    Ok(Json(tenant))
}

pub async fn get_one(
    State(s): State<AppState>,
    Path(tid): Path<String>,
) -> Result<Json<Tenant>, (StatusCode, String)> {
    match s.system.find_by_tenant_id(&tid).await.map_err(err_500)? {
        Some(t) => Ok(Json(t)),
        None    => Err((StatusCode::NOT_FOUND, "tenant not found".into())),
    }
}

pub async fn update(
    State(s): State<AppState>,
    Path(tid): Path<String>,
    Json(body): Json<UpdateTenant>,
) -> Result<Json<Tenant>, (StatusCode, String)> {
    let existing = s.system.find_by_tenant_id(&tid).await.map_err(err_500)?
        .ok_or((StatusCode::NOT_FOUND, "tenant not found".into()))?;
    let updated = s.system.update(existing.id, &body).await.map_err(err_400)?;
    if matches!(updated.status.as_str(), "disabled" | "deleted") {
        if let Ok(t) = TenantId::new(&updated.tenant_id) {
            s.evict(&t).await;
        }
    }
    Ok(Json(updated))
}

pub async fn soft_delete(
    State(s): State<AppState>,
    Path(tid): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    let existing = s.system.find_by_tenant_id(&tid).await.map_err(err_500)?
        .ok_or((StatusCode::NOT_FOUND, "tenant not found".into()))?;
    s.system.delete(existing.id).await.map_err(err_500)?;
    if let Ok(t) = TenantId::new(&existing.tenant_id) {
        s.evict(&t).await;
    }
    Ok(StatusCode::NO_CONTENT)
}

pub async fn enable(
    State(s): State<AppState>,
    Path(tid): Path<String>,
) -> Result<Json<Tenant>, (StatusCode, String)> {
    let existing = s.system.find_by_tenant_id(&tid).await.map_err(err_500)?
        .ok_or((StatusCode::NOT_FOUND, "tenant not found".into()))?;
    s.system.set_status(existing.id, "active").await.map_err(err_500)?;
    Ok(Json(s.system.get(existing.id).await.map_err(err_500)?))
}

pub async fn disable(
    State(s): State<AppState>,
    Path(tid): Path<String>,
) -> Result<Json<Tenant>, (StatusCode, String)> {
    let existing = s.system.find_by_tenant_id(&tid).await.map_err(err_500)?
        .ok_or((StatusCode::NOT_FOUND, "tenant not found".into()))?;
    s.system.set_status(existing.id, "disabled").await.map_err(err_500)?;
    if let Ok(t) = TenantId::new(&existing.tenant_id) {
        s.evict(&t).await;
    }
    Ok(Json(s.system.get(existing.id).await.map_err(err_500)?))
}
