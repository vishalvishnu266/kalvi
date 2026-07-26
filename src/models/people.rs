use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// ── Staff ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Staff {
    pub id: i64,
    pub employee_no: String,
    pub user_id: Option<i64>,
    pub department_id: Option<i64>,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: Option<NaiveDate>,
    pub gender: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub designation: Option<String>,
    pub employment_type: Option<String>,
    pub date_of_joining: NaiveDate,
    pub date_of_leaving: Option<NaiveDate>,
    pub status: String,
    pub photo_path: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewStaff {
    pub employee_no: String,
    pub user_id: Option<i64>,
    pub department_id: Option<i64>,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: Option<NaiveDate>,
    pub gender: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub designation: Option<String>,
    pub employment_type: Option<String>,
    pub date_of_joining: NaiveDate,
    pub photo_path: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct UpdateStaff {
    pub department_id: Option<Option<i64>>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub designation: Option<String>,
    pub employment_type: Option<String>,
    pub date_of_leaving: Option<Option<NaiveDate>>,
    pub status: Option<String>,
    pub photo_path: Option<String>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Department {
    pub id: i64,
    pub name: String,
}
