use axum::{
    extract::Extension,
    http::HeaderMap,
    response::{IntoResponse, Redirect, Response},
};
use shared::{AppState, TenantContext};
use sqlx::SqlitePool;

use crate::repository;
use crate::session;

pub async fn process_logout(
    Extension(pool): Extension<SqlitePool>,
    Extension(ctx): Extension<TenantContext>,
    headers: HeaderMap,
) -> Response {
    if let Some(session_id) = session::extract_session_cookie(&headers) {
        let _ = repository::delete_session(&pool, &session_id).await;
    }
    let mut resp = Redirect::to(&format!("/t/{}/login", ctx.slug)).into_response();
    session::clear_session_cookie(&mut resp);
    resp
}

