use axum::{response::{Html, Redirect, IntoResponse}, Extension, extract::{Path, Form}};
use crate::views::{student_management, student_form};
use sqlx::SqlitePool;
use crate::controllers::dashboard_controller::Student;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct StudentParams {
    pub name: String,
    pub grade: String,
    pub section: String,
    pub status: String,
    pub attendance_pct: f64,
}

pub async fn list_students_handler(
    Path(tenant_id): Path<String>,
    Extension(pool): Extension<SqlitePool>
) -> Html<String> {
    let students = sqlx::query_as::<_, Student>("SELECT * FROM students ORDER BY name ASC")
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

    Html(student_management::render(&tenant_id, students))
}

pub async fn new_student_handler(Path(tenant_id): Path<String>) -> Html<String> {
    Html(student_form::render(&tenant_id, None))
}

pub async fn create_student_handler(
    Path(tenant_id): Path<String>,
    Extension(pool): Extension<SqlitePool>,
    Form(params): Form<StudentParams>
) -> impl IntoResponse {
    let id = Uuid::new_v4().to_string();
    let _ = sqlx::query(
        "INSERT INTO students (id, name, grade, section, status, attendance_pct) VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(id)
    .bind(params.name)
    .bind(params.grade)
    .bind(params.section)
    .bind(params.status)
    .bind(params.attendance_pct)
    .execute(&pool)
    .await;

    Redirect::to(&format!("/web/{}/students", tenant_id))
}

pub async fn edit_student_handler(
    Path((tenant_id, student_id)): Path<(String, String)>,
    Extension(pool): Extension<SqlitePool>
) -> impl IntoResponse {
    let student = sqlx::query_as::<_, Student>("SELECT * FROM students WHERE id = ?")
        .bind(student_id)
        .fetch_optional(&pool)
        .await
        .unwrap();

    if let Some(s) = student {
        Html(student_form::render(&tenant_id, Some(s))).into_response()
    } else {
        Redirect::to(&format!("/web/{}/students", tenant_id)).into_response()
    }
}

pub async fn update_student_handler(
    Path((tenant_id, student_id)): Path<(String, String)>,
    Extension(pool): Extension<SqlitePool>,
    Form(params): Form<StudentParams>
) -> impl IntoResponse {
    let _ = sqlx::query(
        "UPDATE students SET name = ?, grade = ?, section = ?, status = ?, attendance_pct = ? WHERE id = ?"
    )
    .bind(params.name)
    .bind(params.grade)
    .bind(params.section)
    .bind(params.status)
    .bind(params.attendance_pct)
    .bind(student_id)
    .execute(&pool)
    .await;

    Redirect::to(&format!("/web/{}/students", tenant_id))
}

pub async fn delete_student_handler(
    Path((tenant_id, student_id)): Path<(String, String)>,
    Extension(pool): Extension<SqlitePool>
) -> impl IntoResponse {
    let _ = sqlx::query("DELETE FROM students WHERE id = ?")
        .bind(student_id)
        .execute(&pool)
        .await;

    Redirect::to(&format!("/web/{}/students", tenant_id))
}
