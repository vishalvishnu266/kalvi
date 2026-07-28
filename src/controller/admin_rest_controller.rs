use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::application::AppState;
use crate::entity::tenant::{NewTenant, Tenant, UpdateTenant};
use crate::service::tenant_service;
use crate::tenant::tenant_id::validate_tenant_id;

fn err_500<E: std::fmt::Display>(e: E) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
}
fn err_400<E: std::fmt::Display>(e: E) -> (StatusCode, String) {
    (StatusCode::BAD_REQUEST, e.to_string())
}

pub async fn list(State(s): State<AppState>) -> Result<Json<Vec<Tenant>>, (StatusCode, String)> {
    tenant_service::list_all(&s.system)
        .await
        .map(Json)
        .map_err(err_500)
}

pub async fn create(
    State(s): State<AppState>,
    Json(body): Json<NewTenant>,
) -> Result<Json<Tenant>, (StatusCode, String)> {
    let tenant = tenant_service::create(&s.system, &body)
        .await
        .map_err(err_400)?;
    let tid =
        validate_tenant_id(&tenant.tenant_id).map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    s.provision(tid).await.map_err(err_500)?;
    Ok(Json(tenant))
}

pub async fn get_one(
    State(s): State<AppState>,
    Path(tid): Path<String>,
) -> Result<Json<Tenant>, (StatusCode, String)> {
    match tenant_service::find_by_tenant_id(&s.system, &tid)
        .await
        .map_err(err_500)?
    {
        Some(t) => Ok(Json(t)),
        None => Err((StatusCode::NOT_FOUND, "tenant not found".into())),
    }
}

pub async fn update(
    State(s): State<AppState>,
    Path(tid): Path<String>,
    Json(body): Json<UpdateTenant>,
) -> Result<Json<Tenant>, (StatusCode, String)> {
    let existing = tenant_service::find_by_tenant_id(&s.system, &tid)
        .await
        .map_err(err_500)?
        .ok_or((StatusCode::NOT_FOUND, "tenant not found".into()))?;
    let updated = tenant_service::update(&s.system, existing.id, &body)
        .await
        .map_err(err_400)?;
    if matches!(updated.status.as_str(), "disabled" | "deleted")
        && validate_tenant_id(&updated.tenant_id).is_ok()
    {
        s.evict(&updated.tenant_id).await;
    }
    Ok(Json(updated))
}

pub async fn soft_delete(
    State(s): State<AppState>,
    Path(tid): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    let existing = tenant_service::find_by_tenant_id(&s.system, &tid)
        .await
        .map_err(err_500)?
        .ok_or((StatusCode::NOT_FOUND, "tenant not found".into()))?;
    tenant_service::soft_delete(&s.system, existing.id)
        .await
        .map_err(err_500)?;
    if validate_tenant_id(&existing.tenant_id).is_ok() {
        s.evict(&existing.tenant_id).await;
    }
    Ok(StatusCode::NO_CONTENT)
}

pub async fn enable(
    State(s): State<AppState>,
    Path(tid): Path<String>,
) -> Result<Json<Tenant>, (StatusCode, String)> {
    let existing = tenant_service::find_by_tenant_id(&s.system, &tid)
        .await
        .map_err(err_500)?
        .ok_or((StatusCode::NOT_FOUND, "tenant not found".into()))?;
    tenant_service::set_status(&s.system, existing.id, "active")
        .await
        .map_err(err_500)?;
    Ok(Json(
        tenant_service::get(&s.system, existing.id)
            .await
            .map_err(err_500)?,
    ))
}

pub async fn disable(
    State(s): State<AppState>,
    Path(tid): Path<String>,
) -> Result<Json<Tenant>, (StatusCode, String)> {
    let existing = tenant_service::find_by_tenant_id(&s.system, &tid)
        .await
        .map_err(err_500)?
        .ok_or((StatusCode::NOT_FOUND, "tenant not found".into()))?;
    tenant_service::set_status(&s.system, existing.id, "disabled")
        .await
        .map_err(err_500)?;
    if validate_tenant_id(&existing.tenant_id).is_ok() {
        s.evict(&existing.tenant_id).await;
    }
    Ok(Json(
        tenant_service::get(&s.system, existing.id)
            .await
            .map_err(err_500)?,
    ))
}
