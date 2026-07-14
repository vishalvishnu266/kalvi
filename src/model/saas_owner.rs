use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SaasOwner {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
    pub full_name: Option<String>,
    pub created_at: i64,
}
