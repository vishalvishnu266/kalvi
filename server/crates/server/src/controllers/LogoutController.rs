use axum::{
    extract::Extension,
    http::HeaderMap,
    response::{IntoResponse, Redirect, Response},
};
use crate::middleware::TenantMiddleware::TenantContext;
use sqlx::SqlitePool;

use crate::repositories::UserRepository;
use crate::web_utils::SessionUtils;

pub async fn process_logout(
    Extension(pool): Extension<SqlitePool>,
    Extension(ctx): Extension<TenantContext>,
    headers: HeaderMap,
) -> Response {
    if let Some(session_id) = SessionUtils::extract_session_cookie(&headers) {
        let _ = UserRepository::delete_session(&pool, &session_id).await;
    }
    let mut resp = Redirect::to(&format!("/t/{}/login", ctx.slug)).into_response();
    SessionUtils::clear_session_cookie(&mut resp);
    resp
}
