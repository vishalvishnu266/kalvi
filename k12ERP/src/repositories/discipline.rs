//! Discipline incidents.

use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

use crate::error::{RepoError, RepoResult};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct DisciplineIncident {
    pub id: i64,
    pub student_id: i64,
    pub date: NaiveDate,
    pub description: String,
    pub severity: Option<String>,
    pub action_taken: Option<String>,
    pub reported_by_staff_id: Option<i64>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct NewIncident {
    pub student_id: i64,
    pub date: NaiveDate,
    pub description: String,
    pub severity: Option<String>,
    pub action_taken: Option<String>,
    pub reported_by_staff_id: Option<i64>,
}

#[derive(Clone)]
pub struct DisciplineRepo { pool: SqlitePool }

impl DisciplineRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn report(&self, i: &NewIncident) -> RepoResult<DisciplineIncident> {
        if let Some(sev) = &i.severity {
            if !matches!(sev.as_str(), "low"|"medium"|"high") {
                return Err(RepoError::validation("severity must be low|medium|high"));
            }
        }
        let id = sqlx::query_scalar::<_, i64>(
            r#"INSERT INTO discipline_incident
                 (student_id, date, description, severity, action_taken, reported_by_staff_id)
               VALUES (?, ?, ?, ?, ?, ?) RETURNING id"#,
        )
        .bind(i.student_id).bind(i.date).bind(&i.description)
        .bind(&i.severity).bind(&i.action_taken).bind(i.reported_by_staff_id)
        .fetch_one(&self.pool).await?;

        sqlx::query_as::<_, DisciplineIncident>(
            "SELECT * FROM discipline_incident WHERE id = ?",
        ).bind(id).fetch_one(&self.pool).await.map_err(Into::into)
    }

    pub async fn for_student(&self, student_id: i64) -> RepoResult<Vec<DisciplineIncident>> {
        Ok(sqlx::query_as::<_, DisciplineIncident>(
            "SELECT * FROM discipline_incident WHERE student_id = ? ORDER BY date DESC",
        ).bind(student_id).fetch_all(&self.pool).await?)
    }

    pub async fn between(&self, from: NaiveDate, to: NaiveDate)
        -> RepoResult<Vec<DisciplineIncident>>
    {
        Ok(sqlx::query_as::<_, DisciplineIncident>(
            "SELECT * FROM discipline_incident WHERE date BETWEEN ? AND ? ORDER BY date DESC",
        ).bind(from).bind(to).fetch_all(&self.pool).await?)
    }

    pub async fn delete(&self, id: i64) -> RepoResult<()> {
        let res = sqlx::query("DELETE FROM discipline_incident WHERE id = ?")
            .bind(id).execute(&self.pool).await?;
        if res.rows_affected() == 0 { return Err(RepoError::NotFound); }
        Ok(())
    }
}
