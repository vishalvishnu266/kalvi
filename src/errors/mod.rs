use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    ValidationError(String),
    Conflict(String),
    Database(sqlx::Error),
    Unexpected(String),
}

#[derive(Serialize)]
struct ApiErrorResponse {
    status: u16,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_type: Option<String>,
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self { AppError::Database(e) }
}

impl AppError {
    fn get_details(&self) -> (StatusCode, String, &'static str) {
        match self {
            AppError::NotFound(m) => (StatusCode::NOT_FOUND, m.clone(), "not_found"),
            AppError::ValidationError(m) => (StatusCode::UNPROCESSABLE_ENTITY, m.clone(), "validation_error"),
            AppError::Conflict(m) => (StatusCode::CONFLICT, m.clone(), "conflict"),
            AppError::Database(e) => {
                tracing::error!("db error: {e}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error".into(), "database_error")
            }
            AppError::Unexpected(m) => {
                tracing::error!("unexpected error: {m}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong".into(), "unexpected_error")
            }
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message, error_type) = self.get_details();
        (status, Json(ApiErrorResponse {
            status: status.as_u16(),
            message,
            error_type: Some(error_type.into()),
        })).into_response()
    }
}
