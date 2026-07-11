use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
    pub role: String,
    pub full_name: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Session {
    pub id: String,
    pub user_id: i64,
    pub user_agent: Option<String>,
    pub client_ip: Option<String>,
    pub expires_at: chrono::NaiveDateTime,
    pub created_at: Option<chrono::NaiveDateTime>,
}
