use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// ── School ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct School {
    pub id: i64,
    pub name: String,
    pub code: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub logo_path: Option<String>,
    pub currency: String,
    pub timezone: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Default, Clone)]
pub struct UpdateSchool {
    pub name: Option<String>,
    pub code: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub logo_path: Option<String>,
    pub currency: Option<String>,
    pub timezone: Option<String>,
}

// ── Academic year / term ────────────────────────────────────────────

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct AcademicYear {
    pub id: i64,
    pub name: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub is_current: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewAcademicYear {
    pub name: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub is_current: bool,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Term {
    pub id: i64,
    pub academic_year_id: i64,
    pub name: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewTerm {
    pub academic_year_id: i64,
    pub name: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

// ── Academic structure ──────────────────────────────────────────────

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Grade {
    pub id: i64,
    pub name: String,
    pub level: i64,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Section {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Room {
    pub id: i64,
    pub name: String,
    pub capacity: Option<i64>,
    pub kind: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewRoom {
    pub name: String,
    pub capacity: Option<i64>,
    pub kind: Option<String>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Subject {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub is_elective: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewSubject {
    pub code: String,
    pub name: String,
    pub is_elective: bool,
}
