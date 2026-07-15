//! Admin-only User management (list / new / create / edit / update /
//! suspend / reactivate / reset-password).
//!
//! All routes are behind `require_role(&[Role::Admin])` in `routes.rs`.
use askama::Template;
use axum::{
    extract::{Form, Path, Query},
    response::{Html, IntoResponse, Redirect, Response},
    Extension,
};
use sqlx::SqlitePool;

use crate::auth_middleware::CurrentUser;
use crate::errors::AppError;
use crate::models::user::{Role, User, UserFilters, UserForm};
use crate::utils::crud::friendly_db_error;
use crate::utils::page::PageChrome;

const ROLE_OPTIONS: &[(&str, &str)] = &[
    ("admin", "Admin"),
    ("teacher", "Teacher"),
    ("accountant", "Accountant"),
    ("librarian", "Librarian"),
    ("student", "Student"),
    ("guardian", "Guardian"),
];

const STATUS_OPTIONS: &[(&str, &str)] = &[
    ("active", "Active"),
    ("suspended", "Suspended"),
];

// ------------ Templates ------------

#[derive(Template)]
#[template(path = "settings/users/index.html")]
struct IndexTpl {
    chrome: PageChrome,
    tenant_id: String,
    users: Vec<User>,
    filters: UserFilters,
    role_options: &'static [(&'static str, &'static str)],
    status_options: &'static [(&'static str, &'static str)],
    /// Id of the currently signed-in user — used to hide "suspend" / "delete"
    /// actions on their own row so they can't lock themselves out.
    self_id: String,
}

#[derive(Template)]
#[template(path = "settings/users/form.html")]
struct FormTpl {
    chrome: PageChrome,
    tenant_id: String,
    is_edit: bool,
    user_row: User, // renamed from `user` to avoid shadowing chrome context
    error: Option<String>,
    role_options: &'static [(&'static str, &'static str)],
    status_options: &'static [(&'static str, &'static str)],
}

// ------------ List ------------

pub async fn list_handler(
    Path(tenant_id): Path<String>,
    Query(filters): Query<UserFilters>,
    Extension(pool): Extension<SqlitePool>,
    Extension(cu): Extension<CurrentUser>,
) -> Result<Html<String>, AppError> {
    let chrome = PageChrome::load(&pool, &tenant_id, "settings", &cu).await?;
    let users = User::list(&pool, &filters).await?;
    let self_id = cu.0.id.clone();
    let tpl = IndexTpl {
        chrome,
        tenant_id,
        users,
        filters,
        role_options: ROLE_OPTIONS,
        status_options: STATUS_OPTIONS,
        self_id,
    };
    Ok(Html(tpl.render()?))
}

// ------------ New / Edit form ------------

fn blank_user() -> User {
    User {
        role: "teacher".into(),
        status: "active".into(),
        ..Default::default()
    }
}

pub async fn new_handler(
    Path(tenant_id): Path<String>,
    Extension(pool): Extension<SqlitePool>,
    Extension(cu): Extension<CurrentUser>,
) -> Result<Html<String>, AppError> {
    let chrome = PageChrome::load(&pool, &tenant_id, "settings", &cu).await?;
    let tpl = FormTpl {
        chrome,
        tenant_id,
        is_edit: false,
        user_row: blank_user(),
        error: None,
        role_options: ROLE_OPTIONS,
        status_options: STATUS_OPTIONS,
    };
    Ok(Html(tpl.render()?))
}

pub async fn edit_handler(
    Path((tenant_id, id)): Path<(String, String)>,
    Extension(pool): Extension<SqlitePool>,
    Extension(cu): Extension<CurrentUser>,
) -> Result<Response, AppError> {
    let user_row = match User::find_by_id(&pool, &id).await? {
        Some(u) => u,
        None => {
            return Ok(Redirect::to(&format!("/web/{}/settings/users", tenant_id))
                .into_response());
        }
    };
    let chrome = PageChrome::load(&pool, &tenant_id, "settings", &cu).await?;
    let tpl = FormTpl {
        chrome,
        tenant_id,
        is_edit: true,
        user_row,
        error: None,
        role_options: ROLE_OPTIONS,
        status_options: STATUS_OPTIONS,
    };
    Ok(Html(tpl.render()?).into_response())
}

// ------------ Create / Update ------------

pub async fn create_handler(
    Path(tenant_id): Path<String>,
    Extension(pool): Extension<SqlitePool>,
    Extension(cu): Extension<CurrentUser>,
    Form(form): Form<UserForm>,
) -> Result<Response, AppError> {
    if let Err(msg) = form.validate(false) {
        return render_form_error(&pool, &cu, tenant_id, false, None, form, msg).await;
    }
    let role = Role::from_str_opt(&form.role).unwrap(); // validated above

    match User::create(&pool, &form.email, &form.password, &form.full_name, role).await {
        Ok(_) => Ok(Redirect::to(&format!("/web/{}/settings/users", tenant_id)).into_response()),
        Err(AppError::Database(e)) => {
            let msg = friendly_db_error(
                &e,
                &[("users.email", "That email is already in use.")],
            );
            render_form_error(&pool, &cu, tenant_id, false, None, form, msg).await
        }
        Err(other) => Err(other),
    }
}

pub async fn update_handler(
    Path((tenant_id, id)): Path<(String, String)>,
    Extension(pool): Extension<SqlitePool>,
    Extension(cu): Extension<CurrentUser>,
    Form(form): Form<UserForm>,
) -> Result<Response, AppError> {
    if let Err(msg) = form.validate(true) {
        return render_form_error(&pool, &cu, tenant_id, true, Some(id), form, msg).await;
    }
    let role = Role::from_str_opt(&form.role).unwrap();

    // Prevent an admin from demoting or suspending themselves — otherwise
    // they could accidentally lock the ONLY admin out of the tenant.
    if id == cu.0.id {
        if role != Role::Admin {
            let msg = "You can't change your own role.".to_string();
            return render_form_error(&pool, &cu, tenant_id, true, Some(id), form, msg).await;
        }
        if form.status == "suspended" {
            let msg = "You can't suspend your own account.".to_string();
            return render_form_error(&pool, &cu, tenant_id, true, Some(id), form, msg).await;
        }
    }

    let res = User::update_profile(&pool, &id, &form.email, &form.full_name, role, &form.status)
        .await;
    if let Err(AppError::Database(e)) = res {
        let msg = friendly_db_error(&e, &[("users.email", "That email is already in use.")]);
        return render_form_error(&pool, &cu, tenant_id, true, Some(id), form, msg).await;
    }
    res?;

    // Optionally reset password if a new one was provided.
    if !form.password.trim().is_empty() {
        User::set_password(&pool, &id, &form.password).await?;
    }

    Ok(Redirect::to(&format!("/web/{}/settings/users", tenant_id)).into_response())
}

// ------------ Suspend / Reactivate / Reset-password ------------

pub async fn suspend_handler(
    Path((tenant_id, id)): Path<(String, String)>,
    Extension(pool): Extension<SqlitePool>,
    Extension(cu): Extension<CurrentUser>,
) -> Result<Response, AppError> {
    if id == cu.0.id {
        return Err(AppError::Forbidden(
            "You can't suspend your own account.".to_string(),
        ));
    }
    User::suspend(&pool, &id).await?;
    Ok(Redirect::to(&format!("/web/{}/settings/users", tenant_id)).into_response())
}

pub async fn reactivate_handler(
    Path((tenant_id, id)): Path<(String, String)>,
    Extension(pool): Extension<SqlitePool>,
) -> Result<Response, AppError> {
    User::reactivate(&pool, &id).await?;
    Ok(Redirect::to(&format!("/web/{}/settings/users", tenant_id)).into_response())
}

// ------------ Helpers ------------

async fn render_form_error(
    pool: &SqlitePool,
    cu: &CurrentUser,
    tenant_id: String,
    is_edit: bool,
    id: Option<String>,
    form: UserForm,
    error: String,
) -> Result<Response, AppError> {
    let mut user_row = blank_user();
    user_row.email = form.email;
    user_row.full_name = form.full_name;
    user_row.role = form.role;
    if !form.status.is_empty() {
        user_row.status = form.status;
    }
    if let Some(id) = id {
        user_row.id = id;
    }

    let chrome = PageChrome::load(pool, &tenant_id, "settings", cu).await?;
    let tpl = FormTpl {
        chrome,
        tenant_id,
        is_edit,
        user_row,
        error: Some(error),
        role_options: ROLE_OPTIONS,
        status_options: STATUS_OPTIONS,
    };
    Ok(Html(tpl.render()?).into_response())
}
