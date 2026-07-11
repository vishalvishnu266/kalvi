use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::{Response, Redirect, IntoResponse},
};
use crate::config::AppState::AppState;
use crate::repository::TenantRepository::TenantRepository;
use crate::model::Tenant::Tenant;
use crate::util::SessionUtil;
use sqlx::SqlitePool;

#[derive(Clone, Debug)]
pub struct TenantContext {
    pub tenant: Tenant,
    pub pool: SqlitePool,
}

pub async fn tenant_middleware(
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let path = req.uri().path().to_string();
    
    // Check if it's a tenant path
    if path.starts_with("/t/") {
        let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        if segments.len() < 2 {
            // Redirect to common login if just /t/
            return Ok(Redirect::to("/login").into_response());
        }
        
        let slug = segments[1];
        let tenant = match TenantRepository::find_by_slug(&state.db.master_pool, slug).await {
            Ok(Some(t)) => t,
            _ => return Ok(Redirect::to("/login").into_response()), // Tenant not found
        };
            
        let pool: SqlitePool = state.db.get_tenant_pool(&tenant.database_name)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
        req.extensions_mut().insert(TenantContext {
            tenant: tenant.clone(),
            pool,
        });

        // Ensure user is redirected to login if accessing tenant path without auth
        // (Except for the login path itself)
        if !path.contains("/login") {
            let session_id = SessionUtil::get_session_id(req.headers());
            if session_id.is_none() {
                return Ok(Redirect::to(&format!("/t/{}/login", tenant.slug)).into_response());
            }
        }
    } else if path.starts_with("/onboard") || path.starts_with("/saas/") {
        // SaaS Owner paths - requires "saas_" session
        if path.starts_with("/onboard") {
             let session_id = SessionUtil::get_session_id(req.headers());
             match session_id {
                 Some(sid) if sid.starts_with("saas_") => {},
                 _ => return Ok(Redirect::to("/saas/login").into_response()),
             }
        }
    }
    
    Ok(next.run(req).await)
}
