use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PortalMembership {
    pub id: i64,
    pub portal_user_id: i64,
    pub tenant_id: String,
    pub tenant_user_id: i64,
    pub role: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NewPortalMembership {
    pub portal_user_id: i64,
    pub tenant_id: String,
    pub tenant_user_id: i64,
    pub role: String,
}
