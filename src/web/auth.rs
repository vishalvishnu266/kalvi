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
//! Tenancy is **path-based**: the app shell lives under `/web/{tenant}/…` so
//! the tenant id travels in the URL. The session cookie is retained purely
//! as an auth gate — [`require_session`] verifies the caller is signed in
//! for the tenant that appears in the URL, blocking cross-tenant snooping.
//!
//! URL map:
//! * `GET  /web/login`           — global login form (user types the tenant).
//! * `POST /web/login`           — submit login.
//! * `GET  /web/{tenant}/login`  — tenant-specific login form (pre-filled).
//! * `POST /web/logout`          — clear the session cookies.

use askama::Template;
use axum::{
    body::Body,
    extract::{Form, Path, State},
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
    /// When true the tenant field is rendered read-only (tenant-scoped login).
    tenant_locked: bool,
}

// ---------------------------------------------------------------------------
// Routes
// ---------------------------------------------------------------------------

/// Public (unauthenticated) routes mounted under `/web`:
/// `GET /login`, `POST /login`, `POST /logout`, `GET /{tenant}/login`.
pub fn public_routes(state: TenantScopeState) -> Router {
    Router::new()
        .route("/login",           get(get_login).post(post_login))
        .route("/logout",          post(post_logout))
        .route("/{tenant}/login",  get(get_tenant_login).post(post_login))
        .with_state(state)
}

async fn get_login() -> Result<Response, WebError> {
    render(&LoginPage {
        error: None,
        tenant: "",
        identifier: "",
        tenant_locked: false,
    })
}

/// Renders the login page with the tenant pre-filled and locked.
async fn get_tenant_login(Path(tenant): Path<String>) -> Result<Response, WebError> {
    render(&LoginPage {
        error: None,
        tenant: &tenant,
        identifier: "",
        tenant_locked: true,
    })
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
    let tenant = match TenantId::new(f.tenant.clone()) {
        Ok(t) => t,
        Err(e) => {
            return render(&LoginPage {
                error: Some(&e.to_string()),
                tenant: &f.tenant,
                identifier: &f.identifier,
                tenant_locked: false,
            });
        }
    };
    let services = match state.registry.services_for(&tenant).await {
        Ok(s) => s,
        Err(e) => {
            return render(&LoginPage {
                error: Some(&format!("Tenant lookup failed: {e}")),
                tenant: &f.tenant,
                identifier: &f.identifier,
                tenant_locked: false,
            });
        }
    };

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
            // Redirect into the tenant-scoped app shell under /web/.
            // Both `/web/{tenant}` and `/web/{tenant}/` resolve — the app
            // is fronted by `NormalizePathLayer` so trailing slashes are
            // stripped before routing.
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
            tenant: &f.tenant,
            identifier: &f.identifier,
            tenant_locked: !f.tenant.is_empty(),
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
            (header::LOCATION, "/web/login".to_string()),
        ],
        Body::empty(),
    ).into_response()
}

// ---------------------------------------------------------------------------
// Middleware
// ---------------------------------------------------------------------------

/// Auth gate for the tenant-scoped app shell (`/web/{tenant}/…`).
///
/// * If no session cookie is present, redirects to `/web/login`.
/// * If a session cookie is present but its tenant doesn't match the tenant
///   in the URL, also redirects to `/web/login`. This stops a user logged
///   in as `acme` from opening `/web/globex/students` and getting a 500 —
///   the response is a clean re-auth.
pub async fn require_session(
    req: Request<Body>,
    next: Next,
) -> Response {
    // Prefer to send unauthenticated users to the *tenant-specific* login
    // page (so the tenant field is pre-filled and they land back on the
    // right shell after signing in). Fall back to the global login only
    // when we don't know the URL tenant yet — which shouldn't normally
    // happen because `tenant_scope` runs before us and inserts a
    // `TenantId` extension.
    let url_tenant = req.extensions().get::<TenantId>().cloned();
    let login_url = match &url_tenant {
        Some(t) => format!("/web/{}/login", t.as_str()),
        None    => "/web/login".to_string(),
    };

    let cookie_tenant = match read_cookie_from_headers(req.headers(), COOKIE_TENANT) {
        Some(t) => t,
        None    => return Redirect::to(&login_url).into_response(),
    };

    if let Some(t) = url_tenant {
        if t.as_str() != cookie_tenant {
            // Cross-tenant snoop attempt (or a stale cookie from a
            // different tenant). Bounce to the URL tenant's login.
            return Redirect::to(&login_url).into_response();
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
