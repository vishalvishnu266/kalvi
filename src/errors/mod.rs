use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

#[derive(Template)]
#[template(path = "error.html")]
struct ErrorPage {
    status: u16,
    message: String,
    request_id: String,
}

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    ValidationError(String),
    /// The user is authenticated but not permitted to access the resource.
    /// Rendered as a friendly 403 page rather than a redirect (so they know
    /// *why* they can't proceed, and switching accounts is a conscious act).
    Forbidden(String),
    Database(sqlx::Error),
    Unexpected(String),
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self { AppError::Database(e) }
}

impl From<askama::Error> for AppError {
    fn from(e: askama::Error) -> Self { AppError::Unexpected(e.to_string()) }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(m) => (StatusCode::NOT_FOUND, m),
            AppError::ValidationError(m) => (StatusCode::UNPROCESSABLE_ENTITY, m),
            AppError::Forbidden(m) => (StatusCode::FORBIDDEN, m),
            AppError::Database(e) => {
                tracing::error!("db error: {e}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error".into())
            }
            AppError::Unexpected(m) => {
                tracing::error!("unexpected error: {m}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong".into())
            }
        };

        let page = ErrorPage {
            status: status.as_u16(),
            message,
            request_id: String::new(),
        };
        match page.render() {
            Ok(body) => (status, Html(body)).into_response(),
            Err(_) => (status, "Internal Server Error").into_response(),
        }
    }
}
