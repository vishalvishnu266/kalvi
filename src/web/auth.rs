//! Web authentication: login form, logout, and a minimal cookie session.
//!
//! **Intentionally simple** — the cookie stores only the tenant id and
//! username so we can render the shell. Password check goes through
//! [`crate::services::auth`]. Swap for signed sessions / OIDC / etc. later.
//!
//! Tenancy is path-based (`/web/{tenant}/…`). The session cookie is an
//! auth gate only — [`require_session`] verifies the caller is signed in
//! for the tenant that appears in the URL to stop cross-tenant snooping.
//!
//! URL map (routes wired in [`crate::http::routes`]):
//! * `GET  /web/login`              — global login form.
//! * `POST /web/login`              — submit login.
//! * `GET  /web/{tenant}/login`     — tenant-specific login form.
//! * `POST /web/logout`             — clear the session cookies.

use askama::Template;
use axum::{
    body::Body,
    extract::{Form, Path, State},
    http::{header, HeaderMap, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use serde::Deserialize;

use crate::http::AppState;
use crate::tenancy::TenantId;
use crate::web::error::{render, WebError};

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

// ---------------------------------------------------------------- template

#[derive(Template)]
#[template(path = "login.html")]
struct LoginPage<'a> {
    error: Option<&'a str>,
    tenant: &'a str,
    identifier: &'a str,
    tenant_locked: bool,
}

// ---------------------------------------------------------------- handlers

pub async fn get_login() -> Result<Response, WebError> {
    render(&LoginPage { error: None, tenant: "", identifier: "", tenant_locked: false })
}

pub async fn get_tenant_login(Path(tenant): Path<String>) -> Result<Response, WebError> {
    render(&LoginPage { error: None, tenant: &tenant, identifier: "", tenant_locked: true })
}

#[derive(Deserialize)]
pub struct LoginForm {
    tenant: String,
    identifier: String,
    password: String,
}

pub async fn post_login(
    State(state): State<AppState>,
    Form(f): Form<LoginForm>,
) -> Result<Response, WebError> {
    let tenant = match TenantId::new(f.tenant.clone()) {
        Ok(t) => t,
        Err(e) => {
            return render(&LoginPage {
                error: Some(&e.to_string()),
                tenant: &f.tenant, identifier: &f.identifier,
                tenant_locked: false,
            });
        }
    };
    let services = match state.tenants.services_for(&tenant).await {
        Ok(s) => s,
        Err(e) => {
            return render(&LoginPage {
                error: Some(&format!("Tenant lookup failed: {e}")),
                tenant: &f.tenant, identifier: &f.identifier,
                tenant_locked: false,
            });
        }
    };

    match services.auth.login(&f.identifier, &f.password).await {
        Ok(user) => {
            // Issue a server-side session stored in the tenant DB.
            let session = match services.auth.issue_session(user.id, None, None).await {
                Ok(s) => s,
                Err(e) => {
                    return render(&LoginPage {
                        error: Some(&format!("Could not start session: {e}")),
                        tenant: &f.tenant, identifier: &f.identifier,
                        tenant_locked: !f.tenant.is_empty(),
                    });
                }
            };

            let display = user.email.clone().unwrap_or(user.username.clone());
            let max_age = (session.expires_at - chrono::Utc::now().naive_utc())
                .num_seconds().max(0);
            let set_tenant = format!(
                "{COOKIE_TENANT}={}; Path=/; Max-Age={max_age}; SameSite=Lax; HttpOnly",
                tenant.as_str()
            );
            let set_user = format!(
                "{COOKIE_USER}={}; Path=/; Max-Age={max_age}; SameSite=Lax",
                urlencoding::encode(&display)
            );
            let set_session = format!(
                "{COOKIE_SESSION}={}; Path=/; Max-Age={max_age}; SameSite=Lax; HttpOnly",
                session.token
            );
            let location = format!("/web/{}/", tenant.as_str());
            Ok((
                StatusCode::SEE_OTHER,
                [
                    (header::SET_COOKIE, set_tenant),
                    (header::SET_COOKIE, set_user),
                    (header::SET_COOKIE, set_session),
                    (header::LOCATION, location),
                ],
                Body::empty(),
            ).into_response())
        }
        Err(_) => render(&LoginPage {
            error: Some("Invalid credentials"),
            tenant: &f.tenant, identifier: &f.identifier,
            tenant_locked: !f.tenant.is_empty(),
        }),
    }
}

/// Sign out: revoke the current session (server-side) and clear cookies.
///
/// Best-effort — if we can't reach the tenant DB (e.g. tenant disabled),
/// we still clear the cookies client-side and redirect to the login page.
pub async fn post_logout(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Response {
    // Try to revoke the session server-side.
    let cookie_tenant  = read_cookie_from_headers(&headers, COOKIE_TENANT);
    let cookie_session = read_cookie_from_headers(&headers, COOKIE_SESSION);
    if let (Some(tid), Some(token)) = (cookie_tenant, cookie_session) {
        if let Ok(t) = TenantId::new(tid) {
            if let Ok(services) = state.tenants.services_for(&t).await {
                let _ = services.auth.revoke_session(&token).await;
            }
        }
    }

    let clear_tenant  = format!("{COOKIE_TENANT}=; Path=/; Max-Age=0; SameSite=Lax; HttpOnly");
    let clear_user    = format!("{COOKIE_USER}=; Path=/; Max-Age=0; SameSite=Lax");
    let clear_session = format!("{COOKIE_SESSION}=; Path=/; Max-Age=0; SameSite=Lax; HttpOnly");
    (
        StatusCode::SEE_OTHER,
        [
            (header::SET_COOKIE, clear_tenant),
            (header::SET_COOKIE, clear_user),
            (header::SET_COOKIE, clear_session),
            (header::LOCATION, "/web/login".to_string()),
        ],
        Body::empty(),
    ).into_response()
}

// ---------------------------------------------------------------- session gate

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
    mut req: Request<Body>,
    next: Next,
) -> Response {
    let url_tenant = url_tenant_from(req.uri().path());
    let login_url = match &url_tenant {
        Some(t) => format!("/web/{}/login", t),
        None    => "/web/login".to_string(),
    };

    // Need a tenant in the URL to know which DB to check the session against.
    let Some(t_str) = url_tenant.as_deref() else {
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
        display: user.email.clone().unwrap_or(user.username.clone()),
        session_id: session.id,
    });

    next.run(req).await
}

/// Authenticated caller info, attached to the request by [`require_session`]
/// and read by handlers via `Extension<SessionUser>`.
#[derive(Clone, Debug)]
pub struct SessionUser {
    pub user_id: i64,
    pub username: String,
    pub display: String,
    pub session_id: i64,
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
