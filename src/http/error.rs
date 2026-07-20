//! Map [`crate::ServiceError`] → HTTP responses.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use crate::ServiceError;

/// Newtype so we can implement `IntoResponse` outside the services crate.
pub struct ServiceHttpError(pub ServiceError);

impl From<ServiceError> for ServiceHttpError {
    fn from(e: ServiceError) -> Self { Self(e) }
}

impl IntoResponse for ServiceHttpError {
    fn into_response(self) -> Response {
        let (status, code) = match &self.0 {
            ServiceError::NotFound        => (StatusCode::NOT_FOUND, "not_found"),
            ServiceError::Validation(_)   => (StatusCode::BAD_REQUEST, "validation_error"),
            ServiceError::Conflict(_)     => (StatusCode::CONFLICT, "conflict"),
            ServiceError::Unauthorized    => (StatusCode::UNAUTHORIZED, "unauthorized"),
            ServiceError::Forbidden(_)    => (StatusCode::FORBIDDEN, "forbidden"),
            ServiceError::Hash(_)         => (StatusCode::INTERNAL_SERVER_ERROR, "hash_error"),
            ServiceError::Repo(_)         => (StatusCode::INTERNAL_SERVER_ERROR, "repo_error"),
            ServiceError::Sqlx(_)         => (StatusCode::INTERNAL_SERVER_ERROR, "db_error"),
        };
        let body = Json(json!({
            "error": code,
            "message": self.0.to_string(),
        }));
        (status, body).into_response()
    }
}
