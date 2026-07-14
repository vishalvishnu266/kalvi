use axum::{
    http::Request,
    middleware::Next,
    response::{Response, Redirect},
};
use crate::middleware::tenant_middleware::TenantContext;
use crate::repository::user_repository::UserRepository;
use crate::util::id_util::current_timestamp;
use crate::util::errors::AppError;
use cookie::Cookie;

pub async fn auth_middleware<B>(
    req: Request<B>,
    next: Next,
) -> Result<Response, AppError> {
    // 1. Get Tenant Context from extensions
    let context = req.extensions().get::<TenantContext>()
        .ok_or_else(|| AppError::RuntimeException("TenantContext missing in request extensions".to_string()))?;

    // 2. Get kalvi_session cookie
    let session_cookie = req.headers()
        .get("cookie")
        .and_then(|h| h.to_str().ok())
        .and_then(|c| {
            Cookie::split_parse(c)
                .find(|res| res.is_ok() && res.as_ref().unwrap().name() == "kalvi_session")
                .and_then(|res| res.ok())
        });

    if let Some(cookie) = session_cookie {
        let now = current_timestamp();
        // 3. Verify session in the TENANT's isolated DB
        if let Some(user) = UserRepository::find_by_session(&context.pool, cookie.value(), now).await? {
            // 4. Valid session, inject user and continue
            let mut req = req;
            req.extensions_mut().insert(user);
            return Ok(next.run(req).await);
        }
    }

    // 5. Fail: Redirect to tenant-specific login
    let login_url = format!("/web/{}/login", context.tenant.slug);
    Ok(Redirect::to(&login_url).into_response())
}
