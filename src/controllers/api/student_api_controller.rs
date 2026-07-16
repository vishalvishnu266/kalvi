use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Extension,
    Json,
};
use sqlx::SqlitePool;

use crate::errors::AppError;
use crate::models::student::{Student, StudentFilters, StudentForm};
use crate::services::student_service::StudentService;

/// List students with optional filtering.
#[utoipa::path(
    get,
    path = "/api/{tenant_id}/students",
    params(
        ("tenant_id" = String, Path, description = "Tenant identifier"),
        StudentFilters
    ),
    responses(
        (status = 200, description = "List of students retrieved successfully", body = [Student]),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn list_students(
    Extension(pool): Extension<SqlitePool>,
    Query(filters): Query<StudentFilters>,
) -> Result<impl IntoResponse, AppError> {
    let students = StudentService::list(&pool, &filters).await?;
    Ok(Json(students))
}

/// Get a specific student by ID.
#[utoipa::path(
    get,
    path = "/api/{tenant_id}/students/{student_id}",
    params(
        ("tenant_id" = String, Path, description = "Tenant identifier"),
        ("student_id" = String, Path, description = "Student ID")
    ),
    responses(
        (status = 200, description = "Student found", body = Student),
        (status = 404, description = "Student not found")
    )
)]
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

/// Create a new student.
#[utoipa::path(
    post,
    path = "/api/{tenant_id}/students",
    params(
        ("tenant_id" = String, Path, description = "Tenant identifier")
    ),
    request_body = StudentForm,
    responses(
        (status = 201, description = "Student created successfully", body = Student),
        (status = 409, description = "Conflict - admission number already exists"),
        (status = 422, description = "Validation error")
    )
)]
pub async fn create_student(
    Extension(pool): Extension<SqlitePool>,
    Json(form): Json<StudentForm>,
) -> Result<impl IntoResponse, AppError> {
    form.validate().map_err(|e| AppError::ValidationError(e))?;
    let student = StudentService::create(&pool, form).await?;
    Ok((StatusCode::CREATED, Json(student)))
}

/// Update an existing student.
#[utoipa::path(
    put,
    path = "/api/{tenant_id}/students/{student_id}",
    params(
        ("tenant_id" = String, Path, description = "Tenant identifier"),
        ("student_id" = String, Path, description = "Student ID")
    ),
    request_body = StudentForm,
    responses(
        (status = 200, description = "Student updated successfully", body = Student),
        (status = 404, description = "Student not found"),
        (status = 422, description = "Validation error")
    )
)]
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

/// Delete a student by ID.
#[utoipa::path(
    delete,
    path = "/api/{tenant_id}/students/{student_id}",
    params(
        ("tenant_id" = String, Path, description = "Tenant identifier"),
        ("student_id" = String, Path, description = "Student ID")
    ),
    responses(
        (status = 204, description = "Student deleted successfully"),
        (status = 404, description = "Student not found")
    )
)]
pub async fn delete_student(
    Path((_tenant_id, student_id)): Path<(String, String)>,
    Extension(pool): Extension<SqlitePool>,
) -> Result<impl IntoResponse, AppError> {
    StudentService::delete(&pool, &student_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
