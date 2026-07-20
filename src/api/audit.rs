//! `/api/tenant/audit/*` — read-only audit trail queries.

use axum::{
    extract::{Path, Query},
    routing::get,
    Json, Router,
};
use serde::Deserialize;

use crate::http::{ExtractServices, ServiceHttpError};
use crate::http::middleware::TenantScopeState;
use crate::repositories::audit::AuditEntry;

pub fn routes() -> Router<TenantScopeState> {
    Router::new()
        .route("/entity/{entity}/{id}", get(for_entity))
        .route("/user/{uid}",          get(for_user))
}

#[derive(Deserialize)] struct Limit { #[serde(default = "d100")] limit: i64 }
fn d100() -> i64 { 100 }

async fn for_entity(
    ExtractServices(a): ExtractServices,
    Path((entity, id)): Path<(String, i64)>,
    Query(q): Query<Limit>,
) -> Result<Json<Vec<AuditEntry>>, ServiceHttpError>
{ Ok(Json(a.repos.audit_log.for_entity(&entity, id, q.limit).await.map_err(|e| ServiceHttpError(e.into()))?)) }

async fn for_user(
    ExtractServices(a): ExtractServices,
    Path(uid): Path<i64>,
    Query(q): Query<Limit>,
) -> Result<Json<Vec<AuditEntry>>, ServiceHttpError>
{ Ok(Json(a.repos.audit_log.for_user(uid, q.limit).await.map_err(|e| ServiceHttpError(e.into()))?)) }
