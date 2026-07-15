//! Self-service account routes — available to every authenticated user.
//!
//!   GET  /web/{tenant}/account            -> profile page
//!   POST /web/{tenant}/account/password   -> change my password
//!   POST /web/{tenant}/account/profile    -> edit my full_name / email
use askama::Template;
use axum::{
    extract::{Form, Path},
    response::{Html, IntoResponse, Redirect, Response},
    Extension,
};
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::auth_middleware::CurrentUser;
use crate::errors::AppError;
use crate::models::user::{verify_password, ChangePasswordForm, User};
use crate::utils::crud::friendly_db_error;
use crate::utils::page::PageChrome;

#[derive(Template)]
#[template(path = "account/profile.html")]
struct ProfileTpl {
    chrome: PageChrome,
    tenant_id: String,

    profile_form: ProfileForm,
    profile_error: Option<String>,
    profile_success: Option<String>,

    password_error: Option<String>,
    password_success: Option<String>,
}

// -------------------------------------------------------------------
// GET /account
// -------------------------------------------------------------------
pub async fn profile_handler(
    Path(tenant_id): Path<String>,
    Extension(pool): Extension<SqlitePool>,
    Extension(cu): Extension<CurrentUser>,
) -> Result<Html<String>, AppError> {
    render_profile(&pool, &cu, tenant_id, ProfilePageState::default()).await
}

// -------------------------------------------------------------------
// POST /account/profile
// -------------------------------------------------------------------
#[derive(Debug, Default, Deserialize)]
pub struct ProfileForm {
    pub csrf_token: Option<String>,
    #[serde(default)]
    pub full_name: String,
    #[serde(default)]
    pub email: String,
}

impl ProfileForm {
    fn validate(&self) -> Result<(), String> {
        if self.full_name.trim().is_empty() {
            return Err("Full name is required.".into());
        }
        if self.email.trim().is_empty() {
            return Err("Email is required.".into());
        }
        if !self.email.contains('@') {
            return Err("Please enter a valid email address.".into());
        }
        Ok(())
    }
}

pub async fn update_profile_handler(
    Path(tenant_id): Path<String>,
    Extension(pool): Extension<SqlitePool>,
    Extension(cu): Extension<CurrentUser>,
    Form(form): Form<ProfileForm>,
) -> Result<Response, AppError> {
    if let Err(msg) = form.validate() {
        return render_profile(
            &pool,
            &cu,
            tenant_id,
            ProfilePageState {
                profile_form: form,
                profile_error: Some(msg),
                ..Default::default()
            },
        )
        .await
        .map(IntoResponse::into_response);
    }

    // Preserve current role & status; user cannot edit these here.
    let user = &cu.0;
    let role = user.role_enum().unwrap_or(crate::models::user::Role::Teacher);
    let res =
        User::update_profile(&pool, &user.id, &form.email, &form.full_name, role, &user.status)
            .await;

    if let Err(AppError::Database(e)) = res {
        let msg = friendly_db_error(&e, &[("users.email", "That email is already in use.")]);
        return render_profile(
            &pool,
            &cu,
            tenant_id,
            ProfilePageState {
                profile_form: form,
                profile_error: Some(msg),
                ..Default::default()
            },
        )
        .await
        .map(IntoResponse::into_response);
    }
    res?;

    Ok(Redirect::to(&format!("/web/{}/account", tenant_id)).into_response())
}

// -------------------------------------------------------------------
// POST /account/password
// -------------------------------------------------------------------
pub async fn change_password_handler(
    Path(tenant_id): Path<String>,
    Extension(pool): Extension<SqlitePool>,
    Extension(cu): Extension<CurrentUser>,
    Form(form): Form<ChangePasswordForm>,
) -> Result<Response, AppError> {
    if let Err(msg) = form.validate() {
        return render_profile(
            &pool,
            &cu,
            tenant_id,
            ProfilePageState {
                password_error: Some(msg),
                ..Default::default()
            },
        )
        .await
        .map(IntoResponse::into_response);
    }

    // Verify the old password. Never leak whether the user exists — we always do.
    let ok = verify_password(&form.current_password, &cu.0.password_hash).unwrap_or(false);
    if !ok {
        return render_profile(
            &pool,
            &cu,
            tenant_id,
            ProfilePageState {
                password_error: Some("Current password is incorrect.".into()),
                ..Default::default()
            },
        )
        .await
        .map(IntoResponse::into_response);
    }

    User::set_password(&pool, &cu.0.id, &form.new_password).await?;

    render_profile(
        &pool,
        &cu,
        tenant_id,
        ProfilePageState {
            password_success: Some("Password updated successfully.".into()),
            ..Default::default()
        },
    )
    .await
    .map(IntoResponse::into_response)
}

// -------------------------------------------------------------------
// Helpers
// -------------------------------------------------------------------

#[derive(Default)]
struct ProfilePageState {
    profile_form: ProfileForm,
    profile_error: Option<String>,
    profile_success: Option<String>,
    password_error: Option<String>,
    password_success: Option<String>,
}

async fn render_profile(
    pool: &SqlitePool,
    cu: &CurrentUser,
    tenant_id: String,
    mut state: ProfilePageState,
) -> Result<Html<String>, AppError> {
    // If the profile form was NOT submitted this round, prefill from the user.
    if state.profile_form.full_name.is_empty() && state.profile_form.email.is_empty() {
        state.profile_form.full_name = cu.0.full_name.clone();
        state.profile_form.email = cu.0.email.clone();
    }

    let chrome = PageChrome::load(pool, &tenant_id, "account", cu).await?;
    let tpl = ProfileTpl {
        chrome,
        tenant_id,
        profile_form: state.profile_form,
        profile_error: state.profile_error,
        profile_success: state.profile_success,
        password_error: state.password_error,
        password_success: state.password_success,
    };
    Ok(Html(tpl.render()?))
}
