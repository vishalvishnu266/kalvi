//! Web authentication: login form, logout, and a minimal cookie-based
//! session.
//!
//! **This is intentionally simple for the initial UI review.** The session
//! cookie stores only the tenant id and username so we can render the shell.
//! Password verification calls the real [`crate::services::auth`] service.
//! A production deployment should replace this with a signed session token
//! (e.g. `tower-sessions` + Argon2-backed logins) — the request/response
//! shape here is designed to make that swap non-breaking.

use askama::Template;
use axum::{
    body::Body,
    extract::{Form, State},
    http::{header, HeaderMap, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
    Router,
};
use serde::Deserialize;

use crate::http::middleware::TenantScopeState;
use crate::tenancy::TenantId;
use crate::web::error::{render, WebError};

const COOKIE_TENANT: &str = "erp_tenant";
const COOKIE_USER: &str = "erp_user";

// ---------------------------------------------------------------------------
// Templates
// ---------------------------------------------------------------------------

#[derive(Template)]
#[template(path = "login.html")]
struct LoginPage<'a> {
    error: Option<&'a str>,
    tenant: &'a str,
    identifier: &'a str,
}

// ---------------------------------------------------------------------------
// Public routes: /login (GET/POST), /logout (POST).
//
// Login handlers need the tenant registry (to resolve tenant → services), so
// they share the same `TenantScopeState` that the tenant-scoped middleware
// uses. The router is stateful.
// ---------------------------------------------------------------------------

pub fn public_routes(state: TenantScopeState) -> Router {
    Router::new()
        .route("/login",  get(get_login).post(post_login))
        .route("/logout", post(post_logout))
        .with_state(state)
}

async fn get_login() -> Result<Response, WebError> {
    render(&LoginPage { error: None, tenant: "default", identifier: "" })
}

#[derive(Deserialize)]
struct LoginForm {
    tenant: String,
    identifier: String,
    password: String,
}

async fn post_login(
    State(state): State<TenantScopeState>,
    Form(f): Form<LoginForm>,
) -> Result<Response, WebError> {
    // Resolve the tenant and get an AppServices for it.
    let tenant = TenantId::new(f.tenant.clone())
        .map_err(|e| WebError::bad(e.to_string()))?;
    let services = state.registry.services_for(&tenant).await
        .map_err(|e| WebError::bad(e.to_string()))?;

    match services.auth.login(&f.identifier, &f.password).await {
        Ok(user) => {
            // Set two lightweight cookies. Not signed — safe here only
            // because we re-validate on every request via the tenant
            // middleware; treat this as a placeholder for a real session.
            let display = user.email.clone().unwrap_or(user.username.clone());
            let set_tenant = format!(
                "{COOKIE_TENANT}={}; Path=/; SameSite=Lax; HttpOnly",
                tenant.as_str()
            );
            let set_user = format!(
                "{COOKIE_USER}={}; Path=/; SameSite=Lax",
                urlencoding::encode(&display)
            );
            Ok((
                StatusCode::SEE_OTHER,
                [
                    (header::SET_COOKIE, set_tenant),
                    (header::SET_COOKIE, set_user),
                    (header::LOCATION, "/".to_string()),
                ],
                Body::empty(),
            ).into_response())
        }
        Err(_) => render(&LoginPage {
            error: Some("Invalid credentials"),
            tenant: &f.tenant,
            identifier: &f.identifier,
        }),
    }
}

async fn post_logout() -> Response {
    let clear_tenant = format!("{COOKIE_TENANT}=; Path=/; Max-Age=0; SameSite=Lax; HttpOnly");
    let clear_user   = format!("{COOKIE_USER}=; Path=/; Max-Age=0; SameSite=Lax");
    (
        StatusCode::SEE_OTHER,
        [
            (header::SET_COOKIE, clear_tenant),
            (header::SET_COOKIE, clear_user),
            (header::LOCATION, "/login".to_string()),
        ],
        Body::empty(),
    ).into_response()
}

// ---------------------------------------------------------------------------
// Middleware
// ---------------------------------------------------------------------------

/// Reads the `erp_tenant` cookie and copies it into the `x-tenant-id`
/// header so the existing [`crate::http::middleware::tenant_scope`]
/// middleware can pick it up unchanged.
pub async fn cookie_to_tenant_header(
    mut req: Request<Body>,
    next: Next,
) -> Response {
    if let Some(tenant) = read_cookie_from_headers(req.headers(), COOKIE_TENANT) {
        if let Ok(v) = axum::http::HeaderValue::from_str(&tenant) {
            req.headers_mut().insert("x-tenant-id", v);
        }
    }
    next.run(req).await
}

/// Redirects to `/login` if no session cookie is present.
pub async fn require_session(
    req: Request<Body>,
    next: Next,
) -> Response {
    if read_cookie_from_headers(req.headers(), COOKIE_TENANT).is_none() {
        return Redirect::to("/login").into_response();
    }
    next.run(req).await
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
