use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

use crate::error::{RepoError, RepoResult};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct HealthRecord {
    pub id: i64,
    pub student_id: i64,
    pub height_cm: Option<f64>,
    pub weight_kg: Option<f64>,
    pub allergies: Option<String>,
    pub conditions: Option<String>,
    pub updated_at: NaiveDateTime,
}

#[derive(Clone)]
pub struct HealthRecordRepo { pool: SqlitePool }

impl HealthRecordRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn upsert(
        &self, student_id: i64,
        height_cm: Option<f64>, weight_kg: Option<f64>,
        allergies: Option<&str>, conditions: Option<&str>,
    ) -> RepoResult<HealthRecord> {
        sqlx::query(
            r#"INSERT INTO health_record
                 (student_id, height_cm, weight_kg, allergies, conditions)
               VALUES (?, ?, ?, ?, ?)
               ON CONFLICT(student_id) DO UPDATE SET
                 height_cm  = COALESCE(excluded.height_cm,  health_record.height_cm),
                 weight_kg  = COALESCE(excluded.weight_kg,  health_record.weight_kg),
                 allergies  = COALESCE(excluded.allergies,  health_record.allergies),
                 conditions = COALESCE(excluded.conditions, health_record.conditions),
                 updated_at = datetime('now')"#,
        )
        .bind(student_id).bind(height_cm).bind(weight_kg)
        .bind(allergies).bind(conditions)
        .execute(&self.pool).await?;

        self.for_student(student_id).await?.ok_or(RepoError::NotFound)
    }

    pub async fn for_student(&self, student_id: i64) -> RepoResult<Option<HealthRecord>> {
        Ok(sqlx::query_as::<_, HealthRecord>(
            "SELECT * FROM health_record WHERE student_id = ?",
        ).bind(student_id).fetch_optional(&self.pool).await?)
    }
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Vaccination {
    pub id: i64,
    pub student_id: i64,
    pub vaccine_name: String,
    pub dose: Option<String>,
    pub given_on: NaiveDate,
}

#[derive(Clone)]
pub struct VaccinationRepo { pool: SqlitePool }

impl VaccinationRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn record(
        &self, student_id: i64, vaccine_name: &str,
        dose: Option<&str>, given_on: NaiveDate,
    ) -> RepoResult<i64> {
        Ok(sqlx::query_scalar::<_, i64>(
            r#"INSERT INTO vaccination (student_id, vaccine_name, dose, given_on)
               VALUES (?, ?, ?, ?) RETURNING id"#,
        )
        .bind(student_id).bind(vaccine_name).bind(dose).bind(given_on)
        .fetch_one(&self.pool).await?)
    }

    pub async fn for_student(&self, student_id: i64) -> RepoResult<Vec<Vaccination>> {
        Ok(sqlx::query_as::<_, Vaccination>(
            "SELECT * FROM vaccination WHERE student_id = ? ORDER BY given_on DESC",
        ).bind(student_id).fetch_all(&self.pool).await?)
    }
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ClinicVisit {
    pub id: i64,
    pub student_id: i64,
    pub visited_at: NaiveDateTime,
    pub complaint: Option<String>,
    pub treatment: Option<String>,
    pub attended_by_staff_id: Option<i64>,
}

#[derive(Clone)]
pub struct ClinicVisitRepo { pool: SqlitePool }

impl ClinicVisitRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn record(
        &self, student_id: i64,
        complaint: Option<&str>, treatment: Option<&str>,
        attended_by_staff_id: Option<i64>,
    ) -> RepoResult<i64> {
        Ok(sqlx::query_scalar::<_, i64>(
            r#"INSERT INTO clinic_visit
                 (student_id, complaint, treatment, attended_by_staff_id)
               VALUES (?, ?, ?, ?) RETURNING id"#,
        )
        .bind(student_id).bind(complaint).bind(treatment).bind(attended_by_staff_id)
        .fetch_one(&self.pool).await?)
    }

    pub async fn for_student(&self, student_id: i64) -> RepoResult<Vec<ClinicVisit>> {
        Ok(sqlx::query_as::<_, ClinicVisit>(
            "SELECT * FROM clinic_visit WHERE student_id = ? ORDER BY visited_at DESC",
        ).bind(student_id).fetch_all(&self.pool).await?)
    }
}
