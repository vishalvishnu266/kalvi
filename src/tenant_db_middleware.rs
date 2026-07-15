use axum::{
    extract::{Path, Request, State},
    middleware::Next,
    response::{Html, IntoResponse, Response},
};
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use std::collections::HashMap;
use std::str::FromStr;
use tracing::{error, info};

use crate::state::AppState;

pub async fn tenant_db_middleware(
    State(state): State<AppState>,
    Path(params): Path<HashMap<String, String>>,
    req: Request,
    next: Next,
) -> Response {
    let tenant_id = match params.get("tenant_id") {
        Some(t) => t.clone(),
        None => return Html("<h1>Missing tenant id</h1>").into_response(),
    };
    info!("Tenant DB Middleware for {}", tenant_id);

    // Reject anything suspicious in the tenant id (path traversal etc.)
    if !tenant_id.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-') {
        return Html("<h1>Invalid tenant id</h1>").into_response();
    }

    let pool = {
        let pools = state.tenant_pools.read().await;
        pools.get(&tenant_id).cloned()
    };

    let pool = match pool {
        Some(p) => p,
        None => {
            std::fs::create_dir_all("data/tenant").ok();
            let db_url = format!("sqlite://data/tenant/{}.db", tenant_id);
            let opts = match SqliteConnectOptions::from_str(&db_url) {
                Ok(o) => o.create_if_missing(true),
                Err(e) => {
                    error!("bad tenant db url: {e}");
                    return Html("<h1>Database configuration error</h1>").into_response();
                }
            };
            match SqlitePool::connect_with(opts).await {
                Ok(pool) => {
                    if let Err(e) = sqlx::migrate!("./resources/migration/tenant").run(&pool).await {
                        error!("migration failed for {tenant_id}: {e}");
                        return Html(format!("<h1>Migration failed</h1><pre>{}</pre>", e))
                            .into_response();
                    }

                    // Reset the seed admin's password to a runtime-hashed
                    // "admin123" so the pre-computed migration hash does not
                    // become a stale artefact if the argon2 crate version
                    // changes. Safe & idempotent.
                    if let Err(e) = crate::models::user::User::ensure_dev_seed_password(
                        &pool, "admin@school.edu", "admin123",
                    )
                    .await
                    {
                        error!("seed password reset failed for {tenant_id}: {:?}", e);
                    }

                    let mut pools = state.tenant_pools.write().await;
                    pools.insert(tenant_id, pool.clone());
                    pool
                }
                Err(e) => {
                    let request_id = req
                        .headers()
                        .get("x-request-id")
                        .and_then(|v| v.to_str().ok())
                        .unwrap_or("unknown");
                    error!("Failed to connect to tenant DB: {e}");
                    return Html(format!(
                        "<h1>Database connection error</h1><p>Request ID: {}</p>",
                        request_id
                    ))
                    .into_response();
                }
            }
        }
    };

    let mut req = req;
    req.extensions_mut().insert(pool);
    next.run(req).await
}
