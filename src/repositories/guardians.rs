//! Guardians and student ↔ guardian links.

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

use crate::error::{RepoError, RepoResult};

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

#[derive(Debug, Clone)]
pub struct NewGuardian {
    pub user_id: Option<i64>,
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub occupation: Option<String>,
    pub address: Option<String>,
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

#[derive(Clone)]
pub struct GuardianRepo { pool: SqlitePool }

impl GuardianRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn create(&self, g: &NewGuardian) -> RepoResult<Guardian> {
        let id = sqlx::query_scalar::<_, i64>(
            r#"INSERT INTO guardian
               (user_id, first_name, last_name, phone, email, occupation, address)
               VALUES (?, ?, ?, ?, ?, ?, ?) RETURNING id"#,
        )
        .bind(g.user_id).bind(&g.first_name).bind(&g.last_name)
        .bind(&g.phone).bind(&g.email).bind(&g.occupation).bind(&g.address)
        .fetch_one(&self.pool).await?;
        self.get(id).await
    }

    pub async fn get(&self, id: i64) -> RepoResult<Guardian> {
        sqlx::query_as::<_, Guardian>("SELECT * FROM guardian WHERE id = ?")
            .bind(id).fetch_optional(&self.pool).await?
            .ok_or(RepoError::NotFound)
    }

    pub async fn list(&self, limit: i64, offset: i64) -> RepoResult<Vec<Guardian>> {
        Ok(sqlx::query_as::<_, Guardian>(
            "SELECT * FROM guardian ORDER BY last_name, first_name LIMIT ? OFFSET ?",
        ).bind(limit).bind(offset).fetch_all(&self.pool).await?)
    }

    pub async fn delete(&self, id: i64) -> RepoResult<()> {
        let res = sqlx::query("DELETE FROM guardian WHERE id = ?")
            .bind(id).execute(&self.pool).await?;
        if res.rows_affected() == 0 { return Err(RepoError::NotFound); }
        Ok(())
    }

    // ---- Linking ----

    pub async fn link(&self, link: &StudentGuardianLink) -> RepoResult<()> {
        sqlx::query(
            r#"INSERT INTO student_guardian
                 (student_id, guardian_id, relationship, is_primary, is_emergency, can_pickup)
               VALUES (?, ?, ?, ?, ?, ?)
               ON CONFLICT(student_id, guardian_id) DO UPDATE SET
                 relationship = excluded.relationship,
                 is_primary   = excluded.is_primary,
                 is_emergency = excluded.is_emergency,
                 can_pickup   = excluded.can_pickup"#,
        )
        .bind(link.student_id).bind(link.guardian_id).bind(&link.relationship)
        .bind(link.is_primary as i64).bind(link.is_emergency as i64).bind(link.can_pickup as i64)
        .execute(&self.pool).await?;
        Ok(())
    }

    pub async fn unlink(&self, student_id: i64, guardian_id: i64) -> RepoResult<()> {
        sqlx::query("DELETE FROM student_guardian WHERE student_id = ? AND guardian_id = ?")
            .bind(student_id).bind(guardian_id).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn guardians_of_student(&self, student_id: i64) -> RepoResult<Vec<Guardian>> {
        Ok(sqlx::query_as::<_, Guardian>(
            r#"SELECT g.* FROM guardian g
               INNER JOIN student_guardian sg ON sg.guardian_id = g.id
               WHERE sg.student_id = ?
               ORDER BY sg.is_primary DESC, g.last_name"#,
        ).bind(student_id).fetch_all(&self.pool).await?)
    }

    pub async fn students_of_guardian(&self, guardian_id: i64) -> RepoResult<Vec<i64>> {
        Ok(sqlx::query_scalar::<_, i64>(
            "SELECT student_id FROM student_guardian WHERE guardian_id = ?",
        ).bind(guardian_id).fetch_all(&self.pool).await?)
    }
}
