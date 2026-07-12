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
    
    // Reserved public/control-plane paths
    let reserved = ["/login", "/registration", "/pages", "/saas", "/onboard", "/favicon.ico", "/static"];
    let is_reserved = reserved.iter().any(|r| path.starts_with(r)) || path == "/";

    if !is_reserved {
        let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        if let Some(slug) = segments.first() {
            let tenant = match TenantRepository::find_by_slug(&state.db.master_pool, slug).await {
                Ok(Some(t)) => t,
                _ => return Ok(next.run(req).await), // Let it fall through to 404 or other handlers
            };
                
            let pool: SqlitePool = state.db.get_tenant_pool(&tenant.database_name)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                
            req.extensions_mut().insert(TenantContext {
                tenant: tenant.clone(),
                pool,
            });

            // If accessing tenant root /{slug}, ensure we handle redirect to dashboard or login
            // Note: Auth check for sub-routes like /dashboard is handled by auth_middleware
            if path == format!("/{}", slug) || path == format!("/{}/", slug) {
                let session_id = SessionUtil::get_session_id(req.headers());
                if session_id.is_some() {
                    return Ok(Redirect::to(&format!("/{}/dashboard", slug)).into_response());
                } else {
                    return Ok(Redirect::to(&format!("/{}/login", slug)).into_response());
                }
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
