use askama::Template;
use axum::{
    extract::{Form, Path, Query},
    http::HeaderMap,
    response::{Html, IntoResponse, Redirect, Response},
    Extension,
};
// HeaderMap is still needed as an extractor param on list_students_handler,
// but the local `is_turbo_frame` helper now lives in `utils::crud`.
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::csrf_middleware::CSRF_TOKEN_VALUE;
use crate::errors::AppError;
use crate::models::student::{Student, StudentFilters, StudentForm};
use crate::utils::crud::{friendly_db_error, is_turbo_frame, opt, wrap_turbo_frame};

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

async fn load_students(
    pool: &SqlitePool,
    f: &StudentFilters,
) -> Result<Vec<Student>, AppError> {
    let mut sql = String::from("SELECT * FROM students WHERE 1=1");
    let mut binds: Vec<String> = Vec::new();

    if !f.class_name.is_empty() {
        sql.push_str(" AND class_name = ?");
        binds.push(f.class_name.clone());
    }
    if !f.section.is_empty() {
        sql.push_str(" AND section = ?");
        binds.push(f.section.clone());
    }
    if !f.status.is_empty() {
        sql.push_str(" AND status = ?");
        binds.push(f.status.clone());
    }
    if !f.q.trim().is_empty() {
        sql.push_str(
            " AND (first_name LIKE ? OR last_name LIKE ? OR admission_no LIKE ? OR email LIKE ?)",
        );
        let pat = format!("%{}%", f.q.trim());
        binds.push(pat.clone());
        binds.push(pat.clone());
        binds.push(pat.clone());
        binds.push(pat);
    }
    sql.push_str(" ORDER BY class_name, section, roll_no");

    let mut q = sqlx::query_as::<_, Student>(&sql);
    for b in &binds {
        q = q.bind(b);
    }
    Ok(q.fetch_all(pool).await?)
}

pub async fn list_students_handler(
    Path(tenant_id): Path<String>,
    Query(filters): Query<StudentFilters>,
    Extension(pool): Extension<SqlitePool>,
    headers: HeaderMap,
) -> Result<Response, AppError> {
    let students = load_students(&pool, &filters).await?;

    if is_turbo_frame(&headers) {
        // Only re-render the frame contents
        let tpl = ListFrameTpl {
            tenant_id,
            csrf_token: CSRF_TOKEN_VALUE,
            students,
        };
        // Wrap in the same <turbo-frame id="students-list"> so Turbo swaps it in place.
        let body = wrap_turbo_frame("students-list", tpl.render()?);
        return Ok(Html(body).into_response());
    }

    let total: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM students").fetch_one(&pool).await?;

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

async fn student_count(pool: &SqlitePool) -> Result<i64, AppError> {
    Ok(sqlx::query_scalar("SELECT COUNT(*) FROM students")
        .fetch_one(pool)
        .await?)
}

pub async fn new_student_handler(
    Path(tenant_id): Path<String>,
    Extension(pool): Extension<SqlitePool>,
) -> Result<Html<String>, AppError> {
    let tpl = FormTpl {
        tenant_id,
        active: "students",
        student_count: student_count(&pool).await?,
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
    let student = sqlx::query_as::<_, Student>("SELECT * FROM students WHERE id = ?")
        .bind(&student_id)
        .fetch_optional(&pool)
        .await?;

    let student = match student {
        Some(s) => s,
        None => {
            return Ok(Redirect::to(&format!("/web/{}/students", tenant_id)).into_response());
        }
    };

    let tpl = FormTpl {
        tenant_id,
        active: "students",
        student_count: student_count(&pool).await?,
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

    let mut student = form.apply_to(default_new_student());
    student.id = Uuid::new_v4().to_string();

    let res = sqlx::query(
        r#"INSERT INTO students (
            id, admission_no, first_name, last_name, email, phone, date_of_birth, gender, blood_group,
            class_name, section, roll_no, admission_date,
            guardian_name, guardian_phone, guardian_email, guardian_relation,
            address_line, city, state, postal_code, status
        ) VALUES (?,?,?,?,?,?,?,?,?, ?,?,?,?, ?,?,?,?, ?,?,?,?, ?)"#,
    )
    .bind(&student.id)
    .bind(&student.admission_no)
    .bind(&student.first_name)
    .bind(&student.last_name)
    .bind(&student.email)
    .bind(&student.phone)
    .bind(&student.date_of_birth)
    .bind(&student.gender)
    .bind(&student.blood_group)
    .bind(&student.class_name)
    .bind(&student.section)
    .bind(&student.roll_no)
    .bind(&student.admission_date)
    .bind(&student.guardian_name)
    .bind(&student.guardian_phone)
    .bind(&student.guardian_email)
    .bind(&student.guardian_relation)
    .bind(&student.address_line)
    .bind(&student.city)
    .bind(&student.state)
    .bind(&student.postal_code)
    .bind(&student.status)
    .execute(&pool)
    .await;

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

    let res = sqlx::query(
        r#"UPDATE students SET
            admission_no=?, first_name=?, last_name=?, email=?, phone=?, date_of_birth=?, gender=?, blood_group=?,
            class_name=?, section=?, roll_no=?, admission_date=?,
            guardian_name=?, guardian_phone=?, guardian_email=?, guardian_relation=?,
            address_line=?, city=?, state=?, postal_code=?, status=?,
            updated_at = CURRENT_TIMESTAMP
        WHERE id = ?"#,
    )
    .bind(form.admission_no.trim())
    .bind(form.first_name.trim())
    .bind(form.last_name.trim())
    .bind(opt(&form.email))
    .bind(opt(&form.phone))
    .bind(opt(&form.date_of_birth))
    .bind(form.gender.trim())
    .bind(opt(&form.blood_group))
    .bind(form.class_name.trim())
    .bind(form.section.trim())
    .bind(form.roll_no.trim())
    .bind(if form.admission_date.trim().is_empty() {
        None
    } else {
        Some(form.admission_date.trim().to_string())
    })
    .bind(form.guardian_name.trim())
    .bind(form.guardian_phone.trim())
    .bind(opt(&form.guardian_email))
    .bind(opt(&form.guardian_relation))
    .bind(opt(&form.address_line))
    .bind(opt(&form.city))
    .bind(opt(&form.state))
    .bind(opt(&form.postal_code))
    .bind(form.status.trim())
    .bind(&student_id)
    .execute(&pool)
    .await;

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
    sqlx::query("DELETE FROM students WHERE id = ?")
        .bind(&student_id)
        .execute(&pool)
        .await?;
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
        student_count: student_count(pool).await?,
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
