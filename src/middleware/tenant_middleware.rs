use axum::{
    body::Body,
    extract::State,
    http::{Request, Response},
    middleware::Next,
    response::Redirect,
};
use sqlx::SqlitePool;
use crate::config::AppState;
use crate::model::Tenant;
use crate::service::TenantService;
use crate::util::AppError;

#[derive(Clone, Debug)]
pub struct TenantContext {
    pub tenant: Tenant,
    pub pool: SqlitePool,
}

impl TenantContext {
    /// Utility to retrieve tenant context from request extensions in controllers
    pub fn from_req(req: &Request<Body>) -> Result<Self, AppError> {
        req.extensions().get::<Self>()
            .cloned()
            .ok_or_else(|| AppError::RuntimeException("Tenant context missing".to_string()))
    }
}

pub async fn tenant_middleware(
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    // 1. Parse segments (e.g. /web/city-high/dashboard -> segments: ["web", "city-high", "dashboard"])
    let segments: Vec<String> = req.uri().path()
        .split('/')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();
    
    // We expect /web/{tenant}/...
    let (prefix, slug) = match (segments.get(0), segments.get(1)) {
        (Some(p), Some(s)) if p == "web" => (p, s),
        _ => return Ok(next.run(req).await),
    };

    // 2. Resolve Tenant from Master DB
    let tenant = match TenantService::find_by_slug(&state, slug).await? {
        Some(t) => t,
        None => return Ok(Redirect::to("/login").into_response()),
    };
        
    // 3. Resolve Isolated Database Pool
    let pool = state.db.get_tenant_pool(&tenant.database_name).await?;
    
    let ctx = TenantContext {
        tenant,
        pool,
    };
    
    // 4. Attach to request extensions
    req.extensions_mut().insert(ctx);

    Ok(next.run(req).await)
}
