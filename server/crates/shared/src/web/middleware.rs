use crate::AppState;
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use sqlx::SqlitePool;


/// Info about the current tenant, injected into request extensions.
#[derive(Debug, Clone)]
pub struct TenantContext {
    pub slug: String,
    pub database_name: String,
    pub name: String,
}

/// Path-based tenant middleware.
///
/// For paths starting with `/t/{slug}/...` it:
///   1. Looks up the tenant in the master DB by `slug`.
///   2. Opens (or reuses) the tenant DB pool.
///   3. Injects the tenant `SqlitePool` and a `TenantContext` into request extensions.
///
/// For any other path it injects the master `SqlitePool`.
pub async fn tenant_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Response {
    let path = req.uri().path().to_string();

    if let Some(rest) = path.strip_prefix("/t/") {
        let slug = rest.split('/').next().unwrap_or("");
        if slug.is_empty() {
            return (StatusCode::BAD_REQUEST, "Missing tenant slug").into_response();
        }

        let master_pool = state.db_manager.master_pool();

        let tenant = match get_tenant_by_slug(&master_pool, slug).await {
            Ok(Some(t)) => t,
            Ok(None) => {
                return (StatusCode::NOT_FOUND, format!("Tenant '{}' not found", slug))
                    .into_response();
            }
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };

        if !tenant.is_active {
            return (StatusCode::FORBIDDEN, "Tenant is inactive").into_response();
        }

        match state.db_manager.tenant_pool(&tenant.database_name).await {
            Ok(tenant_pool) => {
                let ctx = TenantContext {
                    slug: tenant.slug.clone(),
                    database_name: tenant.database_name.clone(),
                    name: tenant.name.clone(),
                };
                req.extensions_mut().insert(tenant_pool);
                req.extensions_mut().insert(master_pool);
                req.extensions_mut().insert(ctx);
                next.run(req).await
            }
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    } else {
        let master_pool = state.db_manager.master_pool();
        req.extensions_mut().insert(master_pool);
        next.run(req).await
    }
}

async fn get_tenant_by_slug(
    pool: &SqlitePool,
    slug: &str,
) -> Result<Option<TenantRow>, sqlx::Error> {
    sqlx::query_as::<_, TenantRow>(
        "SELECT slug, name, database_name, is_active FROM tenants WHERE slug = ?",
    )
    .bind(slug)
    .fetch_optional(pool)
    .await
}

#[derive(sqlx::FromRow)]
struct TenantRow {
    slug: String,
    name: String,
    database_name: String,
    is_active: bool,
}
