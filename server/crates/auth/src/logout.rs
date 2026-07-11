use axum::{
    extract::Extension,
    http::HeaderMap,
    response::{IntoResponse, Redirect, Response},
    routing::post,
    Router,
};
use shared::{AppState, TenantContext};
use sqlx::SqlitePool;

use crate::session;

async fn process_logout(
    Extension(pool): Extension<SqlitePool>,
    Extension(ctx): Extension<TenantContext>,
    headers: HeaderMap,
) -> Response {
    if let Some(session_id) = session::extract_session_cookie(&headers) {
        let _ = session::delete_session(&pool, &session_id).await;
    }
    let mut resp = Redirect::to(&format!("/t/{}/login", ctx.slug)).into_response();
    session::clear_session_cookie(&mut resp);
    resp
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/t/{tenant_slug}/logout", post(process_logout))
}
