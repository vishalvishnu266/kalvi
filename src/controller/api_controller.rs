use axum::{
    extract::Extension,
    Json,
};
use serde_json::{json, Value};
use crate::middleware::TenantContext;

pub async fn health(Extension(ctx): Extension<TenantContext>) -> Json<Value> {
    Json(json!({
        "status": "up",
        "tenant": ctx.tenant.name,
        "database": ctx.tenant.database_name,
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}
