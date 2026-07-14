use axum::{
    body::Body,
    http::{Request, Response},
    middleware::Next,
};
use crate::middleware::TenantContext;
use crate::repository::UserRepository;
use crate::util::SessionUtil;
use crate::util::AppError;

pub async fn auth_middleware(
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    let ctx = TenantContext::from_req(&req)?;
    
    if let Some(sid) = SessionUtil::get_session_id(req.headers()) {
        if let Some((_, user)) = UserRepository::find_session(&ctx.pool, &sid).await? {
            // Attach user to request extensions
            req.extensions_mut().insert(user);
            return Ok(next.run(req).await);
        }
    }

    // No valid session: Redirect to institution login
    Ok(axum::response::Redirect::to(&format!("/web/{}/login", ctx.tenant.slug)).into_response())
}
