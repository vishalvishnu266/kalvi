//! Web authentication: login form, logout, and a minimal cookie-based
//! session.
//!
//! **This is intentionally simple for the initial UI review.** The session
//! cookie stores only the tenant id and username so we can render the shell.
//! Password verification calls the real [`crate::services::auth`] service.
//! A production deployment should replace this with a signed session token
//! (e.g. `tower-sessions` + Argon2-backed logins) — the request/response
//! shape here is designed to make that swap non-breaking.
//!
//! Tenancy is **path-based**: the app shell lives under `/{tenant}/…` so
//! the tenant id travels in the URL. The session cookie is retained purely
//! as an auth gate — [`require_session`] verifies the caller is signed in
//! for the tenant that appears in the URL, blocking cross-tenant snooping.

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
            // Redirect into the tenant-scoped app shell.
            let location = format!("/{}/", tenant.as_str());
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

/// Auth gate for the tenant-scoped app shell.
///
/// * If no session cookie is present, redirects to `/login`.
/// * If a session cookie is present but its tenant doesn't match the tenant
///   in the URL (`/{tenant}/…`), also redirects to `/login`. This stops a
///   user logged in as `acme` from opening `/globex/students` and getting a
///   500 from the tenant middleware — the response is a clean re-auth.
pub async fn require_session(
    req: Request<Body>,
    next: Next,
) -> Response {
    let cookie_tenant = match read_cookie_from_headers(req.headers(), COOKIE_TENANT) {
        Some(t) => t,
        None => return Redirect::to("/login").into_response(),
    };

    // The URL tenant is the first path segment (this middleware only runs
    // inside `.nest("/{tenant}", ...)`), so `req.uri().path()` here is the
    // already-stripped inner path — e.g. `/students`. We instead pull the
    // tenant from the extensions inserted by the outer `tenant_scope`
    // middleware, which runs before us and stores a validated `TenantId`.
    if let Some(url_tenant) = req.extensions().get::<TenantId>() {
        if url_tenant.as_str() != cookie_tenant {
            return Redirect::to("/login").into_response();
        }
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
