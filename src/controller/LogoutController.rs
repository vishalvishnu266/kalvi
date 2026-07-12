use axum::{
    extract::{Extension, State},
    response::{IntoResponse, Redirect},
    http::HeaderMap,
};
use crate::config::AppState::AppState;
use crate::middleware::AppMiddleware::TenantContext;
use crate::repository::UserRepository::UserRepository;
use crate::util::SessionUtil;

pub async fn process_logout(
    State(_state): State<AppState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    if let Some(session_id) = SessionUtil::get_session_id(&headers) {
        if session_id.starts_with("saas_") {
            let mut response = Redirect::to("/saas/login").into_response();
            SessionUtil::clear_session_cookie(&mut response);
            return response;
        }
    }

    let mut response = Redirect::to("/login").into_response();
    SessionUtil::clear_session_cookie(&mut response);
    response
}

pub async fn process_tenant_logout(
    Extension(ctx): Extension<TenantContext>,
    headers: HeaderMap,
) -> impl IntoResponse {
    if let Some(session_id) = SessionUtil::get_session_id(&headers) {
        let _ = UserRepository::delete_session(&ctx.pool, &session_id).await;
    }

    let mut response = Redirect::to(&format!("/{}/login", ctx.tenant.slug)).into_response();
    SessionUtil::clear_session_cookie(&mut response);
    response
}
