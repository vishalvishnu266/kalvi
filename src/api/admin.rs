//! Control-plane endpoints: `/api/admin/tenants/*`.
//!
//! These operate on the **system DB** only, and also provision the per-tenant
//! DB (create + migrate) via [`TenantRegistry`].

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};

use crate::api::AppState;
use crate::system::{NewTenant, Tenant, UpdateTenant};
use crate::tenancy::TenantId;

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/tenants",              get(list).post(create))
        .route("/tenants/{tenant_id}",   get(get_one).put(update).delete(soft_delete))
        .route("/tenants/{tenant_id}/enable",  post(enable))
        .route("/tenants/{tenant_id}/disable", post(disable))
        .with_state(state)
}

// --- Handlers ---

async fn list(State(s): State<AppState>) -> Result<Json<Vec<Tenant>>, (StatusCode, String)> {
    s.system.list().await
        .map(Json)
        .map_err(err_500)
}

async fn create(
    State(s): State<AppState>,
    Json(body): Json<NewTenant>,
) -> Result<Json<Tenant>, (StatusCode, String)> {
    // Insert control-plane row
    let tenant = s.system.create(&body).await.map_err(err_400)?;

    // Provision the per-tenant DB (create file + migrate).
    let tid = TenantId::new(&tenant.tenant_id).map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    s.tenants.provision(tid).await.map_err(err_500)?;

    Ok(Json(tenant))
}

async fn get_one(
    State(s): State<AppState>,
    Path(tid): Path<String>,
) -> Result<Json<Tenant>, (StatusCode, String)> {
    match s.system.find_by_tenant_id(&tid).await.map_err(err_500)? {
        Some(t) => Ok(Json(t)),
        None    => Err((StatusCode::NOT_FOUND, "tenant not found".into())),
    }
}

async fn update(
    State(s): State<AppState>,
    Path(tid): Path<String>,
    Json(body): Json<UpdateTenant>,
) -> Result<Json<Tenant>, (StatusCode, String)> {
    let existing = s.system.find_by_tenant_id(&tid).await.map_err(err_500)?
        .ok_or((StatusCode::NOT_FOUND, "tenant not found".into()))?;
    let updated = s.system.update(existing.id, &body).await.map_err(err_400)?;

    // If disabled/deleted, evict the cached pool so open connections drain.
    if matches!(updated.status.as_str(), "disabled" | "deleted") {
        if let Ok(t) = TenantId::new(&updated.tenant_id) {
            s.tenants.evict(&t).await;
        }
    }
    Ok(Json(updated))
}

async fn soft_delete(
    State(s): State<AppState>,
    Path(tid): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    let existing = s.system.find_by_tenant_id(&tid).await.map_err(err_500)?
        .ok_or((StatusCode::NOT_FOUND, "tenant not found".into()))?;
    s.system.delete(existing.id).await.map_err(err_500)?;
    if let Ok(t) = TenantId::new(&existing.tenant_id) {
        s.tenants.evict(&t).await;
    }
    Ok(StatusCode::NO_CONTENT)
}

async fn enable(
    State(s): State<AppState>,
    Path(tid): Path<String>,
) -> Result<Json<Tenant>, (StatusCode, String)> {
    let existing = s.system.find_by_tenant_id(&tid).await.map_err(err_500)?
        .ok_or((StatusCode::NOT_FOUND, "tenant not found".into()))?;
    s.system.set_status(existing.id, "active").await.map_err(err_500)?;
    Ok(Json(s.system.get(existing.id).await.map_err(err_500)?))
}

async fn disable(
    State(s): State<AppState>,
    Path(tid): Path<String>,
) -> Result<Json<Tenant>, (StatusCode, String)> {
    let existing = s.system.find_by_tenant_id(&tid).await.map_err(err_500)?
        .ok_or((StatusCode::NOT_FOUND, "tenant not found".into()))?;
    s.system.set_status(existing.id, "disabled").await.map_err(err_500)?;
    if let Ok(t) = TenantId::new(&existing.tenant_id) {
        s.tenants.evict(&t).await;
    }
    Ok(Json(s.system.get(existing.id).await.map_err(err_500)?))
}

// --- helpers ---

fn err_500<E: std::fmt::Display>(e: E) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
}
fn err_400<E: std::fmt::Display>(e: E) -> (StatusCode, String) {
    (StatusCode::BAD_REQUEST, e.to_string())
}
