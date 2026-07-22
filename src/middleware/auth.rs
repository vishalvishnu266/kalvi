use std::collections::HashMap;
use axum::{
    body::Body,
    extract::State,
    http::{header, HeaderMap, Request as AxumRequest},
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use axum::extract::Path;
use crate::http::AppState;
use crate::tenancy::TenantId;

/// Cookie names.
///
/// * `erp_tenant`  — remembers which tenant the user last signed into so
///   the landing page can auto-redirect to `/web/{tenant}/`.
/// * `erp_user`    — display-only cookie (used by the sidebar to render
///   the user's name without a DB hit). NOT authoritative.
/// * `erp_session` — opaque server-side session token; the ONLY cookie
///   the auth gate trusts.
pub const COOKIE_TENANT: &str = "erp_tenant";
pub const COOKIE_USER: &str = "erp_user";
pub const COOKIE_SESSION: &str = "erp_session";

/// Authenticated caller info, attached to the request by [`require_session`]
/// and read by handlers via `Extension<SessionUser>`.
#[derive(Clone, Debug)]
pub struct SessionUser {
    pub user_id: i64,
    pub username: String,
    pub display: String,
    pub session_id: i64,
}

/// Auth gate for the tenant-scoped app shell (`/web/{tenant}/…`).
///
/// Rules, evaluated top-down:
/// 1. Pull the `{tenant}` segment from the URL. If absent, we can't scope
///    the check — redirect to the global login.
/// 2. Require an `erp_session` cookie. Missing → redirect to
///    `/web/{tenant}/login`.
/// 3. Resolve the session against the **tenant's own DB** (sessions live
///    per-tenant). Unknown / revoked / expired → redirect to login.
/// 4. Stash the resolved user into the request extensions so downstream
///    handlers can read it via [`SessionUser`] without a second DB hit.
pub async fn require_session(
    State(state): State<AppState>,
    Path(params): Path<HashMap<String, String>>, // 1. Moved Path BEFORE `req`
    mut req: AxumRequest<Body>,
    next: Next,                                  // 2. Next is last
) -> Response {
    // 3. Extract the tenant directly from path parameters
    let tenant_param = params.get("tenant").map(|s| s.as_str());

    // Construct the redirect URL based on whether a tenant was found
    let login_url = match tenant_param {
        Some(t) => format!("/web/{}/login", t),
        None    => "/web/login".to_string(),
    };

    // Need a tenant in the URL to know which DB to check the session against.
    let Some(t_str) = tenant_param else {
        return Redirect::to(&login_url).into_response();
    };

    let Ok(tenant) = TenantId::new(t_str.to_string()) else {
        return Redirect::to(&login_url).into_response();
    };

    let Some(token) = read_cookie_from_headers(req.headers(), COOKIE_SESSION) else {
        return Redirect::to(&login_url).into_response();
    };

    let Ok(services) = state.tenants.services_for(&tenant).await else {
        return Redirect::to(&login_url).into_response();
    };

    let (session, user) = match services.auth.resolve_session(&token).await {
        Ok(pair) => pair,
        Err(_)   => return Redirect::to(&login_url).into_response(),
    };

    // Make the authenticated user available to handlers without re-reading
    // cookies / re-hitting the DB.
    req.extensions_mut().insert(SessionUser {
        user_id: user.id,
        username: user.username.clone(),
        display: user.email.clone().unwrap_or_else(|| user.username.clone()),
        session_id: session.id,
    });

    next.run(req).await
}

/// Parse the `{tenant}` segment out of a URL path like `/web/acme/students`.
/// Returns `None` if the path doesn't start with `/web/<tenant>/`.
fn url_tenant_from(path: &str) -> Option<String> {
    let rest = path.strip_prefix("/web/")?;
    let seg = rest.split('/').next()?;
    if seg.is_empty() || seg == "login" || seg == "logout" { return None; }
    Some(seg.to_string())
}

/// Reads a cookie value from a headermap. Returns `None` if missing.
pub fn read_cookie_from_headers(headers: &HeaderMap, name: &str) -> Option<String> {
    let raw = headers.get(header::COOKIE)?.to_str().ok()?;
    for kv in raw.split(';') {
        let kv = kv.trim();
        if let Some((k, v)) = kv.split_once('=') {
            if k == name {
                return Some(urlencoding::decode(v).ok()?.into_owned());
            }
        }
    }
    None
}
