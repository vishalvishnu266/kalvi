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
            r#"<div class="min-h-screen flex items-center justify-center p-6 bg-slate-50 dark:bg-slate-950">
                <div class="max-w-md w-full text-center">
                    <div class="mb-6 inline-flex items-center justify-center w-20 h-20 rounded-full bg-red-100 text-red-600 shadow-lg shadow-red-200/50">
                        <svg class="w-10 h-10" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"/>
                        </svg>
                    </div>
                    <h1 class="text-3xl font-bold text-slate-900 dark:text-white mb-2">Oops! Something went wrong</h1>
                    <p class="text-slate-600 dark:text-slate-400 mb-6">{message}</p>
                    
                    <div class="bg-white dark:bg-slate-900 border dark:border-slate-800 rounded-2xl p-4 mb-8">
                        <p class="text-xs uppercase font-bold text-slate-400 mb-1">Support ID</p>
                        <code class="text-sm font-mono text-primary break-all select-all">{support_id}</code>
                    </div>

                    <a href="/" class="inline-flex items-center justify-center px-8 py-3 border border-transparent text-base font-bold rounded-xl text-white bg-primary hover:bg-primary-600 shadow-lg shadow-primary/30 transition-all">
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
