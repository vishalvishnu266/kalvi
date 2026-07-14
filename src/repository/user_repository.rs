use sqlx::{SqlitePool, Row};
use serde::Serialize;
use crate::util::errors::AppError;

#[derive(Debug, Clone, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
    pub role: String,
    pub full_name: Option<String>,
    pub created_at: i64,
}

pub struct UserRepository;

impl UserRepository {
    pub async fn find_by_username(pool: &SqlitePool, username: &str) -> Result<Option<User>, AppError> {
        let row = sqlx::query("SELECT id, username, password_hash, role, full_name, created_at FROM users WHERE username = ?")
            .bind(username)
            .fetch_optional(pool)
            .await
            .map_err(|e| AppError::RuntimeException(e.to_string()))?;

        match row {
            Some(row) => Ok(Some(User {
                id: row.get("id"),
                username: row.get("username"),
                password_hash: row.get("password_hash"),
                role: row.get("role"),
                full_name: row.get("full_name"),
                created_at: row.get("created_at"),
            })),
            None => Ok(None),
        }
    }

    pub async fn find_by_session(pool: &SqlitePool, session_id: &str, now: i64) -> Result<Option<User>, AppError> {
        let row = sqlx::query(
            "SELECT u.id, u.username, u.password_hash, u.role, u.full_name, u.created_at 
             FROM users u 
             JOIN sessions s ON u.id = s.user_id 
             WHERE s.id = ? AND s.expires_at > ?"
        )
        .bind(session_id)
        .bind(now)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::RuntimeException(e.to_string()))?;

        match row {
            Some(row) => Ok(Some(User {
                id: row.get("id"),
                username: row.get("username"),
                password_hash: row.get("password_hash"),
                role: row.get("role"),
                full_name: row.get("full_name"),
                created_at: row.get("created_at"),
            })),
            None => Ok(None),
        }
    }

    pub async fn create_session(
        pool: &SqlitePool,
        session_id: &str,
        user_id: i64,
        expires_at: i64,
        now: i64,
    ) -> Result<(), AppError> {
        sqlx::query("INSERT INTO sessions (id, user_id, expires_at, created_at) VALUES (?, ?, ?, ?)")
            .bind(session_id)
            .bind(user_id)
            .bind(expires_at)
            .bind(now)
            .execute(pool)
            .await
            .map_err(|e| AppError::RuntimeException(e.to_string()))?;
        Ok(())
    }
}
