use axum::{
    extract::{Extension, State},
    response::Response,
    http::HeaderMap,
};
use crate::config::AppState;
use crate::middleware::TenantContext;
use crate::repository::UserRepository;
use crate::util::{SessionUtil, AppError};

pub async fn process_logout(
    State(_state): State<AppState>,
    headers: HeaderMap,
) -> Result<Response, AppError> {
    let redirect = if SessionUtil::is_saas_session(&headers) { "/saas/login" } else { "/login" };
    Ok(SessionUtil::finalize_logout(redirect))
}

pub async fn process_tenant_logout(
    Extension(ctx): Extension<TenantContext>,
    headers: HeaderMap,
) -> Result<Response, AppError> {
    if let Some(session_id) = SessionUtil::get_session_id(&headers) {
        UserRepository::delete_session(&ctx.pool, &session_id).await.ok();
    }
    Ok(SessionUtil::finalize_logout(&ctx.login_url()))
}
