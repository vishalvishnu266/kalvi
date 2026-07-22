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
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use serde::Deserialize;

use crate::http::AppState;
use crate::middleware::auth::read_cookie_from_headers;
use crate::tenancy::TenantId;
use crate::web::error::{render, WebError};

pub const COOKIE_TENANT: &str = crate::middleware::auth::COOKIE_TENANT;
pub const COOKIE_USER: &str = crate::middleware::auth::COOKIE_USER;
pub const COOKIE_SESSION: &str = crate::middleware::auth::COOKIE_SESSION;

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
    let cookie_tenant  = read_cookie_from_headers(&headers, crate::middleware::auth::COOKIE_TENANT);
    let cookie_session = read_cookie_from_headers(&headers, crate::middleware::auth::COOKIE_SESSION);
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
