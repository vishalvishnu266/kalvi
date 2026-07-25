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
use crate::services::auth as auth_svc;
use crate::tenancy::TenantId;
use crate::web::error::{render, WebError};

pub const COOKIE_TENANT: &str = crate::middleware::auth::COOKIE_TENANT;
pub const COOKIE_USER: &str = crate::middleware::auth::COOKIE_USER;
pub const COOKIE_SESSION: &str = crate::middleware::auth::COOKIE_SESSION;

#[derive(Template)]
#[template(path = "login.html")]
struct LoginPage<'a> {
    error: Option<&'a str>,
    tenant: &'a str,
    identifier: &'a str,
    tenant_locked: bool,
}

pub async fn get_login() -> Result<Response, WebError> {
    render(&LoginPage { error: None, tenant: "", identifier: "", tenant_locked: false })
}

pub async fn get_tenant_login(Path(tenant): Path<String>) -> Result<Response, WebError> {
    render(&LoginPage { error: None, tenant: &tenant, identifier: "", tenant_locked: true })
}

pub async fn get_portal_tenant_login(Path(tenant): Path<String>) -> Result<Response, WebError> {
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
    post_login_with_redirect(state, f, "/web").await
}

pub async fn post_portal_login(
    State(state): State<AppState>,
    Form(f): Form<LoginForm>,
) -> Result<Response, WebError> {
    post_login_with_redirect(state, f, "/portal").await
}

async fn post_login_with_redirect(
    state: AppState,
    f: LoginForm,
    base_path: &str,
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
    let pool = match state.pool_for(&tenant).await {
        Ok(p) => p,
        Err(e) => {
            return render(&LoginPage {
                error: Some(&format!("Tenant lookup failed: {e}")),
                tenant: &f.tenant, identifier: &f.identifier,
                tenant_locked: false,
            });
        }
    };

    match auth_svc::login(&pool, &f.identifier, &f.password).await {
        Ok(user) => {
            let session = match auth_svc::issue_session(&state.sessions, user.id, None, None).await {
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
            let location = format!("{}/{}/", base_path, tenant.as_str());
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

pub async fn post_logout(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Response {
    let cookie_tenant  = read_cookie_from_headers(&headers, crate::middleware::auth::COOKIE_TENANT);
    let cookie_session = read_cookie_from_headers(&headers, crate::middleware::auth::COOKIE_SESSION);
    if let (Some(tid), Some(token)) = (cookie_tenant, cookie_session) {
        if let Ok(t) = TenantId::new(tid) {
            if let Ok(_pool) = state.pool_for(&t).await {
                let _ = auth_svc::revoke_session(&state.sessions, &token).await;
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
