use axum::{
    http::StatusCode,
    response::{IntoResponse, Response, Html},
};
use std::collections::HashMap;
use std::fmt;
use tracing::error;

pub enum AppError {
    /// System-level errors (DB, File IO, etc.) - Logged to console, hidden from user.
    RuntimeException(String),
    /// User-correctable errors (Validation, Duplicate slug, etc.) - Shown to user.
    BusinessException(String, HashMap<String, String>),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::RuntimeException(e) => write!(f, "RUNTIME_EXCEPTION: {}", e),
            AppError::BusinessException(msg, fields) => write!(f, "BUSINESS_EXCEPTION: {} | Fields: {:?}", msg, fields),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::BusinessException(msg, _) => {
                // Business exceptions return a 400 and are handled by the controller to show inline
                (StatusCode::BAD_REQUEST, msg).into_response()
            }
            AppError::RuntimeException(details) => {
                let service_id = crate::util::id_util::generate_prefixed_id("err");
                
                // 1. Log full technical details to console with the unique ID
                println!("\n[!] CRITICAL ERROR - Service ID: {}\nDetails: {}\n", service_id, details);
                error!(service_id = %service_id, "{}", details);

                // 2. Return the "Oops" error page to the user
                (StatusCode::INTERNAL_SERVER_ERROR, Html(render_error_page(&service_id))).into_response()
            }
        }
    }
}

fn render_error_page(service_id: &str) -> String {
    format!(
        //language=HTML
        r###"<!DOCTYPE html>
<html>
<head>
    <title>Oops! Something went wrong</title>
    <script src="https://cdn.tailwindcss.com"></script>
</head>
<body class="bg-slate-50 flex items-center justify-center min-h-screen p-6">
    <div class="max-w-md w-full text-center">
        <h1 class="text-3xl font-bold text-slate-900 mb-4">Oops! Something went wrong</h1>
        <p class="text-slate-600 mb-8">An internal system error occurred. Our team has been notified.</p>
        
        <div class="bg-white border rounded-xl p-4 mb-8">
            <p class="text-xs uppercase font-bold text-slate-400 mb-1">Service ID</p>
            <code class="text-sm font-mono text-blue-600">{}</code>
        </div>
        
        <a href="/" class="text-blue-500 font-semibold hover:underline">Return to safety</a>
    </div>
</body>
</html>"###,
        service_id
    )
}

// Helper conversions
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::RuntimeException(err.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::RuntimeException(err.to_string())
    }
}
