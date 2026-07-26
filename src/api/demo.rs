//! Demo JSON API handlers.
//!
//! Illustrates the axum handler shape every new module should adopt:
//!   * Extract [`TenantScope`] to get the tenant pool + `RequestCtx`.
//!   * Enforce permissions via `scope.ctx.require(...)` (or do it in
//!     the service, as `services::demo::create_message` does).
//!   * Return `Json<T>` on success, `ServiceHttpError` on failure.

use axum::Json;
use serde_json::json;

use crate::http::{ServiceHttpError, TenantScope};
use crate::models::demo::{DemoMessage, NewDemoMessage};
use crate::services::demo as demo_svc;
use crate::services::perm;

/// Simple liveness-style ping that also proves the tenant pool works.
pub async fn ping(scope: TenantScope) -> Json<serde_json::Value> {
    Json(json!({
        "status":     "ok",
        "tenant":     scope.tenant.as_str(),
        "request_id": scope.ctx.request_id,
    }))
}

pub async fn list(scope: TenantScope) -> Result<Json<Vec<DemoMessage>>, ServiceHttpError> {
    scope.ctx.require(perm::DEMO_VIEW)?;
    Ok(Json(demo_svc::list_messages(&scope.pool, 50).await?))
}

pub async fn echo(
    scope: TenantScope,
    Json(body): Json<NewDemoMessage>,
) -> Result<Json<DemoMessage>, ServiceHttpError> {
    let row = demo_svc::create_message(&scope.pool, &scope.ctx, body).await?;
    Ok(Json(row))
}
