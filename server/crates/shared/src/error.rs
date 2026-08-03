use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub enum AppError {
    Database(sqlx::Error),
    NotFound,
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Resource not found"),
        };

        (status, message).into_response()
    }
}
