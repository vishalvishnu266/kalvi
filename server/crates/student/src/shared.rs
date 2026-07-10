use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

/// Core Student model used across all student features
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Student {
    pub id: i64,
    pub name: String,
}

/// Common database operations shared across features
pub mod db {
    use super::*;
    
    /// Get a single student by ID - used by multiple features
    pub async fn get_student_by_id(pool: &SqlitePool, id: i64) -> Result<Option<Student>, sqlx::Error> {
        sqlx::query_as::<_, Student>("SELECT id, name FROM students WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
    }
    
    /// List all students - used by list and search features
    pub async fn list_students(pool: &SqlitePool) -> Result<Vec<Student>, sqlx::Error> {
        sqlx::query_as::<_, Student>("SELECT id, name FROM students ORDER BY name")
            .fetch_all(pool)
            .await
    }
}
