use thiserror::Error;

/// Single error type used across all repositories.
#[derive(Debug, Error)]
pub enum RepoError {
    #[error("record not found")]
    NotFound,

    #[error("validation error: {0}")]
    Validation(String),

    #[error("conflict: {0}")]
    Conflict(String),

    #[error("database error: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("migrate error: {0}")]
    Migrate(#[from] sqlx::migrate::MigrateError),
}

impl RepoError {
    pub fn validation(msg: impl Into<String>) -> Self {
        RepoError::Validation(msg.into())
    }

    pub fn conflict(msg: impl Into<String>) -> Self {
        RepoError::Conflict(msg.into())
    }
}

pub type RepoResult<T> = Result<T, RepoError>;
