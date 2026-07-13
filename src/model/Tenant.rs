use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Tenant {
    pub id: i64,
    pub slug: String,
    pub name: String,
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
    pub address: Option<String>,
    pub database_name: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}

pub struct NewTenant {
    pub slug: String,
    pub name: String,
}
