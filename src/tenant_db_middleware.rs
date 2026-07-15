use axum::{
    extract::{State, Request, Path},
    middleware::Next,
    response::{Response, IntoResponse, Html},
};
use crate::state::AppState;
use sqlx::SqlitePool;
use tracing::error;
use tracing::log::info;

pub async fn tenant_db_middleware(
    State(state): State<AppState>,
    Path(tenant_id): Path<String>, // Axum extracts it automatically
    req: Request,
    next: Next,
) -> Response {
    info!("Tenant DB Middleware");
    let tenant_id = tenant_id;
    
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
                    sqlx::migrate!("./resources/migration/tenant")
                        .run(&pool)
                        .await
                        .expect("Failed to run migrations");
                    let mut pools = state.tenant_pools.write().await;
                    pools.insert(tenant_id, pool.clone());
                    pool
                }
                Err(e) => {
                    let request_id = req.headers().get("x-request-id")
                        .and_then(|v| v.to_str().ok())
                        .unwrap_or("unknown");
                    error!("Failed to connect to database: {} ", e);
                    return Html(format!("<h1>Database connection error</h1><p>Request ID: {}</p>", request_id)).into_response();
                }
            }
        }
    };

    let mut req = req;
    req.extensions_mut().insert(pool);
    next.run(req).await
}
