use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Session {
    pub id: i64,
    pub token: String,
    pub user_id: i64,
    pub tenant_id: Option<String>,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
    pub last_seen_at: NaiveDateTime,
    pub revoked_at: Option<NaiveDateTime>,
    pub user_agent: Option<String>,
    pub remote_ip: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NewSession {
    pub token: String,
    pub user_id: i64,
    pub tenant_id: Option<String>,
    pub expires_at: NaiveDateTime,
    pub user_agent: Option<String>,
    pub remote_ip: Option<String>,
}
