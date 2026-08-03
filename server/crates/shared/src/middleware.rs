use crate::TenantDatabaseManager;
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::sync::Arc;

/// Application state that can be used across all domains
#[derive(Clone)]
pub struct AppState {
    pub db_manager: Arc<TenantDatabaseManager>,
}

/// Tenant middleware - extracts tenant ID from header and provides database pool
/// 
/// This middleware:
/// 1. Reads X-Tenant-ID header (optional)
/// 2. Gets appropriate database pool for tenant
/// 3. Injects pool into request extensions
/// 
/// Usage in handlers:
/// ```
/// async fn handler(Extension(pool): Extension<SqlitePool>) {
///     // Use pool here
/// }
/// ```
pub async fn tenant_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Response {
    // Extract tenant ID from header (optional)
    let tenant_id = req
        .headers()
        .get("X-Tenant-ID")
        .and_then(|h| h.to_str().ok());
    
    // Get database pool for this tenant
    match state.db_manager.get_pool(tenant_id).await {
        Ok(pool) => {
            // Inject pool into request for handlers to use
            req.extensions_mut().insert(pool);
            next.run(req).await
        }
        Err(_) => {
            axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
