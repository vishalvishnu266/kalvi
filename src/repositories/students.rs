use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

use crate::error::{RepoError, RepoResult};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Student {
    pub id: i64,
    pub admission_no: String,
    pub user_id: Option<i64>,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub date_of_birth: NaiveDate,
    pub gender: Option<String>,
    pub blood_group: Option<String>,
    pub nationality: Option<String>,
    pub religion: Option<String>,
    pub photo_path: Option<String>,
    pub admission_date: NaiveDate,
    pub status: String,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewStudent {
    pub admission_no: String,
    pub user_id: Option<i64>,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub date_of_birth: NaiveDate,
    pub gender: Option<String>,
    pub blood_group: Option<String>,
    pub nationality: Option<String>,
    pub religion: Option<String>,
    pub photo_path: Option<String>,
    pub admission_date: NaiveDate,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct UpdateStudent {
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub gender: Option<String>,
    pub blood_group: Option<String>,
    pub nationality: Option<String>,
    pub religion: Option<String>,
    pub photo_path: Option<String>,
    pub status: Option<String>,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
}

#[derive(Clone)]
pub struct StudentRepo { pool: SqlitePool }

impl StudentRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn create(&self, s: &NewStudent) -> RepoResult<Student> {
        let id = sqlx::query_scalar::<_, i64>(
            r#"INSERT INTO student (
                admission_no, user_id, first_name, middle_name, last_name,
                date_of_birth, gender, blood_group, nationality, religion, photo_path,
                admission_date, address_line1, address_line2, city, state, postal_code, country
              ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) RETURNING id"#,
        )
        .bind(&s.admission_no).bind(s.user_id)
        .bind(&s.first_name).bind(&s.middle_name).bind(&s.last_name)
        .bind(s.date_of_birth).bind(&s.gender).bind(&s.blood_group)
        .bind(&s.nationality).bind(&s.religion).bind(&s.photo_path)
        .bind(s.admission_date)
        .bind(&s.address_line1).bind(&s.address_line2)
        .bind(&s.city).bind(&s.state).bind(&s.postal_code).bind(&s.country)
        .fetch_one(&self.pool).await?;
        self.get(id).await
    }

    pub async fn get(&self, id: i64) -> RepoResult<Student> {
        sqlx::query_as::<_, Student>("SELECT * FROM student WHERE id = ?")
            .bind(id).fetch_optional(&self.pool).await?
            .ok_or(RepoError::NotFound)
    }

    pub async fn find_by_admission_no(&self, no: &str) -> RepoResult<Option<Student>> {
        Ok(sqlx::query_as::<_, Student>("SELECT * FROM student WHERE admission_no = ?")
            .bind(no).fetch_optional(&self.pool).await?)
    }

    pub async fn list(&self, limit: i64, offset: i64) -> RepoResult<Vec<Student>> {
        Ok(sqlx::query_as::<_, Student>(
            "SELECT * FROM student ORDER BY last_name, first_name LIMIT ? OFFSET ?",
        ).bind(limit).bind(offset).fetch_all(&self.pool).await?)
    }

pub async fn list_by_ids(&self, ids: &[i64]) -> RepoResult<Vec<Student>> {
        if ids.is_empty() { return Ok(Vec::new()); }

let placeholders = std::iter::repeat("?").take(ids.len()).collect::<Vec<_>>().join(",");
        let sql = format!(
            "SELECT * FROM student WHERE id IN ({placeholders})
             ORDER BY last_name, first_name",
        );
        let mut q = sqlx::query_as::<_, Student>(&sql);
        for id in ids { q = q.bind(id); }
        Ok(q.fetch_all(&self.pool).await?)
    }

pub async fn find_by_user_id(&self, user_id: i64) -> RepoResult<Option<Student>> {
        Ok(sqlx::query_as::<_, Student>(
            "SELECT * FROM student WHERE user_id = ? LIMIT 1",
        ).bind(user_id).fetch_optional(&self.pool).await?)
    }

    pub async fn search(&self, q: &str, limit: i64) -> RepoResult<Vec<Student>> {
        let like = format!("%{}%", q);
        Ok(sqlx::query_as::<_, Student>(
            r#"SELECT * FROM student
               WHERE first_name LIKE ? OR last_name LIKE ? OR admission_no LIKE ?
               ORDER BY last_name, first_name LIMIT ?"#,
        )
        .bind(&like).bind(&like).bind(&like).bind(limit)
        .fetch_all(&self.pool).await?)
    }

    pub async fn count_by_status(&self, status: &str) -> RepoResult<i64> {
        Ok(sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM student WHERE status = ?",
        ).bind(status).fetch_one(&self.pool).await?)
    }

    pub async fn update(&self, id: i64, u: &UpdateStudent) -> RepoResult<Student> {
        sqlx::query(
            r#"UPDATE student SET
                 first_name    = COALESCE(?, first_name),
                 middle_name   = COALESCE(?, middle_name),
                 last_name     = COALESCE(?, last_name),
                 gender        = COALESCE(?, gender),
                 blood_group   = COALESCE(?, blood_group),
                 nationality   = COALESCE(?, nationality),
                 religion      = COALESCE(?, religion),
                 photo_path    = COALESCE(?, photo_path),
                 status        = COALESCE(?, status),
                 address_line1 = COALESCE(?, address_line1),
                 address_line2 = COALESCE(?, address_line2),
                 city          = COALESCE(?, city),
                 state         = COALESCE(?, state),
                 postal_code   = COALESCE(?, postal_code),
                 country       = COALESCE(?, country),
                 updated_at    = datetime('now')
               WHERE id = ?"#,
        )
        .bind(&u.first_name).bind(&u.middle_name).bind(&u.last_name)
        .bind(&u.gender).bind(&u.blood_group)
        .bind(&u.nationality).bind(&u.religion).bind(&u.photo_path)
        .bind(&u.status)
        .bind(&u.address_line1).bind(&u.address_line2)
        .bind(&u.city).bind(&u.state).bind(&u.postal_code).bind(&u.country)
        .bind(id).execute(&self.pool).await?;
        self.get(id).await
    }

    pub async fn set_status(&self, id: i64, status: &str) -> RepoResult<()> {
        sqlx::query("UPDATE student SET status = ?, updated_at = datetime('now') WHERE id = ?")
            .bind(status).bind(id).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> RepoResult<()> {
        let res = sqlx::query("DELETE FROM student WHERE id = ?")
            .bind(id).execute(&self.pool).await?;
        if res.rows_affected() == 0 { return Err(RepoError::NotFound); }
        Ok(())
    }
}
