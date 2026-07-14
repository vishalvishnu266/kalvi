use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Tenant {
    pub id: i64,
    pub slug: String,
    pub name: String,
    pub database_name: String,
    pub created_at: i64,
}
