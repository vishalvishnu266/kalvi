use thiserror::Error;

use crate::exception::repo_error::RepoError;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("not found")]
    NotFound,

    #[error("validation error: {0}")]
    Validation(String),

    #[error("conflict: {0}")]
    Conflict(String),

    #[error("unauthorized")]
    Unauthorized,

    #[error("forbidden: missing permission {0}")]
    Forbidden(String),

    #[error("password hashing failed: {0}")]
    Hash(String),

    #[error("repository error: {0}")]
    Repo(#[from] RepoError),

    #[error("database error: {0}")]
    Sqlx(#[from] sqlx::Error),
}

impl ServiceError {
    pub fn validation(m: impl Into<String>) -> Self {
        Self::Validation(m.into())
    }
    pub fn conflict(m: impl Into<String>) -> Self {
        Self::Conflict(m.into())
    }
    pub fn forbidden(perm: impl Into<String>) -> Self {
        Self::Forbidden(perm.into())
    }
}

pub type ServiceResult<T> = Result<T, ServiceError>;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        let (status, code) = match &self {
            ServiceError::NotFound => (StatusCode::NOT_FOUND, "not_found"),
            ServiceError::Validation(_) => (StatusCode::BAD_REQUEST, "validation_error"),
            ServiceError::Conflict(_) => (StatusCode::CONFLICT, "conflict"),
            ServiceError::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized"),
            ServiceError::Forbidden(_) => (StatusCode::FORBIDDEN, "forbidden"),
            ServiceError::Hash(_) => (StatusCode::INTERNAL_SERVER_ERROR, "hash_error"),
            ServiceError::Repo(_) => (StatusCode::INTERNAL_SERVER_ERROR, "repo_error"),
            ServiceError::Sqlx(_) => (StatusCode::INTERNAL_SERVER_ERROR, "db_error"),
        };
        let body = Json(serde_json::json!({
            "error":   code,
            "message": self.to_string(),
        }));
        (status, body).into_response()
    }
}
