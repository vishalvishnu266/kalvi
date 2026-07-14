use axum::{
    body::Body,
    extract::State,
    http::Request,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use sqlx::SqlitePool;
use crate::config::AppState;
use crate::repository::UserRepository;
use crate::util::{SessionUtil, AppError};
use crate::model::Tenant;
use crate::service::TenantService;

pub async fn tenant_middleware(
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    let segments: Vec<String> = req.uri().path()
        .split('/')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();
    
    let (prefix, slug) = match (segments.get(0), segments.get(1)) {
        (Some(p), Some(s)) => (p, s),
        _ => return Ok(next.run(req).await),
    };

    let tenant = match TenantService::find_by_slug(&state, slug.as_str()).await? {
        Some(t) => t,
        None => return Ok(Redirect::to("/login").into_response()),
    };
        
    let ctx = TenantContext {
        pool: state.db.get_tenant_pool(&tenant.database_name).await?,
        tenant,
    };
    
    req.extensions_mut().insert(ctx.clone());

    if prefix == "web" && segments.len() == 2 {
        let redirect_url = if SessionUtil::get_session_id(req.headers()).is_some() {
            ctx.dashboard_url()
        } else {
            ctx.login_url()
        };
        return Ok(Redirect::to(&redirect_url).into_response());
    }
    
    Ok(next.run(req).await)
}

#[derive(Clone, Debug)]
pub struct TenantContext {
    pub tenant: Tenant,
    pub pool: SqlitePool,
}

impl TenantContext {
    pub fn from_req(req: &Request<Body>) -> Result<Self, AppError> {
        req.extensions().get::<Self>()
            .cloned()
            .ok_or_else(|| AppError::Internal("Tenant context missing".to_string()))
    }

    pub fn login_url(&self) -> String {
        format!("/web/{}/login", self.tenant.slug)
    }

    pub fn dashboard_url(&self) -> String {
        format!("/web/{}/dashboard", self.tenant.slug)
    }
}