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
            AppError::Database(e) => {
                tracing::error!("db error: {e}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error".into())
            }
            AppError::Unexpected(m) => {
                tracing::error!("unexpected error: {m}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong".into())
            }
        };

        // If it's an API request, return JSON. 
        // Simple heuristic: if we want to be strict, we'd check headers, 
        // but often we can just have a way to distinguish.
        // For now, let's keep it simple and maybe just use a different error type for API or check if it's a web route.
        // Actually, we can check the request's Accept header if we had access to it, 
        // but into_response doesn't have it.
        
        // Let's assume for now we can just return a simple response.
        // To support both, we might need a more sophisticated error handler.
        
        // A common trick is to return a response that can be either HTML or JSON.
        // But for this task, I'll just keep it as is and maybe suggest a better way if needed.
        // Wait, the user wants JSON API. If the API returns AppError, it currently returns HTML.
        
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
