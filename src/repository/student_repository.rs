use sqlx::{SqlitePool, Row};
use serde::Serialize;
use crate::util::errors::AppError;

#[derive(Debug, Clone, Serialize)]
pub struct Student {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub enrollment_number: Option<String>,
    pub created_at: i64,
}

pub struct StudentRepository;

impl StudentRepository {
    pub async fn create(
        pool: &SqlitePool,
        first_name: &str,
        last_name: &str,
        email: Option<String>,
        phone: Option<String>,
        enrollment_number: Option<String>,
        created_at: i64,
    ) -> Result<i64, AppError> {
        let result = sqlx::query(
            "INSERT INTO students (first_name, last_name, email, phone, enrollment_number, created_at) 
             VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(first_name)
        .bind(last_name)
        .bind(email)
        .bind(phone)
        .bind(enrollment_number)
        .bind(created_at)
        .execute(pool)
        .await
        .map_err(|e| AppError::RuntimeException(e.to_string()))?;

        Ok(result.last_insert_rowid())
    }

    pub async fn count(pool: &SqlitePool) -> Result<i64, AppError> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM students")
            .fetch_one(pool)
            .await
            .map_err(|e| AppError::RuntimeException(e.to_string()))?;
        
        Ok(row.get("count"))
    }
}
