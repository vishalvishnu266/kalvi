use crate::exception::tenant_error::TenantError;

/// A tenant identifier used to look up tenant pools and scope every SQL
/// statement into the correct tenant database.
pub type TenantId = String;

/// Validate a raw tenant id and return an owned [`TenantId`].
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
