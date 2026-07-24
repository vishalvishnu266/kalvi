use std::collections::{HashMap, HashSet};
use axum::{
    body::Body,
    extract::{Path, State},
    http::Request as AxumRequest,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};

use crate::http::AppState;
use crate::middleware::auth::{COOKIE_SESSION, read_cookie_from_headers, SessionUser};
use crate::tenancy::TenantId;

pub async fn require_session(
    State(state): State<AppState>,
    Path(params): Path<HashMap<String, String>>,
    mut req: AxumRequest<Body>,
    next: Next,
) -> Response {
    let t_str = params.get("tenant").map(|s| s.as_str())
        .ok_or_else(|| Redirect::to("/web/login").into_response());

    let t_str = match t_str {
        Ok(t) => t,
        Err(r) => return r,
    };

    let login_url = format!("/web/{}/login", t_str);

    let Ok(tenant) = TenantId::new(t_str.to_string()) else {
        return Redirect::to(&login_url).into_response();
    };

    let Some(token) = read_cookie_from_headers(req.headers(), COOKIE_SESSION) else {
        return Redirect::to(&login_url).into_response();
    };

    let Ok(services) = state.services_for(&tenant).await else {
        return Redirect::to(&login_url).into_response();
    };

    let (session, user) = match services.auth.resolve_session(&token).await {
        Ok(pair) => pair,
        Err(_)   => return Redirect::to(&login_url).into_response(),
    };

    let roles: Vec<String> = services.repos.users
        .roles_of(user.id).await
        .map(|rs| rs.into_iter().map(|r| r.name).collect())
        .unwrap_or_default();
    let permissions: HashSet<String> = services.repos.users
        .permissions_of(user.id).await
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

pub async fn require_staff_shell(
    req: AxumRequest<Body>,
    next: Next,
) -> Response {
    let user = req.extensions().get::<SessionUser>();

    if let Some(_) = user {
        return next.run(req).await;
    }

    let path = req.uri().path();
    let tenant = path.split('/').nth(2).unwrap_or("default");
    Redirect::to(&format!("/web/{}/login", tenant)).into_response()
}
