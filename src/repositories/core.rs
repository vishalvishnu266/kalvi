//! Core reference data: [`School`], [`AcademicYear`], [`Term`].

use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

use crate::error::{RepoError, RepoResult};

// ---------- School ----------

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

#[derive(Clone)]
pub struct SchoolRepo {
    pool: SqlitePool,
}

impl SchoolRepo {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Fetch the single-tenant school profile (row `id = 1`).
    pub async fn get(&self) -> RepoResult<School> {
        sqlx::query_as::<_, School>("SELECT * FROM school WHERE id = 1")
            .fetch_optional(&self.pool)
            .await?
            .ok_or(RepoError::NotFound)
    }

    /// Upsert the single school row.
    pub async fn upsert(&self, s: &UpdateSchool) -> RepoResult<School> {
        let existing = sqlx::query_scalar::<_, i64>("SELECT id FROM school WHERE id = 1")
            .fetch_optional(&self.pool)
            .await?;

        if existing.is_none() {
            let name = s
                .name
                .clone()
                .ok_or_else(|| RepoError::validation("school.name is required"))?;
            sqlx::query(
                r#"INSERT INTO school (id, name, code, address, phone, email, logo_path, currency, timezone)
                   VALUES (1, ?, ?, ?, ?, ?, ?, COALESCE(?, 'USD'), COALESCE(?, 'UTC'))"#,
            )
            .bind(name)
            .bind(&s.code)
            .bind(&s.address)
            .bind(&s.phone)
            .bind(&s.email)
            .bind(&s.logo_path)
            .bind(&s.currency)
            .bind(&s.timezone)
            .execute(&self.pool)
            .await?;
        } else {
            sqlx::query(
                r#"UPDATE school SET
                     name       = COALESCE(?, name),
                     code       = COALESCE(?, code),
                     address    = COALESCE(?, address),
                     phone      = COALESCE(?, phone),
                     email      = COALESCE(?, email),
                     logo_path  = COALESCE(?, logo_path),
                     currency   = COALESCE(?, currency),
                     timezone   = COALESCE(?, timezone),
                     updated_at = datetime('now')
                   WHERE id = 1"#,
            )
            .bind(&s.name)
            .bind(&s.code)
            .bind(&s.address)
            .bind(&s.phone)
            .bind(&s.email)
            .bind(&s.logo_path)
            .bind(&s.currency)
            .bind(&s.timezone)
            .execute(&self.pool)
            .await?;
        }
        self.get().await
    }
}

// ---------- Academic Year ----------

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

#[derive(Clone)]
pub struct AcademicYearRepo {
    pool: SqlitePool,
}

impl AcademicYearRepo {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, y: &NewAcademicYear) -> RepoResult<AcademicYear> {
        if y.start_date >= y.end_date {
            return Err(RepoError::validation("start_date must be before end_date"));
        }
        let mut tx = self.pool.begin().await?;

        if y.is_current {
            sqlx::query("UPDATE academic_year SET is_current = 0 WHERE is_current = 1")
                .execute(&mut *tx)
                .await?;
        }

        let id = sqlx::query_scalar::<_, i64>(
            r#"INSERT INTO academic_year (name, start_date, end_date, is_current)
               VALUES (?, ?, ?, ?)
               RETURNING id"#,
        )
        .bind(&y.name)
        .bind(y.start_date)
        .bind(y.end_date)
        .bind(y.is_current as i64)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        self.get(id).await
    }

    pub async fn get(&self, id: i64) -> RepoResult<AcademicYear> {
        sqlx::query_as::<_, AcademicYear>("SELECT * FROM academic_year WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?
            .ok_or(RepoError::NotFound)
    }

    pub async fn current(&self) -> RepoResult<AcademicYear> {
        sqlx::query_as::<_, AcademicYear>("SELECT * FROM academic_year WHERE is_current = 1")
            .fetch_optional(&self.pool)
            .await?
            .ok_or(RepoError::NotFound)
    }

    pub async fn list(&self) -> RepoResult<Vec<AcademicYear>> {
        Ok(
            sqlx::query_as::<_, AcademicYear>("SELECT * FROM academic_year ORDER BY start_date DESC")
                .fetch_all(&self.pool)
                .await?,
        )
    }

    pub async fn set_current(&self, id: i64) -> RepoResult<()> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("UPDATE academic_year SET is_current = 0 WHERE is_current = 1")
            .execute(&mut *tx)
            .await?;
        let res = sqlx::query("UPDATE academic_year SET is_current = 1 WHERE id = ?")
            .bind(id)
            .execute(&mut *tx)
            .await?;
        if res.rows_affected() == 0 {
            return Err(RepoError::NotFound);
        }
        tx.commit().await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> RepoResult<()> {
        let res = sqlx::query("DELETE FROM academic_year WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        if res.rows_affected() == 0 {
            return Err(RepoError::NotFound);
        }
        Ok(())
    }
}

// ---------- Term ----------

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

#[derive(Clone)]
pub struct TermRepo {
    pool: SqlitePool,
}

impl TermRepo {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, t: &NewTerm) -> RepoResult<Term> {
        if t.start_date >= t.end_date {
            return Err(RepoError::validation("start_date must be before end_date"));
        }
        let id = sqlx::query_scalar::<_, i64>(
            r#"INSERT INTO term (academic_year_id, name, start_date, end_date)
               VALUES (?, ?, ?, ?) RETURNING id"#,
        )
        .bind(t.academic_year_id)
        .bind(&t.name)
        .bind(t.start_date)
        .bind(t.end_date)
        .fetch_one(&self.pool)
        .await?;
        self.get(id).await
    }

    pub async fn get(&self, id: i64) -> RepoResult<Term> {
        sqlx::query_as::<_, Term>("SELECT * FROM term WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?
            .ok_or(RepoError::NotFound)
    }

    pub async fn list_for_year(&self, academic_year_id: i64) -> RepoResult<Vec<Term>> {
        Ok(sqlx::query_as::<_, Term>(
            "SELECT * FROM term WHERE academic_year_id = ? ORDER BY start_date",
        )
        .bind(academic_year_id)
        .fetch_all(&self.pool)
        .await?)
    }

    pub async fn delete(&self, id: i64) -> RepoResult<()> {
        let res = sqlx::query("DELETE FROM term WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        if res.rows_affected() == 0 {
            return Err(RepoError::NotFound);
        }
        Ok(())
    }
}
