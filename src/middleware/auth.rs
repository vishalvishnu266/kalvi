use std::collections::{HashMap, HashSet};
use axum::{
    body::Body,
    extract::State,
    http::{header, HeaderMap, Request as AxumRequest},
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use axum::extract::Path;
use crate::http::AppState;
use crate::tenancy::{tenant_services_for, TenantId};

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
///
/// `roles` and `permissions` are hydrated once per request from the tenant DB
/// so downstream handlers, templates, and nav filters can call `has(...)`
/// without additional queries.
#[derive(Clone, Debug)]
pub struct SessionUser {
    pub user_id: i64,
    pub username: String,
    pub display: String,
    pub session_id: i64,
    /// Role names the user holds (e.g. `["admin"]`, `["guardian"]`,
    /// `["teacher","class_teacher"]`).
    pub roles: Vec<String>,
    /// Flattened permission codes granted via all held roles.
    pub permissions: HashSet<String>,
}

impl SessionUser {
    /// True if the user has the exact permission code.
    pub fn has(&self, code: &str) -> bool {
        self.permissions.contains(code)
    }

    /// True if the user has *any* of the given permission codes. Convenient
    /// for screens that accept either a "view" or a "view_own" variant.
    pub fn any_of(&self, codes: &[&str]) -> bool {
        codes.iter().any(|c| self.has(c))
    }

    /// True if the user holds the named role.
    pub fn is_role(&self, r: &str) -> bool {
        self.roles.iter().any(|x| x == r)
    }
}

/// Auth gate for the tenant-scoped app shell (`/web/{tenant}/…`).
///
/// Rules, evaluated top-down:
/// 1. Pull the `{tenant}` segment from the URL. If absent, we can't scope
///    the check — redirect to the global login.
/// 2. Require an `erp_session` cookie. Missing → redirect to
///    `/web/{tenant}/login`.
/// 3. Resolve the session against the tenant-scoped session backend.
///    Unknown / revoked / expired → redirect to login.
/// 4. Stash the resolved user into the request extensions so downstream
///    handlers can read it via [`SessionUser`] without a second DB hit.
// Shell guards moved to src/http/{web,portal}/middleware.rs
// global check_perm remains for require_perm! macro

pub async fn check_perm(
    codes: &'static [&'static str],
    req: AxumRequest<Body>,
    next: Next,
) -> Response {
    let allowed = req
        .extensions()
        .get::<SessionUser>()
        .map(|s| s.any_of(codes))
        .unwrap_or(false);
    if allowed {
        next.run(req).await
    } else {
        let path = req.uri().path().to_string();
        forbidden_response(&path, codes)
    }
}

/// Ergonomic wrapper around [`check_perm`] for use with `.route_layer(...)`.
///
/// Expands to a fresh `axum::middleware::from_fn(...)` closure per invocation,
/// which sidesteps the "closure captures a non-'static reference" issue that
/// pops up when trying to build the same layer through a plain function.
///
/// # Example
/// ```ignore
/// use crate::{middleware::auth::require_perm, services::perm};
///
/// Router::new()
///     .route("/staff", get(handler))
///     .route_layer(require_perm!(perm::STAFF_VIEW));
///
/// // Multiple codes — passes if session has *any* of them:
/// .route_layer(require_perm!(perm::FEES_VIEW, perm::FEES_VIEW_OWN))
/// ```
#[macro_export]
macro_rules! require_perm {
    ( $($code:expr),+ $(,)? ) => {{
        const CODES: &[&str] = &[ $($code),+ ];
        ::axum::middleware::from_fn(
            move |req: ::axum::extract::Request, next: ::axum::middleware::Next| async move {
                $crate::middleware::auth::check_perm(CODES, req, next).await
            },
        )
    }};
}

fn forbidden_response(path: &str, missing: &[&str]) -> Response {
    use axum::http::StatusCode;
    if path.starts_with("/api/") {
        let body = serde_json::json!({
            "error": "forbidden",
            "missing_any_of": missing,
        });
        (StatusCode::FORBIDDEN, axum::Json(body)).into_response()
    } else {
        let missing_html = missing.join(", ");
        let html = format!(
            r#"<!doctype html>
<html lang="en"><head><meta charset="utf-8">
<title>403 · Access denied</title>
<style>
  body {{ font-family: system-ui, sans-serif; background:#fafafa; color:#111; margin:0;
         display:flex; align-items:center; justify-content:center; min-height:100vh; }}
  main {{ max-width: 32rem; padding: 2rem; text-align:center; }}
  h1   {{ font-size: 2.25rem; margin: 0 0 .5rem; }}
  p    {{ color:#555; }}
  code {{ background:#eee; padding: .1rem .35rem; border-radius: .25rem; font-size: .9em; }}
  a    {{ color:#4f46e5; text-decoration:none; }}
</style></head>
<body><main>
  <h1>403 · Access denied</h1>
  <p>Your account doesn't have permission to open this page.</p>
  <p><small>Required (any of): <code>{missing_html}</code></small></p>
  <p><a href="/">← Back to home</a></p>
</main></body></html>"#
        );
        (StatusCode::FORBIDDEN,
         [(axum::http::header::CONTENT_TYPE, "text/html; charset=utf-8")],
         html).into_response()
    }
}

fn url_tenant_from_prefix(path: &str, prefix: &str) -> Option<String> {
    let rest = path.strip_prefix(prefix)?;
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
