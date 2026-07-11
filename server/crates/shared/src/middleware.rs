use crate::TenantDatabaseManager;
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
    http::StatusCode,
};
use std::sync::Arc;
use sqlx::SqlitePool;

/// Application state that can be used across all domains
#[derive(Clone)]
pub struct AppState {
    pub db_manager: Arc<TenantDatabaseManager>,
}

/// Tenant context extracted from path
#[derive(Debug, Clone)]
pub struct TenantContext {
    pub slug: String,
    pub database_name: String,
}

/// Path-based tenant middleware - extracts tenant slug from URL path
/// 
/// This middleware:
/// 1. Checks if path starts with /t/{tenant_slug}
/// 2. Looks up tenant in master database
/// 3. Gets tenant-specific database pool
/// 4. Injects both master pool and tenant pool into request extensions
/// 
/// For non-tenant routes (like /onboard), only master pool is injected
/// 
/// Usage in handlers:
/// ```
/// async fn handler(Extension(pool): Extension<SqlitePool>) {
///     // This is the tenant pool for /t/{slug} routes
///     // Or master pool for non-tenant routes
/// }
/// ```
pub async fn tenant_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Response {
    let path = req.uri().path();
    
    // Check if this is a tenant-specific route (/t/{slug}/...)
    if path.starts_with("/t/") {
        // Extract tenant slug from path
        let parts: Vec<&str> = path.split('/').collect();
        if parts.len() < 3 {
            return StatusCode::BAD_REQUEST.into_response();
        }
        
        let tenant_slug = parts[2];
        
        // Look up tenant in master database
        let master_pool = state.db_manager.get_master_pool();
        
        let tenant = match get_tenant_by_slug(&master_pool, tenant_slug).await {
            Ok(Some(tenant)) => tenant,
            Ok(None) => {
                return (StatusCode::NOT_FOUND, format!("Tenant '{}' not found", tenant_slug))
                    .into_response();
            }
            Err(_) => {
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        };
        
        if !tenant.is_active {
            return (StatusCode::FORBIDDEN, "Tenant is inactive")
                .into_response();
        }
        
        // Get tenant-specific database pool
        match state.db_manager.get_tenant_pool(&tenant.database_name).await {
            Ok(tenant_pool) => {
                // Inject both pools into request
                req.extensions_mut().insert(tenant_pool.clone());
                req.extensions_mut().insert(master_pool);
                
                // Store tenant context
                let context = TenantContext {
                    slug: tenant.slug.clone(),
                    database_name: tenant.database_name.clone(),
                };
                req.extensions_mut().insert(context);
                
                next.run(req).await
            }
            Err(_) => {
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    } else {
        // Non-tenant route - only inject master pool
        let master_pool = state.db_manager.get_master_pool();
        req.extensions_mut().insert(master_pool);
        next.run(req).await
    }
}

// Helper to look up tenant by slug
async fn get_tenant_by_slug(
    pool: &SqlitePool, 
    slug: &str
) -> Result<Option<TenantRecord>, sqlx::Error> {
    sqlx::query_as::<_, TenantRecord>("SELECT * FROM tenants WHERE slug = ?")
        .bind(slug)
        .fetch_optional(pool)
        .await
}

/// Only the fields actively used by request routing are read here; the rest
/// are included so the row can be deserialized by sqlx from `SELECT *`.
#[derive(sqlx::FromRow)]
#[allow(dead_code)]
struct TenantRecord {
    id: i64,
    slug: String,
    name: String,
    contact_email: String,
    contact_phone: String,
    address: String,
    database_name: String,
    is_active: bool,
    created_at: String,
}
