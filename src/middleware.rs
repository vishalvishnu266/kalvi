use axum::{
    extract::{State, Request},
    middleware::Next,
    response::{Response, IntoResponse},
    http::StatusCode,
};
use crate::state::AppState;
use sqlx::SqlitePool;
use tracing::error;

pub async fn tenant_db_middleware(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Response {
    let path = req.uri().path();
    let components: Vec<&str> = path.split('/').collect();
    let tenant_id = components.get(2);

    if let Some(tenant_id) = tenant_id {
        let tenant_id = tenant_id.to_string();
        
        let pool = {
            let pools = state.tenant_pools.read().await;
            pools.get(&tenant_id).cloned()
        };

        let pool = match pool {
            Some(p) => p,
            None => {
                let db_url = format!("sqlite://data/tenant/{}.db", tenant_id);
                match SqlitePool::connect(&db_url).await {
                    Ok(pool) => {
                        let mut pools = state.tenant_pools.write().await;
                        pools.insert(tenant_id, pool.clone());
                        pool
                    }
                    Err(e) => {
                        let request_id = req.headers().get("x-request-id")
                            .and_then(|v| v.to_str().ok())
                            .unwrap_or("unknown");
                        error!("Failed to connect to database: {} (Request ID: {})", e, request_id);
                        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                    }
                }
            }
        };

        let mut req = req;
        req.extensions_mut().insert(pool);
        next.run(req).await
    } else {
        next.run(req).await
    }
}
