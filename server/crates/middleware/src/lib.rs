use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
};
use repository::TenantDatabaseManager;
use std::sync::Arc;


#[derive(Clone)]
pub struct AppState {
    pub db_manager: Arc<TenantDatabaseManager>,
}

pub async fn tenant_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Response {
    let tenant_id = req.headers().get("X-Tenant-ID").and_then(|h| h.to_str().ok());
    
    match state.db_manager.get_pool(tenant_id).await {
        Ok(pool) => {
            req.extensions_mut().insert(pool);
            next.run(req).await
        },
        Err(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
