//! Login / logout handlers.
//!
//! These routes live INSIDE the tenant scope (`/web/{tenant}/login`) so the
//! tenant DB pool is available, but they must NOT go through
//! `auth_middleware` — otherwise unauthenticated users would be redirected in
//! a loop.

use askama::Template;
use axum::{
    extract::{Form, Path, Query},
    http::{header, HeaderMap, HeaderValue},
    response::{Html, IntoResponse, Redirect, Response},
    Extension,
};
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::csrf_middleware::CSRF_TOKEN_VALUE;
use crate::errors::AppError;
use crate::models::session::{Session, SESSION_COOKIE_NAME, SESSION_LIFETIME_DAYS};
use crate::models::user::{verify_password, User};

// -------------------------------------------------------------------
// GET /web/{tenant}/login
// -------------------------------------------------------------------
#[derive(Template)]
#[template(path = "auth/login.html")]
struct LoginTpl<'a> {
    tenant_id: String,
    csrf_token: &'a str,
    error: Option<String>,
    email: String,
    next: String, // preserved via hidden input so we redirect there after login
}

#[derive(Debug, Deserialize, Default)]
pub struct LoginQuery {
    #[serde(default)]
    pub next: Option<String>,
    /// Present when the auth middleware redirected due to expiry.
    #[serde(default)]
    pub expired: Option<String>,
}

pub async fn login_page_handler(
    Path(tenant_id): Path<String>,
    Extension(pool): Extension<SqlitePool>,
    Query(q): Query<LoginQuery>,
    headers: HeaderMap,
) -> Result<Response, AppError> {
    // If the caller already has a valid session cookie, skip the form.
    if let Some(sid) = extract_cookie(&headers, SESSION_COOKIE_NAME) {
        if let Ok(Some(_)) = Session::find_valid(&pool, &sid).await {
            return Ok(Redirect::to(&format!("/web/{}/dashboard", tenant_id)).into_response());
        }
    }

    let tpl = LoginTpl {
        tenant_id,
        csrf_token: CSRF_TOKEN_VALUE,
        error: q.expired.map(|_| "Your session expired — please sign in again.".to_string()),
        email: String::new(),
        next: q.next.unwrap_or_default(),
    };
    Ok(Html(tpl.render()?).into_response())
}

// -------------------------------------------------------------------
// POST /web/{tenant}/login
// -------------------------------------------------------------------
#[derive(Debug, Deserialize)]
pub struct LoginForm {
    pub csrf_token: Option<String>,
    pub email: String,
    pub password: String,
    #[serde(default)]
    pub next: String,
}

pub async fn login_submit_handler(
    Path(tenant_id): Path<String>,
    Extension(pool): Extension<SqlitePool>,
    headers: HeaderMap,
    Form(form): Form<LoginForm>,
) -> Result<Response, AppError> {
    // Look up user. NEVER differentiate "no such user" from "wrong password"
    // in the response — that would leak whether an email is registered.
    let user_opt = User::find_by_email(&pool, &form.email).await?;
    let generic_error = || "Invalid email or password.".to_string();

    let user = match user_opt {
        Some(u) if u.is_active() => u,
        _ => return render_login_error(tenant_id, form, generic_error()),
    };

    let ok = verify_password(&form.password, &user.password_hash).unwrap_or(false);
    if !ok {
        return render_login_error(tenant_id, form, generic_error());
    }

    // Success → create session, set cookie, redirect.
    let ip = headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.split(',').next().unwrap_or(s).trim().to_string());
    let ua = headers
        .get(header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.chars().take(255).collect::<String>());

    let sid = Session::create(&pool, &user.id, ip.as_deref(), ua.as_deref()).await?;
    User::touch_last_login(&pool, &user.id).await?;

    // Determine redirect target: sanitized "next" or dashboard.
    let target = sanitize_next(&form.next, &tenant_id)
        .unwrap_or_else(|| format!("/web/{}/dashboard", tenant_id));

    let mut resp = Redirect::to(&target).into_response();
    resp.headers_mut().insert(
        header::SET_COOKIE,
        HeaderValue::from_str(&build_session_cookie(&sid, false))
            .map_err(|_| AppError::Unexpected("bad cookie".into()))?,
    );
    Ok(resp)
}

fn render_login_error(tenant_id: String, form: LoginForm, error: String) -> Result<Response, AppError> {
    let tpl = LoginTpl {
        tenant_id,
        csrf_token: CSRF_TOKEN_VALUE,
        error: Some(error),
        email: form.email,
        next: form.next,
    };
    Ok(Html(tpl.render()?).into_response())
}

// -------------------------------------------------------------------
// POST /web/{tenant}/logout
// -------------------------------------------------------------------
pub async fn logout_handler(
    Path(tenant_id): Path<String>,
    Extension(pool): Extension<SqlitePool>,
    headers: HeaderMap,
) -> Result<Response, AppError> {
    if let Some(sid) = extract_cookie(&headers, SESSION_COOKIE_NAME) {
        let _ = Session::delete(&pool, &sid).await; // ignore errors — cookie is going away regardless
    }
    let mut resp = Redirect::to(&format!("/web/{}/login", tenant_id)).into_response();
    resp.headers_mut().insert(
        header::SET_COOKIE,
        HeaderValue::from_str(&build_session_cookie("", true))
            .map_err(|_| AppError::Unexpected("bad cookie".into()))?,
    );
    Ok(resp)
}

// -------------------------------------------------------------------
// Cookie helpers
// -------------------------------------------------------------------

/// Build a `Set-Cookie` value. If `delete` is true, sets an expired cookie
/// to instruct the browser to remove it.
fn build_session_cookie(value: &str, delete: bool) -> String {
    // NOTE: We deliberately omit `Secure` so this works over plain HTTP in
    // dev. Add it once the app runs behind TLS.
    let max_age = if delete { 0 } else { 60 * 60 * 24 * SESSION_LIFETIME_DAYS };
    format!(
        "{}={}; Path=/; HttpOnly; SameSite=Lax; Max-Age={}",
        SESSION_COOKIE_NAME, value, max_age
    )
}

fn extract_cookie(headers: &HeaderMap, name: &str) -> Option<String> {
    let v = headers.get(header::COOKIE)?.to_str().ok()?;
    for pair in v.split(';') {
        let pair = pair.trim();
        if let Some((k, val)) = pair.split_once('=') {
            if k.trim() == name {
                return Some(val.trim().to_string());
            }
        }
    }
    None
}

/// Only accept internal, absolute paths under the current tenant to avoid
/// open-redirect vulnerabilities via a crafted `?next=https://evil`.
fn sanitize_next(raw: &str, tenant_id: &str) -> Option<String> {
    let decoded = url::form_urlencoded::parse(raw.as_bytes())
        .map(|(k, v)| if k.is_empty() { v.to_string() } else { format!("{k}={v}") })
        .next()
        .unwrap_or_else(|| raw.to_string());
    let candidate = if decoded.is_empty() { raw.to_string() } else { decoded };
    if candidate.starts_with(&format!("/web/{}/", tenant_id))
        // Refuse protocol-relative or scheme-carrying targets.
        && !candidate.starts_with("//")
        && !candidate.contains("://")
    {
        Some(candidate)
    } else {
        None
    }
}
