use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub enum AppError {
    ValidationError(String),
    BusinessLogicError(String),
    UnexpectedError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::ValidationError(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg),
            AppError::BusinessLogicError(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::UnexpectedError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        (status, error_message).into_response()
    }
}
