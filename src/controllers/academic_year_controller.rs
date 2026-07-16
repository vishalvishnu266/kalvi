use askama::Template;
use axum::{
    extract::{Form, Path},
    response::{Html, IntoResponse, Redirect, Response},
    Extension,
};
use sqlx::SqlitePool;

use crate::csrf_middleware::CSRF_TOKEN_VALUE;
use crate::errors::AppError;
use crate::models::academic_year::{AcademicYear, AcademicYearForm};
use crate::services::academic_year_service::AcademicYearService;
use crate::services::student_service::StudentService;
use crate::utils::crud::friendly_db_error;

// ------------ Templates ------------

#[derive(Template)]
#[template(path = "settings/academic_years/index.html")]
struct IndexTpl<'a> {
    tenant_id: String,
    active: &'static str,
    csrf_token: &'a str,
    student_count: i64,
    years: Vec<AcademicYear>,
}

#[derive(Template)]
#[template(path = "settings/academic_years/form.html")]
struct FormTpl<'a> {
    tenant_id: String,
    active: &'static str,
    csrf_token: &'a str,
    student_count: i64,
    is_edit: bool,
    year: AcademicYear,
    error: Option<String>,
}

// ------------ List ------------

pub async fn list_handler(
    Path(tenant_id): Path<String>,
    Extension(pool): Extension<SqlitePool>,
) -> Result<Html<String>, AppError> {
    let years = AcademicYearService::list_all(&pool).await?;
    let sc = StudentService::count(&pool).await.unwrap_or(0);

    let tpl = IndexTpl {
        tenant_id,
        active: "settings",
        csrf_token: CSRF_TOKEN_VALUE,
        student_count: sc,
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
) -> Result<Html<String>, AppError> {
    let sc = StudentService::count(&pool).await.unwrap_or(0);
    let tpl = FormTpl {
        tenant_id,
        active: "settings",
        csrf_token: CSRF_TOKEN_VALUE,
        student_count: sc,
        is_edit: false,
        year: blank_year(),
        error: None,
    };
    Ok(Html(tpl.render()?))
}

pub async fn edit_handler(
    Path((tenant_id, id)): Path<(String, String)>,
    Extension(pool): Extension<SqlitePool>,
) -> Result<Response, AppError> {
    let year = match AcademicYearService::find(&pool, &id).await? {
        Some(y) => y,
        None => {
            return Ok(Redirect::to(&format!(
                "/web/{}/settings/academic-years",
                tenant_id
            ))
            .into_response());
        }
    };
    let sc = StudentService::count(&pool).await.unwrap_or(0);
    let tpl = FormTpl {
        tenant_id,
        active: "settings",
        csrf_token: CSRF_TOKEN_VALUE,
        student_count: sc,
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
    Form(form): Form<AcademicYearForm>,
) -> Result<Response, AppError> {
    if let Err(msg) = form.validate() {
        return render_form_error(&pool, tenant_id, false, form, msg).await;
    }

    let res = AcademicYearService::create(&pool, form.clone()).await;

    if let Err(e) = res {
        let msg = friendly_db_error(&e, &[("academic_years.name", "That academic year name already exists.")]);
        return render_form_error(&pool, tenant_id, false, form, msg).await;
    }

    Ok(Redirect::to(&format!("/web/{}/settings/academic-years", tenant_id)).into_response())
}

pub async fn update_handler(
    Path((tenant_id, id)): Path<(String, String)>,
    Extension(pool): Extension<SqlitePool>,
    Form(form): Form<AcademicYearForm>,
) -> Result<Response, AppError> {
    if let Err(msg) = form.validate() {
        return render_form_error_edit(&pool, tenant_id, id, form, msg).await;
    }

    let res = AcademicYearService::update(&pool, &id, form.clone()).await;

    if let Err(e) = res {
        let msg = friendly_db_error(&e, &[("academic_years.name", "That academic year name already exists.")]);
        return render_form_error_edit(&pool, tenant_id, id, form, msg).await;
    }

    Ok(Redirect::to(&format!("/web/{}/settings/academic-years", tenant_id)).into_response())
}

pub async fn delete_handler(
    Path((tenant_id, id)): Path<(String, String)>,
    Extension(pool): Extension<SqlitePool>,
) -> Result<Response, AppError> {
    AcademicYearService::delete(&pool, &id).await?;
    Ok(Redirect::to(&format!("/web/{}/settings/academic-years", tenant_id)).into_response())
}

pub async fn set_current_handler(
    Path((tenant_id, id)): Path<(String, String)>,
    Extension(pool): Extension<SqlitePool>,
) -> Result<Redirect, AppError> {
    AcademicYearService::set_current(&pool, &id).await?;
    Ok(Redirect::to(&format!(
        "/web/{}/settings/academic-years",
        tenant_id
    )))
}

// ------------ Helpers ------------

async fn render_form_error(
    pool: &SqlitePool,
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
    let sc = StudentService::count(pool).await.unwrap_or(0);
    let tpl = FormTpl {
        tenant_id,
        active: "settings",
        csrf_token: CSRF_TOKEN_VALUE,
        student_count: sc,
        is_edit,
        year,
        error: Some(error),
    };
    Ok(Html(tpl.render()?).into_response())
}

async fn render_form_error_edit(
    pool: &SqlitePool,
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
    let sc = StudentService::count(pool).await.unwrap_or(0);
    let tpl = FormTpl {
        tenant_id,
        active: "settings",
        csrf_token: CSRF_TOKEN_VALUE,
        student_count: sc,
        is_edit: true,
        year,
        error: Some(error),
    };
    Ok(Html(tpl.render()?).into_response())
}
