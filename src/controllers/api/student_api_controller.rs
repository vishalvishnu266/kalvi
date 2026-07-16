use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Extension,
    Json,
};
use sqlx::SqlitePool;

use crate::errors::AppError;
use crate::models::student::{StudentFilters, StudentForm};
use crate::services::student_service::StudentService;

pub async fn list_students(
    Extension(pool): Extension<SqlitePool>,
    Query(filters): Query<StudentFilters>,
) -> Result<impl IntoResponse, AppError> {
    let students = StudentService::list(&pool, &filters).await?;
    Ok(Json(students))
}

pub async fn get_student(
    Path((_tenant_id, student_id)): Path<(String, String)>,
    Extension(pool): Extension<SqlitePool>,
) -> Result<impl IntoResponse, AppError> {
    let student = StudentService::get_by_id(&pool, &student_id).await?;
    match student {
        Some(s) => Ok(Json(s).into_response()),
        None => Err(AppError::NotFound("Student not found".into())),
    }
}

pub async fn create_student(
    Extension(pool): Extension<SqlitePool>,
    Json(form): Json<StudentForm>,
) -> Result<impl IntoResponse, AppError> {
    form.validate().map_err(|e| AppError::ValidationError(e))?;
    let student = StudentService::create(&pool, form).await?;
    Ok((StatusCode::CREATED, Json(student)))
}

pub async fn update_student(
    Path((_tenant_id, student_id)): Path<(String, String)>,
    Extension(pool): Extension<SqlitePool>,
    Json(form): Json<StudentForm>,
) -> Result<impl IntoResponse, AppError> {
    form.validate().map_err(|e| AppError::ValidationError(e))?;
    StudentService::update(&pool, &student_id, form).await?;
    let student = StudentService::get_by_id(&pool, &student_id).await?;
    match student {
        Some(s) => Ok(Json(s).into_response()),
        None => Err(AppError::NotFound("Student not found".into())),
    }
}

pub async fn delete_student(
    Path((_tenant_id, student_id)): Path<(String, String)>,
    Extension(pool): Extension<SqlitePool>,
) -> Result<impl IntoResponse, AppError> {
    StudentService::delete(&pool, &student_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
