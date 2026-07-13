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
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    let ctx = TenantContext::from_req(&req)?;
    
    if let Some(sid) = SessionUtil::get_session_id(req.headers()) {
        if let Some((_, user)) = UserRepository::find_session(&ctx.pool, &sid).await? {
            req.extensions_mut().insert(user);
            return Ok(next.run(req).await);
        }
    }

    if req.uri().path().starts_with("/api/") {
        Err(AppError::Unauthorized("API access requires authentication".to_string()))
    } else {
        Ok(Redirect::to(&ctx.login_url()).into_response())
    }
}
