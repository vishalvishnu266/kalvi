use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use sqlx::SqlitePool;
use crate::config::AppState::AppState;
use crate::repository::TenantRepository::TenantRepository;
use crate::util::SessionUtil;
use crate::repository::UserRepository::UserRepository;
use crate::model::Tenant::Tenant;

pub async fn app_middleware(
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let path = req.uri().path().to_string();
    
    // --- Tenant Middleware Logic ---
    let mut tenant_ctx: Option<TenantContext> = None;
    
    // Reserved public/control-plane paths
    let reserved = ["/login", "/registration", "/pages", "/favicon.ico", "/static"];
    let is_reserved = reserved.iter().any(|r| path.starts_with(r)) || path == "/";

    if !is_reserved {
        let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        if let Some(slug) = segments.first() {
            let tenant = match TenantRepository::find_by_slug(&state.db.master_pool, slug).await {
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
            tenant_ctx = Some(ctx);

            // If accessing tenant root /{slug}, ensure we handle redirect to dashboard or login
            if path == format!("/{}", slug) || path == format!("/{}/", slug) {
                let session_id = SessionUtil::get_session_id(req.headers());
                if session_id.is_some() {
                    return Ok(Redirect::to(&format!("/{}/dashboard", slug)).into_response());
                } else {
                    return Ok(Redirect::to(&format!("/{}/login", slug)).into_response());
                }
            }
        }
    }
    
    // --- Auth Middleware Logic ---
    let needs_auth = path.contains("/dashboard") || path.contains("/settings") || path.contains("/logout");
    
    if needs_auth {
        let ctx = tenant_ctx.ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
        
        let session_id = SessionUtil::get_session_id(req.headers());
        
        if let Some(sid) = session_id {
            if let Ok(Some((_session, user))) = UserRepository::find_session(&ctx.pool, &sid).await {
                req.extensions_mut().insert(user);
                let mut response = next.run(req).await;
                // Production Tip: Ensure the tenant hint is always present for the root-level redirect
                SessionUtil::set_tenant_cookie(&mut response, &ctx.tenant.slug);
                return Ok(response);
            }
        }
        
        // Redirect to login if not authenticated
        let login_url = format!("/{}/login", ctx.tenant.slug);
        return Ok(Redirect::to(&login_url).into_response());
    }
    
    Ok(next.run(req).await)
}

#[derive(Clone, Debug)]
pub struct TenantContext {
    pub tenant: Tenant,
    pub pool: SqlitePool,
}