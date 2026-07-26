use crate::error::RepoError;

/// A tenant identifier used to look up tenant pools and scope every SQL
/// statement into the correct tenant database.
///
/// This is a plain [`String`] — we intentionally do not wrap it in a
/// newtype. Use [`validate_tenant_id`] to construct a validated tenant
/// id from raw input.
pub type TenantId = String;

/// Validate a raw tenant id and return an owned [`TenantId`] (i.e. a
/// `String`) if it satisfies the tenant id format rules.
///
/// Rules:
/// * length must be `1..=64`
/// * only ASCII alphanumerics, `-` and `_` are allowed
pub fn validate_tenant_id(raw: impl Into<String>) -> Result<TenantId, TenantError> {
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
    Ok(raw)
}

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
