use axum::{
    body::Body,
    extract::{Path, State},
    http::Request as AxumRequest,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use std::collections::{HashMap, HashSet};

use crate::application::AppState;
use crate::security::session_user::{read_cookie_from_headers, SessionUser, COOKIE_SESSION};
use crate::service::auth_service;
use crate::tenant::tenant_id::validate_tenant_id;

/// Resolves the `{tenant}` path segment + session cookie into a
/// [`SessionUser`] extension. Used by both web and portal shells.
pub async fn require_session(
    State(state): State<AppState>,
    Path(params): Path<HashMap<String, String>>,
    mut req: AxumRequest<Body>,
    next: Next,
) -> Response {
    let t_str = match params.get("tenant").map(|s| s.as_str()) {
        Some(t) => t,
        None => return Redirect::to("/web/login").into_response(),
    };
    let path_prefix = if req.uri().path().starts_with("/portal") {
        "portal"
    } else {
        "web"
    };
    let login_url = format!("/{}/{}/login", path_prefix, t_str);

    let Ok(tenant) = validate_tenant_id(t_str.to_string()) else {
        return Redirect::to(&login_url).into_response();
    };
    let Some(token) = read_cookie_from_headers(req.headers(), COOKIE_SESSION) else {
        return Redirect::to(&login_url).into_response();
    };
    let Ok(pool) = state.pool_for(&tenant).await else {
        return Redirect::to(&login_url).into_response();
    };

    let (session, user) = match auth_service::resolve_session(&pool, &state.sessions, &token).await
    {
        Ok(pair) => pair,
        Err(_) => return Redirect::to(&login_url).into_response(),
    };

    let roles: Vec<String> = auth_service::roles_of(&pool, user.id)
        .await
        .map(|rs| rs.into_iter().map(|r| r.name).collect())
        .unwrap_or_default();
    let permissions: HashSet<String> = auth_service::permissions_of(&pool, user.id)
        .await
        .map(|ps| ps.into_iter().map(|p| p.code).collect())
        .unwrap_or_default();

    req.extensions_mut().insert(SessionUser {
        user_id: user.id,
        username: user.username.clone(),
        display: user.email.clone().unwrap_or_else(|| user.username.clone()),
        session_id: session.id,
        roles,
        permissions,
    });

    next.run(req).await
}
