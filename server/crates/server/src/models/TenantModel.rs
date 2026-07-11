use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Tenant {
    pub id: i64,
    pub slug: String,
    pub name: String,
    pub contact_email: String,
    pub contact_phone: String,
    pub address: String,
    pub database_name: String,
    pub is_active: bool,
    pub primary_color: String,
    pub dark_mode: bool,
    pub created_at: String,
}

pub struct NewTenant<'a> {
    pub slug: &'a str,
    pub name: &'a str,
    pub contact_email: &'a str,
    pub contact_phone: &'a str,
    pub address: &'a str,
}
