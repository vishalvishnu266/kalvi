use sqlx::SqlitePool;
use uuid::Uuid;
use crate::models::academic_year::{AcademicYear, AcademicYearForm};
use crate::errors::AppError;

pub struct AcademicYearService;

impl AcademicYearService {
    pub async fn list_all(pool: &SqlitePool) -> Result<Vec<AcademicYear>, AppError> {
        Ok(sqlx::query_as::<_, AcademicYear>(
            "SELECT * FROM academic_years ORDER BY start_date DESC",
        )
        .fetch_all(pool)
        .await?)
    }

    pub async fn find(pool: &SqlitePool, id: &str) -> Result<Option<AcademicYear>, AppError> {
        Ok(sqlx::query_as::<_, AcademicYear>(
            "SELECT * FROM academic_years WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(pool)
        .await?)
    }

    pub async fn current(pool: &SqlitePool) -> Result<Option<AcademicYear>, AppError> {
        Ok(sqlx::query_as::<_, AcademicYear>(
            "SELECT * FROM academic_years WHERE is_current = 1 LIMIT 1",
        )
        .fetch_optional(pool)
        .await?)
    }

    pub async fn create(pool: &SqlitePool, form: AcademicYearForm) -> Result<AcademicYear, AppError> {
        let id = Uuid::new_v4().to_string();
        sqlx::query(
            r#"INSERT INTO academic_years (id, name, start_date, end_date, status)
               VALUES (?, ?, ?, ?, ?)"#,
        )
        .bind(&id)
        .bind(form.name.trim())
        .bind(form.start_date.trim())
        .bind(form.end_date.trim())
        .bind(form.effective_status())
        .execute(pool)
        .await?;

        Ok(Self::find(pool, &id).await?.unwrap())
    }

    pub async fn update(pool: &SqlitePool, id: &str, form: AcademicYearForm) -> Result<AcademicYear, AppError> {
        sqlx::query(
            r#"UPDATE academic_years
               SET name = ?, start_date = ?, end_date = ?, status = ?, updated_at = CURRENT_TIMESTAMP
               WHERE id = ?"#,
        )
        .bind(form.name.trim())
        .bind(form.start_date.trim())
        .bind(form.end_date.trim())
        .bind(form.effective_status())
        .bind(id)
        .execute(pool)
        .await?;

        Ok(Self::find(pool, id).await?.unwrap())
    }

    pub async fn delete(pool: &SqlitePool, id: &str) -> Result<(), AppError> {
        sqlx::query("DELETE FROM academic_years WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn set_current(pool: &SqlitePool, id: &str) -> Result<(), AppError> {
        let mut tx = pool.begin().await?;
        sqlx::query("UPDATE academic_years SET is_current = 0 WHERE is_current = 1")
            .execute(&mut *tx)
            .await?;
        sqlx::query(
            "UPDATE academic_years SET is_current = 1, updated_at = CURRENT_TIMESTAMP WHERE id = ?",
        )
        .bind(id)
        .execute(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(())
    }
}
