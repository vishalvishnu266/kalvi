use axum::{
    extract::{Extension, Path},
    response::{Html, IntoResponse},
    Json,
};
use service::student_service;
use sqlx::SqlitePool;
use web::student_template;

pub async fn student_handler(
    Extension(pool): Extension<SqlitePool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match student_service::get_student(&pool, id).await {
        Ok(Some(student)) => Html(student_template(&student).into_string()),
        _ => Html("<h1>Student not found</h1>".to_string()),
    }
}

pub async fn student_json_handler(
    Extension(pool): Extension<SqlitePool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match student_service::get_student(&pool, id).await {
        Ok(Some(student)) => Ok(Json(student)),
        _ => Err(axum::http::StatusCode::NOT_FOUND),
    }
}
