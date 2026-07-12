use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response, Redirect},
};
use crate::middleware::TenantMiddleware::TenantContext;
use crate::repository::UserRepository::UserRepository;
use crate::util::SessionUtil;
use crate::model::User::User;

pub async fn auth_middleware(
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let ctx = req.extensions().get::<TenantContext>().ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
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
    Ok(Redirect::to(&login_url).into_response())
}
