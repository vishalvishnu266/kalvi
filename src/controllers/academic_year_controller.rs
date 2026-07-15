//! CRUD for the tenant's Academic Years.
use askama::Template;
use axum::{
    extract::{Form, Path},
    response::{Html, IntoResponse, Redirect, Response},
    Extension,
};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::auth_middleware::CurrentUser;
use crate::errors::AppError;
use crate::models::academic_year::{AcademicYear, AcademicYearForm};
use crate::models::user::User;
use crate::utils::crud::friendly_db_error;
use crate::utils::page::PageChrome;

// ------------ Templates ------------

#[derive(Template)]
#[template(path = "settings/academic_years/index.html")]
struct IndexTpl {
    chrome: PageChrome,
    tenant_id: String,
    years: Vec<AcademicYear>,
}

#[derive(Template)]
#[template(path = "settings/academic_years/form.html")]
struct FormTpl {
    chrome: PageChrome,
    tenant_id: String,
    is_edit: bool,
    year: AcademicYear,
    error: Option<String>,
}

// ------------ List ------------

pub async fn list_handler(
    Path(tenant_id): Path<String>,
    Extension(pool): Extension<SqlitePool>,
    Extension(cu): Extension<CurrentUser>,
) -> Result<Html<String>, AppError> {
    let chrome = PageChrome::load(&pool, &tenant_id, "settings", &cu).await?;
    let years = AcademicYear::list_all(&pool).await?;
    let tpl = IndexTpl {
        chrome,
        tenant_id,
        years,
    };
    Ok(Html(tpl.render()?))
}

// ------------ New / Edit form ------------

fn blank_year() -> AcademicYear {
    AcademicYear {
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
        year: blank_year(),
        error: None,
    };
    Ok(Html(tpl.render()?))
}

pub async fn edit_handler(
    Path((tenant_id, id)): Path<(String, String)>,
    Extension(pool): Extension<SqlitePool>,
    Extension(cu): Extension<CurrentUser>,
) -> Result<Response, AppError> {
    let year = match AcademicYear::find(&pool, &id).await? {
        Some(y) => y,
        None => {
            return Ok(Redirect::to(&format!(
                "/web/{}/settings/academic-years",
                tenant_id
            ))
            .into_response())
        }
    };
    let chrome = PageChrome::load(&pool, &tenant_id, "settings", &cu).await?;
    let tpl = FormTpl {
        chrome,
        tenant_id,
        is_edit: true,
        year,
        error: None,
    };
    Ok(Html(tpl.render()?).into_response())
}

// ------------ Create / Update / Delete / SetCurrent ------------

pub async fn create_handler(
    Path(tenant_id): Path<String>,
    Extension(pool): Extension<SqlitePool>,
    Extension(CurrentUser(user)): Extension<CurrentUser>,
    Form(form): Form<AcademicYearForm>,
) -> Result<Response, AppError> {
    if let Err(msg) = form.validate() {
        return render_form_error(&pool, &user, tenant_id, false, form, msg).await;
    }

    let id = Uuid::new_v4().to_string();
    let res = sqlx::query(
        r#"INSERT INTO academic_years (id, name, start_date, end_date, is_current, status)
           VALUES (?, ?, ?, ?, 0, ?)"#,
    )
    .bind(&id)
    .bind(form.name.trim())
    .bind(form.start_date.trim())
    .bind(form.end_date.trim())
    .bind(form.effective_status())
    .execute(&pool)
    .await;

    if let Err(e) = res {
        let msg = friendly_db_error(&e, &[("academic_years.name", "That academic year name already exists.")]);
        return render_form_error(&pool, &user, tenant_id, false, form, msg).await;
    }

    Ok(Redirect::to(&format!("/web/{}/settings/academic-years", tenant_id)).into_response())
}

pub async fn update_handler(
    Path((tenant_id, id)): Path<(String, String)>,
    Extension(pool): Extension<SqlitePool>,
    Extension(CurrentUser(user)): Extension<CurrentUser>,
    Form(form): Form<AcademicYearForm>,
) -> Result<Response, AppError> {
    if let Err(msg) = form.validate() {
        return render_form_error_edit(&pool, &user, tenant_id, id, form, msg).await;
    }

    let res = sqlx::query(
        r#"UPDATE academic_years
           SET name = ?, start_date = ?, end_date = ?, status = ?, updated_at = CURRENT_TIMESTAMP
           WHERE id = ?"#,
    )
    .bind(form.name.trim())
    .bind(form.start_date.trim())
    .bind(form.end_date.trim())
    .bind(form.effective_status())
    .bind(&id)
    .execute(&pool)
    .await;

    if let Err(e) = res {
        let msg = friendly_db_error(&e, &[("academic_years.name", "That academic year name already exists.")]);
        return render_form_error_edit(&pool, &user, tenant_id, id, form, msg).await;
    }

    Ok(Redirect::to(&format!("/web/{}/settings/academic-years", tenant_id)).into_response())
}

pub async fn delete_handler(
    Path((tenant_id, id)): Path<(String, String)>,
    Extension(pool): Extension<SqlitePool>,
) -> Result<Response, AppError> {
    // Refuse to delete the current AY (safety guard).
    if let Some(y) = AcademicYear::find(&pool, &id).await? {
        if y.is_current_bool() {
            return Ok(Redirect::to(&format!(
                "/web/{}/settings/academic-years",
                tenant_id
            ))
            .into_response());
        }
    }
    sqlx::query("DELETE FROM academic_years WHERE id = ?")
        .bind(&id)
        .execute(&pool)
        .await?;
    Ok(Redirect::to(&format!("/web/{}/settings/academic-years", tenant_id)).into_response())
}

pub async fn set_current_handler(
    Path((tenant_id, id)): Path<(String, String)>,
    Extension(pool): Extension<SqlitePool>,
) -> Result<Response, AppError> {
    AcademicYear::set_current(&pool, &id).await?;
    Ok(Redirect::to(&format!("/web/{}/settings/academic-years", tenant_id)).into_response())
}

// ------------ Local helpers ------------

async fn render_form_error(
    pool: &SqlitePool,
    user: &User,
    tenant_id: String,
    is_edit: bool,
    form: AcademicYearForm,
    error: String,
) -> Result<Response, AppError> {
    let year = AcademicYear {
        name: form.name,
        start_date: form.start_date,
        end_date: form.end_date,
        status: form.status,
        ..blank_year()
    };
    let chrome = PageChrome::load_with_user(pool, &tenant_id, "settings", user).await?;
    let tpl = FormTpl {
        chrome,
        tenant_id,
        is_edit,
        year,
        error: Some(error),
    };
    Ok(Html(tpl.render()?).into_response())
}

async fn render_form_error_edit(
    pool: &SqlitePool,
    user: &User,
    tenant_id: String,
    id: String,
    form: AcademicYearForm,
    error: String,
) -> Result<Response, AppError> {
    let year = AcademicYear {
        id,
        name: form.name,
        start_date: form.start_date,
        end_date: form.end_date,
        status: form.status,
        ..blank_year()
    };
    let chrome = PageChrome::load_with_user(pool, &tenant_id, "settings", user).await?;
    let tpl = FormTpl {
        chrome,
        tenant_id,
        is_edit: true,
        year,
        error: Some(error),
    };
    Ok(Html(tpl.render()?).into_response())
}
