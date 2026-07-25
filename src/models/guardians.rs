use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Guardian {
    pub id: i64,
    pub user_id: Option<i64>,
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub occupation: Option<String>,
    pub address: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewGuardian {
    pub user_id: Option<i64>,
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub occupation: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateGuardian {
    pub first_name: Option<String>,
    pub last_name:  Option<String>,
    pub phone:      Option<String>,
    pub email:      Option<String>,
    pub occupation: Option<String>,
    pub address:    Option<String>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct StudentGuardianLink {
    pub student_id: i64,
    pub guardian_id: i64,
    pub relationship: String,
    pub is_primary: bool,
    pub is_emergency: bool,
    pub can_pickup: bool,
}
