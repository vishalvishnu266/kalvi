use std::path::PathBuf;
use std::sync::Arc;

use sqlx::SqlitePool;
use crate::error::{RepoError, RepoResult};
use crate::repositories::Repositories;
use crate::services::auth::AuthService;
use crate::services::AppServices;
use crate::session::SessionStore;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct TenantId(String);

impl TenantId {
    pub fn new(raw: impl Into<String>) -> Result<Self, TenantError> {
        let raw = raw.into();
        if raw.is_empty() || raw.len() > 64 {
            return Err(TenantError::InvalidId("length must be 1..=64".into()));
        }
        if !raw.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') {
            return Err(TenantError::InvalidId("only [A-Za-z0-9_-] allowed".into()));
        }
        Ok(Self(raw))
    }

    pub fn as_str(&self) -> &str { &self.0 }
}

impl std::fmt::Display for TenantId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TenantError {
    #[error("invalid tenant id: {0}")]
    InvalidId(String),

    #[error("tenant not found: {0}")]
    NotFound(TenantId),

    #[error("tenant is disabled: {0}")]
    Disabled(TenantId),

    #[error(transparent)]
    Repo(#[from] RepoError),
}

pub async fn build_tenant_services(
    pool: &SqlitePool,
    sessions: SessionStore,
) -> RepoResult<AppServices> {
    let repos = Arc::new(Repositories::new(pool.clone()));
    let auth = AuthService::with_session_store(
        repos.clone(),
        sessions,
    );
    Ok(AppServices::from_repos_with_auth(repos, auth))
}

pub fn tenant_db_path(root: &std::path::Path, tenant: &TenantId) -> PathBuf {
    root.join(format!("{}.db", tenant.as_str()))
}

pub fn tenant_db_url(path: &std::path::Path) -> String {
    format!("sqlite://{}", path.display())
}
