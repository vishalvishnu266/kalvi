use axum::Json;
use serde_json::json;

use crate::entity::demo_message::{DemoMessage, NewDemoMessage};
use crate::exception::service_error::ServiceError;
use crate::security::permissions;
use crate::security::tenant_scope::TenantScope;
use crate::service::demo_service;

pub async fn ping(scope: TenantScope) -> Json<serde_json::Value> {
    Json(json!({
        "status":     "ok",
        "tenant":     scope.tenant.as_str(),
        "request_id": scope.ctx.request_id,
    }))
}

pub async fn list(scope: TenantScope) -> Result<Json<Vec<DemoMessage>>, ServiceError> {
    scope.ctx.require(permissions::DEMO_VIEW)?;
    Ok(Json(demo_service::list_messages(&scope.pool, 50).await?))
}

pub async fn echo(
    scope: TenantScope,
    Json(body): Json<NewDemoMessage>,
) -> Result<Json<DemoMessage>, ServiceError> {
    let row = demo_service::create_message(&scope.pool, &scope.ctx, body).await?;
    Ok(Json(row))
}
