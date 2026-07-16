use askama::Template;
use axum::{
    extract::{Form, Path, Query},
    http::HeaderMap,
    response::{Html, IntoResponse, Redirect, Response},
    Extension,
};
use sqlx::SqlitePool;

use crate::csrf_middleware::CSRF_TOKEN_VALUE;
use crate::errors::AppError;
use crate::models::student::{Student, StudentFilters, StudentForm};
use crate::services::student_service::StudentService;
use crate::utils::crud::{friendly_db_error, is_turbo_frame, wrap_turbo_frame};

use super::{CLASS_OPTIONS, GENDER_OPTIONS, SECTION_OPTIONS, STATUS_OPTIONS};

// ---------------- List ----------------

#[derive(Template)]
#[template(path = "students/index.html")]
struct IndexTpl<'a> {
    tenant_id: String,
    active: &'static str,
    student_count: i64,
    csrf_token: &'a str,

    students: Vec<Student>,
    filters: StudentFilters,

    class_options: &'a [&'a str],
    section_options: &'a [&'a str],
    status_options: &'a [&'a str],
}

#[derive(Template)]
#[template(path = "students/_list.html")]
struct ListFrameTpl<'a> {
    tenant_id: String,
    csrf_token: &'a str,
    students: Vec<Student>,
}

pub async fn list_students_handler(
    Path(tenant_id): Path<String>,
    Query(filters): Query<StudentFilters>,
    Extension(pool): Extension<SqlitePool>,
    headers: HeaderMap,
) -> Result<Response, AppError> {
    let students = StudentService::list(&pool, &filters).await?;

    if is_turbo_frame(&headers) {
        let tpl = ListFrameTpl {
            tenant_id,
            csrf_token: CSRF_TOKEN_VALUE,
            students,
        };
        let body = wrap_turbo_frame("students-list", tpl.render()?);
        return Ok(Html(body).into_response());
    }

    let total = StudentService::count(&pool).await?;

    let tpl = IndexTpl {
        tenant_id,
        active: "students",
        student_count: total,
        csrf_token: CSRF_TOKEN_VALUE,
        students,
        filters,
        class_options: CLASS_OPTIONS,
        section_options: SECTION_OPTIONS,
        status_options: STATUS_OPTIONS,
    };
    Ok(Html(tpl.render()?).into_response())
}

// ---------------- Form (new / edit) ----------------

#[derive(Template)]
#[template(path = "students/form.html")]
struct FormTpl<'a> {
    tenant_id: String,
    active: &'static str,
    student_count: i64,
    csrf_token: &'a str,

    is_edit: bool,
    student: Student,
    error: Option<String>,

    class_options: &'a [&'a str],
    section_options: &'a [&'a str],
    status_options: &'a [&'a str],
    gender_options: &'a [&'a str],
}

fn default_new_student() -> Student {
    Student {
        gender: "Male".into(),
        class_name: CLASS_OPTIONS[0].into(),
        section: SECTION_OPTIONS[0].into(),
        status: "Active".into(),
        admission_date: chrono::Utc::now().date_naive().to_string(),
        ..Default::default()
    }
}

pub async fn new_student_handler(
    Path(tenant_id): Path<String>,
    Extension(pool): Extension<SqlitePool>,
) -> Result<Html<String>, AppError> {
    let tpl = FormTpl {
        tenant_id,
        active: "students",
        student_count: StudentService::count(&pool).await?,
        csrf_token: CSRF_TOKEN_VALUE,
        is_edit: false,
        student: default_new_student(),
        error: None,
        class_options: CLASS_OPTIONS,
        section_options: SECTION_OPTIONS,
        status_options: STATUS_OPTIONS,
        gender_options: GENDER_OPTIONS,
    };
    Ok(Html(tpl.render()?))
}

pub async fn edit_student_handler(
    Path((tenant_id, student_id)): Path<(String, String)>,
    Extension(pool): Extension<SqlitePool>,
) -> Result<Response, AppError> {
    let student = StudentService::get_by_id(&pool, &student_id).await?;

    let student = match student {
        Some(s) => s,
        None => {
            return Ok(Redirect::to(&format!("/web/{}/students", tenant_id)).into_response());
        }
    };

    let tpl = FormTpl {
        tenant_id,
        active: "students",
        student_count: StudentService::count(&pool).await?,
        csrf_token: CSRF_TOKEN_VALUE,
        is_edit: true,
        student,
        error: None,
        class_options: CLASS_OPTIONS,
        section_options: SECTION_OPTIONS,
        status_options: STATUS_OPTIONS,
        gender_options: GENDER_OPTIONS,
    };
    Ok(Html(tpl.render()?).into_response())
}

// ---------------- Create / Update / Delete ----------------

pub async fn create_student_handler(
    Path(tenant_id): Path<String>,
    Extension(pool): Extension<SqlitePool>,
    Form(form): Form<StudentForm>,
) -> Result<Response, AppError> {
    if let Err(msg) = form.validate() {
        return render_form_with_error(&pool, tenant_id, false, None, form, msg).await;
    }

    let res = StudentService::create(&pool, form.clone()).await;

    if let Err(e) = res {
        let msg = friendly_db_error(&e, &[("admission_no", "Admission number already exists.")]);
        return render_form_with_error(&pool, tenant_id, false, None, form, msg).await;
    }

    Ok(Redirect::to(&format!("/web/{}/students", tenant_id)).into_response())
}

pub async fn update_student_handler(
    Path((tenant_id, student_id)): Path<(String, String)>,
    Extension(pool): Extension<SqlitePool>,
    Form(form): Form<StudentForm>,
) -> Result<Response, AppError> {
    if let Err(msg) = form.validate() {
        return render_form_with_error(&pool, tenant_id, true, Some(student_id), form, msg).await;
    }

    let res = StudentService::update(&pool, &student_id, form.clone()).await;

    if let Err(e) = res {
        let msg = friendly_db_error(&e, &[("admission_no", "Admission number already exists.")]);
        return render_form_with_error(&pool, tenant_id, true, Some(student_id), form, msg).await;
    }

    Ok(Redirect::to(&format!("/web/{}/students", tenant_id)).into_response())
}

pub async fn delete_student_handler(
    Path((tenant_id, student_id)): Path<(String, String)>,
    Extension(pool): Extension<SqlitePool>,
) -> Result<Response, AppError> {
    StudentService::delete(&pool, &student_id).await?;
    Ok(Redirect::to(&format!("/web/{}/students", tenant_id)).into_response())
}

// ---------------- Helpers ----------------

async fn render_form_with_error(
    pool: &SqlitePool,
    tenant_id: String,
    is_edit: bool,
    student_id: Option<String>,
    form: StudentForm,
    error: String,
) -> Result<Response, AppError> {
    let mut student = form.apply_to(default_new_student());
    if let Some(id) = student_id {
        student.id = id;
    }

    let tpl = FormTpl {
        tenant_id,
        active: "students",
        student_count: StudentService::count(pool).await?,
        csrf_token: CSRF_TOKEN_VALUE,
        is_edit,
        student,
        error: Some(error),
        class_options: CLASS_OPTIONS,
        section_options: SECTION_OPTIONS,
        status_options: STATUS_OPTIONS,
        gender_options: GENDER_OPTIONS,
    };
    Ok(Html(tpl.render()?).into_response())
}
