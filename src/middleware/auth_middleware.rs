use ax_middleware::Next;
use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware as ax_middleware,
    response::{IntoResponse, Redirect, Response},
};
use crate::middleware::TenantContext;
use crate::repository::UserRepository;
use crate::util::SessionUtil;

pub async fn auth_middleware(
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let tenant_ctx = req.extensions().get::<TenantContext>()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let session_id = SessionUtil::get_session_id(req.headers());
    
    if let Some(sid) = session_id {
        if let Ok(Some((_session, user))) = UserRepository::find_session(&tenant_ctx.pool, &sid).await {
            let mut req = req;
            req.extensions_mut().insert(user);
            return Ok(next.run(req).await);
        }
    }

    // Unauthenticated logic
    let path = req.uri().path();
    if path.starts_with("/api/") {
        Err(StatusCode::UNAUTHORIZED)
    } else {
        let login_url = format!("/web/{}/login", tenant_ctx.tenant.slug);
        Ok(Redirect::to(&login_url).into_response())
    }
}
