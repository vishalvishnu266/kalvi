//! Cookie-backed session authentication for `/web/{tenant}/*` routes.
//!
//! Behaviour:
//!   * Reads `sd_session` cookie.
//!   * Looks up the session in the tenant DB (must be non-expired).
//!   * Loads the associated `User`.
//!   * Injects `CurrentUser(User)` into request extensions so handlers can
//!     depend on it via `Extension<CurrentUser>`.
//!   * If any of the above fails, redirects the browser to
//!     `/web/{tenant}/login` (preserving the original URL as `?next=`).
//!
//! Login/logout routes must be exempted from this middleware — see
//! `routes.rs`. They rely on the tenant DB middleware (for the pool) but
//! never require an existing user.
use axum::{
    extract::{Path, Request},
    http::{header, StatusCode},
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use sqlx::SqlitePool;
use std::collections::HashMap;

use crate::models::session::{Session, SESSION_COOKIE_NAME};
use crate::models::user::User;

/// Newtype for the current authenticated user, injected into request
/// extensions. Handlers accept `Extension<CurrentUser>` to read it.
#[derive(Clone)]
pub struct CurrentUser(pub User);

// -------------------------------------------------------------------
// require_role middleware
// -------------------------------------------------------------------

/// Middleware factory: only allow requests whose `CurrentUser` has one of
/// the listed roles. Runs AFTER `auth_middleware`.
///
/// # Example (used in `routes.rs`):
/// ```ignore
/// .layer(middleware::from_fn(require_role(&[Role::Admin])))
/// ```
///
/// Behaviour on failure: returns a **403 Forbidden** page rendered via
/// `AppError::Forbidden`. We deliberately don't redirect to login — the
/// user IS logged in; they just lack the role.
pub fn require_role(
    allowed: &'static [crate::models::user::Role],
) -> impl Fn(
    axum::extract::Request,
    axum::middleware::Next,
) -> std::pin::Pin<
    Box<dyn std::future::Future<Output = axum::response::Response> + Send>,
> + Clone
       + Send
       + Sync
       + 'static {
    move |req, next| {
        Box::pin(async move {
            use axum::response::IntoResponse;
            let ok = req
                .extensions()
                .get::<CurrentUser>()
                .and_then(|cu| cu.0.role_enum())
                .map(|role| allowed.contains(&role))
                .unwrap_or(false);

            if ok {
                next.run(req).await
            } else {
                crate::errors::AppError::Forbidden(
                    "You don't have permission to access this page.".to_string(),
                )
                .into_response()
            }
        })
    }
}

pub async fn auth_middleware(
    Path(params): Path<HashMap<String, String>>,
    req: Request,
    next: Next,
) -> Response {
    let tenant_id = match params.get("tenant_id") {
        Some(t) => t.clone(),
        None => return (StatusCode::BAD_REQUEST, "Missing tenant").into_response(),
    };
    let login_url = format!("/web/{}/login", tenant_id);

    // Pool was injected by tenant_db_middleware (which must run BEFORE us).
    let pool = match req.extensions().get::<SqlitePool>().cloned() {
        Some(p) => p,
        None => {
            tracing::error!("auth_middleware: tenant pool missing from extensions");
            return (StatusCode::INTERNAL_SERVER_ERROR, "Server misconfigured").into_response();
        }
    };

    // Extract session id from cookie.
    let session_id = req
        .headers()
        .get(header::COOKIE)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| parse_cookie(s, SESSION_COOKIE_NAME));

    let session_id = match session_id {
        Some(id) => id,
        None => return redirect_to_login(&login_url, req.uri().path_and_query()),
    };

    // Validate against DB.
    let session = match Session::find_valid(&pool, &session_id).await {
        Ok(Some(s)) => s,
        _ => return redirect_to_login(&login_url, req.uri().path_and_query()),
    };

    let user = match User::find_by_id(&pool, &session.user_id).await {
        Ok(Some(u)) if u.is_active() => u,
        _ => return redirect_to_login(&login_url, req.uri().path_and_query()),
    };

    let mut req = req;
    req.extensions_mut().insert(CurrentUser(user));
    next.run(req).await
}

fn redirect_to_login(
    login_url: &str,
    path_and_query: Option<&axum::http::uri::PathAndQuery>,
) -> Response {
    match path_and_query {
        Some(pq) => {
            let next = url::form_urlencoded::byte_serialize(pq.as_str().as_bytes()).collect::<String>();
            Redirect::to(&format!("{}?next={}", login_url, next)).into_response()
        }
        None => Redirect::to(login_url).into_response(),
    }
}

/// Very small cookie-header parser. Handles `k=v; k2=v2; ...` — good enough
/// for a first-party session cookie. Avoids pulling in a bigger crate.
fn parse_cookie(header_value: &str, name: &str) -> Option<String> {
    for pair in header_value.split(';') {
        let pair = pair.trim();
        if let Some((k, v)) = pair.split_once('=') {
            if k.trim() == name {
                return Some(v.trim().to_string());
            }
        }
    }
    None
}
