use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use sqlx::SqlitePool;
use crate::config::AppState;
use crate::repository::UserRepository;
use crate::util::SessionUtil;
use crate::model::Tenant;
use crate::service::TenantService;

pub async fn tenant_middleware(
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let path = req.uri().path().to_string();
    let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    
    // This middleware is only called for /web/{slug}/... and /api/{slug}/...
    // so we can safely assume the structure.
    let prefix = segments.get(0).copied().unwrap_or("");
    let slug = match segments.get(1) {
        Some(s) => s,
        None => return Ok(next.run(req).await), // Should not happen with current routing
    };

    let tenant = match TenantService::find_by_slug(&state, slug).await {
        Ok(Some(t)) => t,
        _ => return Ok(Redirect::to("/login").into_response()),
    };
        
    let pool = state.db.get_tenant_pool(&tenant.database_name)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
    let ctx = TenantContext {
        tenant: tenant.clone(),
        pool,
    };
    
    req.extensions_mut().insert(ctx.clone());

    // Handle tenant root redirect (/web/{slug} or /web/{slug}/)
    if prefix == "web" && segments.len() == 2 {
        let session_id = SessionUtil::get_session_id(req.headers());
        if session_id.is_some() {
            return Ok(Redirect::to(&format!("/web/{}/dashboard", slug)).into_response());
        } else {
            return Ok(Redirect::to(&format!("/web/{}/login", slug)).into_response());
        }
    }
    
    Ok(next.run(req).await)
}

#[derive(Clone, Debug)]
pub struct TenantContext {
    pub tenant: Tenant,
    pub pool: SqlitePool,
}