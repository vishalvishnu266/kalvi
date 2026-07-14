use axum::{
    http::StatusCode,
    response::{IntoResponse, Response, Html},
};
use std::fmt;
use tracing::error;
use uuid::Uuid;
use crate::view::{render_layout, LayoutContext};

#[derive(Debug)]
pub enum AppError {
    Database(String),
    Internal(String),
    NotFound(String),
    Unauthorized(String),
    TooManyRequests(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Database(e) => write!(f, "Database error: {}", e),
            AppError::Internal(e) => write!(f, "Internal error: {}", e),
            AppError::NotFound(e) => write!(f, "Not found: {}", e),
            AppError::Unauthorized(e) => write!(f, "Unauthorized: {}", e),
            AppError::TooManyRequests(e) => write!(f, "Too many requests: {}", e),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let support_id = Uuid::new_v4().to_string();
        
        error!(
            support_id = %support_id,
            "Application error: {}", self
        );

        let (status, message) = match self {
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "A database error occurred. We have been notified.".to_string()),
            AppError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "An internal server error occurred.".to_string()),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::TooManyRequests(_) => (StatusCode::TOO_MANY_REQUESTS, "Slow down! You are sending too many requests.".to_string()),
        };

        // Render a generic crash/error page
        let html_content = format!(
            //language=HTML
            r#"<div class="container min-vh-100 d-flex align-items-center justify-content-center p-4">
                <div class="text-center" style="max-width: 500px;">
                    <div class="mb-5 d-inline-flex align-items-center justify-content-center bg-danger-subtle text-danger rounded-4 shadow-sm" style="width: 80px; height: 80px;">
                        <svg width="40" height="40" fill="currentColor" viewBox="0 0 16 16">
                            <path d="M8.982 1.566a1.13 1.13 0 0 0-1.96 0L.165 13.233c-.457.778.091 1.767.98 1.767h13.713c.889 0 1.438-.99.98-1.767L8.982 1.566zM8 5c.535 0 .954.462.9.995l-.35 3.507a.552.552 0 0 1-1.1 0L7.1 5.995A.905.905 0 0 1 8 5zm.002 6a1 1 0 1 1 0 2 1 1 0 0 1 0-2z"/>
                        </svg>
                    </div>
                    <h1 class="h2 fw-bold text-body-emphasis mb-3">Oops! Something went wrong</h1>
                    <p class="text-secondary mb-5">{message}</p>
                    
                    <div class="card shadow-sm border-0 mb-5">
                        <div class="card-body p-4 bg-body-tertiary rounded-4">
                            <p class="small text-uppercase fw-bold text-secondary mb-1 text-center">Support ID</p>
                            <code class="small fw-bold text-primary break-all select-all">{support_id}</code>
                        </div>
                    </div>
                    
                    <a href="/" class="btn btn-primary px-5 py-3 shadow-sm fw-bold">
                        Back to Home
                    </a>
                </div>
            </div>"#,
            message = message,
            support_id = support_id
        );

        let body = render_layout(LayoutContext::default(), html_content);
        (status, Html(body)).into_response()
    }
}

// Implement From for common error types to allow using the `?` operator
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err.to_string())
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Internal(err.to_string())
    }
}
