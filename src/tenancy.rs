use crate::error::RepoError;

/// A validated tenant identifier used to look up tenant pools and
/// scope every SQL statement into the correct tenant database.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct TenantId(String);

impl TenantId {
    pub fn new(raw: impl Into<String>) -> Result<Self, TenantError> {
        let raw = raw.into();
        if raw.is_empty() || raw.len() > 64 {
            return Err(TenantError::InvalidId("length must be 1..=64".into()));
        }
        if !raw
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        {
            return Err(TenantError::InvalidId("only [A-Za-z0-9_-] allowed".into()));
        }
        Ok(Self(raw))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
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
