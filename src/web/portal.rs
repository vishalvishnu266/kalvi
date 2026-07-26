//! Portal (parent/student self-service) web handlers.
//!
//! Student- and guardian-specific views used to live here and read from
//! the tenant DB via `services::people` / `services::guardians`. Those
//! services have been removed — a dedicated portal service layer will
//! replace them. For now the portal exposes only the shared
//! login/register/logout/home + tenant-link surfaces; the student &
//! guardian screens are stubbed out at the route layer.

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use askama::Template;
use axum::{
    body::Body,
    extract::{Form, State},
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Redirect, Response},
    Extension,
};
use serde::Deserialize;

use crate::http::{AppState, TenantScope};
use crate::middleware::auth::{read_cookie_from_headers, SessionUser};
use crate::models::auth::User;
use crate::services::{auth as auth_svc, system as sys_svc};
use crate::system::{NewPortalMembership, NewPortalUser};
use crate::tenancy::validate_tenant_id;
use crate::web::error::{render, WebError};

const COOKIE_PORTAL: &str = "erp_portal";

#[derive(Template)]
#[template(path = "portal/index.html")]
struct PortalHome<'a> {
    tenant_id: &'a str,
    user_display: &'a str,
}

#[derive(Template)]
#[template(path = "portal/login.html")]
struct PortalLoginPage<'a> {
    error: Option<&'a str>,
    identifier: &'a str,
}

#[derive(Template)]
#[template(path = "portal/register.html")]
struct PortalRegisterPage<'a> {
    error: Option<&'a str>,
    username: &'a str,
    email: &'a str,
}

#[derive(Template)]
#[template(path = "portal/home.html")]
struct PortalHomePage<'a> {
    user_display: &'a str,
    memberships: Vec<PortalMembershipView>,
    error: Option<&'a str>,
}

#[derive(Clone)]
struct PortalMembershipView {
    tenant_id: String,
    role: String,
}

#[derive(Deserialize)]
pub struct PortalLoginForm {
    identifier: String,
    password: String,
}

#[derive(Deserialize)]
pub struct PortalRegisterForm {
    username: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
pub struct LinkTenantForm {
    tenant: String,
    identifier: String,
    password: String,
}

pub async fn get_login() -> Result<Response, WebError> {
    render(&PortalLoginPage {
        error: None,
        identifier: "",
    })
}

pub async fn post_login(
    State(state): State<AppState>,
    Form(f): Form<PortalLoginForm>,
) -> Result<Response, WebError> {
    let Some(user) = sys_svc::find_portal_user_by_identifier(&state.system, &f.identifier).await?
    else {
        return render(&PortalLoginPage {
            error: Some("Invalid credentials"),
            identifier: &f.identifier,
        });
    };
    if !user.is_active || !verify_password(&f.password, &user.password_hash) {
        return render(&PortalLoginPage {
            error: Some("Invalid credentials"),
            identifier: &f.identifier,
        });
    }
    let set_cookie = format!(
        "{COOKIE_PORTAL}={}; Path=/; Max-Age={}; SameSite=Lax; HttpOnly",
        user.id,
        60 * 60 * 24 * 14
    );
    Ok((
        StatusCode::SEE_OTHER,
        [
            (header::SET_COOKIE, set_cookie),
            (header::LOCATION, "/portal".to_string()),
        ],
        Body::empty(),
    )
        .into_response())
}

pub async fn get_register() -> Result<Response, WebError> {
    render(&PortalRegisterPage {
        error: None,
        username: "",
        email: "",
    })
}

pub async fn post_register(
    State(state): State<AppState>,
    Form(f): Form<PortalRegisterForm>,
) -> Result<Response, WebError> {
    if f.password.len() < 8 {
        return render(&PortalRegisterPage {
            error: Some("Password must be at least 8 characters"),
            username: &f.username,
            email: &f.email,
        });
    }
    let password_hash = hash_password(&f.password)
        .map_err(|e| WebError::bad(format!("password hash failed: {e}")))?;
    let user = sys_svc::create_portal_user(
        &state.system,
        &NewPortalUser {
            username: f.username.trim().to_string(),
            email: f.email.trim().to_string(),
            password_hash,
        },
    )
    .await
    .map_err(|e| WebError::bad(e.to_string()))?;

    let set_cookie = format!(
        "{COOKIE_PORTAL}={}; Path=/; Max-Age={}; SameSite=Lax; HttpOnly",
        user.id,
        60 * 60 * 24 * 14
    );
    Ok((
        StatusCode::SEE_OTHER,
        [
            (header::SET_COOKIE, set_cookie),
            (header::LOCATION, "/portal".to_string()),
        ],
        Body::empty(),
    )
        .into_response())
}

pub async fn post_logout() -> Response {
    let clear_cookie = format!("{COOKIE_PORTAL}=; Path=/; Max-Age=0; SameSite=Lax; HttpOnly");
    (
        StatusCode::SEE_OTHER,
        [
            (header::SET_COOKIE, clear_cookie),
            (header::LOCATION, "/portal/login".to_string()),
        ],
        Body::empty(),
    )
        .into_response()
}

pub async fn home(State(state): State<AppState>, headers: HeaderMap) -> Result<Response, WebError> {
    let Some(uid) = portal_user_id_from_headers(&headers) else {
        return Ok(Redirect::to("/portal/login").into_response());
    };
    let user = sys_svc::get_portal_user(&state.system, uid)
        .await
        .map_err(|_| WebError::forbidden("invalid portal session"))?;

    let views = build_membership_views(&state, uid).await?;
    render(&PortalHomePage {
        user_display: &user.email,
        memberships: views,
        error: None,
    })
}

pub async fn post_link_tenant(
    State(state): State<AppState>,
    headers: HeaderMap,
    Form(f): Form<LinkTenantForm>,
) -> Result<Response, WebError> {
    let Some(portal_user_id) = portal_user_id_from_headers(&headers) else {
        return Ok(Redirect::to("/portal/login").into_response());
    };
    let tid = validate_tenant_id(f.tenant.trim().to_string())
        .map_err(|e| WebError::bad(e.to_string()))?;
    let pool = state
        .pool_for(&tid)
        .await
        .map_err(|e| WebError::bad(format!("tenant unavailable: {e}")))?;
    let user: User = auth_svc::login(&pool, &f.identifier, &f.password)
        .await
        .map_err(|_| WebError::forbidden("invalid tenant credentials"))?;
    let roles = auth_svc::roles_of(&pool, user.id).await?;
    let role = if roles.iter().any(|r| r.name == "guardian") {
        "guardian"
    } else if roles.iter().any(|r| r.name == "student") {
        "student"
    } else {
        return Err(WebError::forbidden(
            "tenant account must be guardian or student",
        ));
    };

    sys_svc::add_portal_membership(
        &state.system,
        &NewPortalMembership {
            portal_user_id,
            tenant_id: tid.as_str().to_string(),
            tenant_user_id: user.id,
            role: role.to_string(),
        },
    )
    .await?;

    Ok(Redirect::to("/portal").into_response())
}

pub async fn index(
    scope: TenantScope,
    Extension(session): Extension<SessionUser>,
) -> Result<Response, WebError> {
    render(&PortalHome {
        tenant_id: scope.tenant.as_str(),
        user_display: &session.display,
    })
}

async fn build_membership_views(
    state: &AppState,
    portal_user_id: i64,
) -> Result<Vec<PortalMembershipView>, WebError> {
    let memberships = sys_svc::list_portal_memberships(&state.system, portal_user_id).await?;
    let mut out = Vec::new();
    for m in memberships {
        if validate_tenant_id(m.tenant_id.clone()).is_err() {
            continue;
        }
        out.push(PortalMembershipView {
            tenant_id: m.tenant_id,
            role: m.role,
        });
    }
    Ok(out)
}

fn portal_user_id_from_headers(headers: &HeaderMap) -> Option<i64> {
    read_cookie_from_headers(headers, COOKIE_PORTAL).and_then(|v| v.parse::<i64>().ok())
}

fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| e.to_string())
}

fn verify_password(password: &str, hash: &str) -> bool {
    match PasswordHash::new(hash) {
        Ok(parsed) => Argon2::default()
            .verify_password(password.as_bytes(), &parsed)
            .is_ok(),
        Err(_) => false,
    }
}
