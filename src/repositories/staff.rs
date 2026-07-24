use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

use crate::error::{RepoError, RepoResult};

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

#[derive(Clone)]
pub struct StaffRepo { pool: SqlitePool }

impl StaffRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn create(&self, s: &NewStaff) -> RepoResult<Staff> {
        let id = sqlx::query_scalar::<_, i64>(
            r#"INSERT INTO staff (
                employee_no, user_id, department_id, first_name, last_name,
                date_of_birth, gender, phone, email, designation,
                employment_type, date_of_joining, photo_path
              ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) RETURNING id"#,
        )
        .bind(&s.employee_no).bind(s.user_id).bind(s.department_id)
        .bind(&s.first_name).bind(&s.last_name)
        .bind(s.date_of_birth).bind(&s.gender)
        .bind(&s.phone).bind(&s.email).bind(&s.designation)
        .bind(&s.employment_type).bind(s.date_of_joining).bind(&s.photo_path)
        .fetch_one(&self.pool).await?;
        self.get(id).await
    }

    pub async fn get(&self, id: i64) -> RepoResult<Staff> {
        sqlx::query_as::<_, Staff>("SELECT * FROM staff WHERE id = ?")
            .bind(id).fetch_optional(&self.pool).await?
            .ok_or(RepoError::NotFound)
    }

    pub async fn find_by_employee_no(&self, no: &str) -> RepoResult<Option<Staff>> {
        Ok(sqlx::query_as::<_, Staff>("SELECT * FROM staff WHERE employee_no = ?")
            .bind(no).fetch_optional(&self.pool).await?)
    }

    pub async fn list(&self, limit: i64, offset: i64) -> RepoResult<Vec<Staff>> {
        Ok(sqlx::query_as::<_, Staff>(
            "SELECT * FROM staff ORDER BY last_name, first_name LIMIT ? OFFSET ?",
        ).bind(limit).bind(offset).fetch_all(&self.pool).await?)
    }

    pub async fn list_active(&self) -> RepoResult<Vec<Staff>> {
        Ok(sqlx::query_as::<_, Staff>(
            "SELECT * FROM staff WHERE status = 'active' ORDER BY last_name, first_name",
        ).fetch_all(&self.pool).await?)
    }

    pub async fn update(&self, id: i64, u: &UpdateStaff) -> RepoResult<Staff> {
        sqlx::query(
            r#"UPDATE staff SET
                 department_id    = COALESCE(?, department_id),
                 first_name       = COALESCE(?, first_name),
                 last_name        = COALESCE(?, last_name),
                 phone            = COALESCE(?, phone),
                 email            = COALESCE(?, email),
                 designation      = COALESCE(?, designation),
                 employment_type  = COALESCE(?, employment_type),
                 date_of_leaving  = COALESCE(?, date_of_leaving),
                 status           = COALESCE(?, status),
                 photo_path       = COALESCE(?, photo_path),
                 updated_at       = datetime('now')
               WHERE id = ?"#,
        )
        .bind(u.department_id.unwrap_or(None))
        .bind(&u.first_name).bind(&u.last_name)
        .bind(&u.phone).bind(&u.email).bind(&u.designation)
        .bind(&u.employment_type)
        .bind(u.date_of_leaving.unwrap_or(None))
        .bind(&u.status).bind(&u.photo_path).bind(id)
        .execute(&self.pool).await?;
        self.get(id).await
    }

    pub async fn delete(&self, id: i64) -> RepoResult<()> {
        let res = sqlx::query("DELETE FROM staff WHERE id = ?")
            .bind(id).execute(&self.pool).await?;
        if res.rows_affected() == 0 { return Err(RepoError::NotFound); }
        Ok(())
    }
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Department {
    pub id: i64,
    pub name: String,
}
