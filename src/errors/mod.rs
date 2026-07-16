use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    Json,
};

#[derive(Template)]
#[template(path = "error.html")]
struct ErrorPage {
    status: u16,
    message: String,
    request_id: String,
}

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

impl From<askama::Error> for AppError {
    fn from(e: askama::Error) -> Self { AppError::Unexpected(e.to_string()) }
}

impl AppError {
    pub fn to_json_response(&self) -> Response {
        let (status, message, error_type) = self.get_details();
        (status, Json(ApiErrorResponse {
            status: status.as_u16(),
            message,
            error_type: Some(error_type.into()),
        })).into_response()
    }

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
        let (status, message, _) = self.get_details();

        let page = ErrorPage {
            status: status.as_u16(),
            message: message.clone(),
            request_id: String::new(),
        };
        
        match page.render() {
            Ok(body) => (status, Html(body)).into_response(),
            Err(_) => (status, message).into_response(),
        }
    }
}
