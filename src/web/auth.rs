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

const COOKIE_TENANT: &str = "erp_tenant";
const COOKIE_USER: &str = "erp_user";

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
            let display = user.email.clone().unwrap_or(user.username.clone());
            let set_tenant = format!(
                "{COOKIE_TENANT}={}; Path=/; SameSite=Lax; HttpOnly",
                tenant.as_str()
            );
            let set_user = format!(
                "{COOKIE_USER}={}; Path=/; SameSite=Lax",
                urlencoding::encode(&display)
            );
            let location = format!("/web/{}/", tenant.as_str());
            Ok((
                StatusCode::SEE_OTHER,
                [
                    (header::SET_COOKIE, set_tenant),
                    (header::SET_COOKIE, set_user),
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

pub async fn post_logout() -> Response {
    let clear_tenant = format!("{COOKIE_TENANT}=; Path=/; Max-Age=0; SameSite=Lax; HttpOnly");
    let clear_user   = format!("{COOKIE_USER}=; Path=/; Max-Age=0; SameSite=Lax");
    (
        StatusCode::SEE_OTHER,
        [
            (header::SET_COOKIE, clear_tenant),
            (header::SET_COOKIE, clear_user),
            (header::LOCATION, "/web/login".to_string()),
        ],
        Body::empty(),
    ).into_response()
}

// ---------------------------------------------------------------- session gate

/// Auth gate for the tenant-scoped app shell (`/web/{tenant}/…`).
///
/// * If no session cookie is present, redirects to `/web/{tenant}/login`
///   (or `/web/login` if the URL doesn't have a resolvable tenant).
/// * If a session cookie is present but its tenant doesn't match the tenant
///   in the URL, also redirects to the URL tenant's login. This stops a
///   user logged in as `acme` from opening `/web/globex/students`.
///
/// The URL tenant is extracted directly from the request path — no request
/// extension plumbing, no middleware upstream. This keeps the auth gate
/// self-contained: it depends only on the request itself.
pub async fn require_session(req: Request<Body>, next: Next) -> Response {
    let url_tenant = url_tenant_from(req.uri().path());
    let login_url = match &url_tenant {
        Some(t) => format!("/web/{}/login", t),
        None    => "/web/login".to_string(),
    };

    let cookie_tenant = match read_cookie_from_headers(req.headers(), COOKIE_TENANT) {
        Some(t) => t,
        None    => return Redirect::to(&login_url).into_response(),
    };

    if let Some(t) = url_tenant.as_deref() {
        if t != cookie_tenant {
            return Redirect::to(&login_url).into_response();
        }
    }
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
