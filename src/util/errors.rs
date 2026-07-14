use axum::{
    http::StatusCode,
    response::{IntoResponse, Response, Html},
};
use std::collections::HashMap;
use serde::Serialize;
use tracing::error;

#[derive(Debug, Serialize)]
pub enum AppError {
    RuntimeException(String),
    BusinessException {
        errors: HashMap<String, String>,
        message: Option<String>,
    },
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::RuntimeException(err) => {
                let service_id = crate::util::id_util::generate_uuid();
                error!(service_id = %service_id, "Runtime Exception: {}", err);
                
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Html(format!(
                        "<html><body><h1>Something went wrong</h1><p>Service ID: {}</p></body></html>",
                        service_id
                    )),
                ).into_response()
            }
            AppError::BusinessException { errors, message } => {
                // In a real app, this would render a specific Turbo fragment or JSON
                // For now, returning a simple 400 with the message
                let msg = message.unwrap_or_else(|| "Validation error".to_string());
                (StatusCode::BAD_REQUEST, msg).into_response()
            }
        }
    }
}
