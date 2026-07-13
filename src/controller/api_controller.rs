use axum::{
    extract::Extension,
    Json,
};
use serde_json::{json, Value};
use crate::middleware::TenantContext;
use crate::util::AppError;

pub async fn health(Extension(ctx): Extension<TenantContext>) -> Result<Json<Value>, AppError> {
    Ok(Json(json!({
        "status": "up",
        "tenant": ctx.tenant.name,
        "database": ctx.tenant.database_name,
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}
