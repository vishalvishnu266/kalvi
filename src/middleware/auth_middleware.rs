use ax_middleware::Next;
use axum::{
    body::Body,
    http::Request,
    middleware as ax_middleware,
    response::{IntoResponse, Redirect, Response},
};
use crate::middleware::TenantContext;
use crate::repository::UserRepository;
use crate::util::{SessionUtil, AppError};

pub async fn auth_middleware(
    req: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    let tenant_ctx = req.extensions().get::<TenantContext>()
        .ok_or(AppError::Internal("Tenant context missing".to_string()))?;
    
    let session_id = SessionUtil::get_session_id(req.headers());
    
    if let Some(sid) = session_id {
        if let Some((_session, user)) = UserRepository::find_session(&tenant_ctx.pool, &sid).await? {
            let mut req = req;
            req.extensions_mut().insert(user);
            return Ok(next.run(req).await);
        }
    }

    let path = req.uri().path();
    if path.starts_with("/api/") {
        Err(AppError::Unauthorized("API access requires authentication".to_string()))
    } else {
        let login_url = format!("/web/{}/login", tenant_ctx.tenant.slug);
        Ok(Redirect::to(&login_url).into_response())
    }
}
