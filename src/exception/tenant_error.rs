use crate::exception::repo_error::RepoError;

#[derive(Debug, thiserror::Error)]
pub enum TenantError {
    #[error("invalid tenant id: {0}")]
    InvalidId(String),

    #[error("tenant not found: {0}")]
    NotFound(String),

    #[error("tenant is disabled: {0}")]
    Disabled(String),

    #[error(transparent)]
    Repo(#[from] RepoError),
}
