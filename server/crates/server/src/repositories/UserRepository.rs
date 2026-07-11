use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::{Duration, Utc};
use crate::models::UserModel::User;
use crate::models::SessionModel::Session;

const SESSION_DURATION_DAYS: i64 = 7;

pub async fn get_user_by_username(
    pool: &SqlitePool,
    username: &str,
) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = ?")
        .bind(username)
        .fetch_optional(pool)
        .await
}

pub async fn get_user_by_id(pool: &SqlitePool, id: i64) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn create_session(
    pool: &SqlitePool,
    user_id: i64,
    ip_address: Option<String>,
    user_agent: Option<String>,
) -> Result<Session, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let created_at = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let expires_at = (Utc::now() + Duration::days(SESSION_DURATION_DAYS))
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();

    sqlx::query(
        "INSERT INTO sessions (id, user_id, ip_address, user_agent, created_at, expires_at)
         VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(user_id)
    .bind(&ip_address)
    .bind(&user_agent)
    .bind(&created_at)
    .bind(&expires_at)
    .execute(pool)
    .await?;

    Ok(Session {
        id,
        user_id,
        ip_address,
        user_agent,
        created_at,
        expires_at,
    })
}

pub async fn get_valid_session(
    pool: &SqlitePool,
    session_id: &str,
) -> Result<Option<Session>, sqlx::Error> {
    let session: Option<Session> = sqlx::query_as::<_, Session>(
        "SELECT * FROM sessions WHERE id = ? AND expires_at > datetime('now')",
    )
    .bind(session_id)
    .fetch_optional(pool)
    .await?;

    Ok(session)
}

pub async fn delete_session(pool: &SqlitePool, session_id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM sessions WHERE id = ?")
        .bind(session_id)
        .execute(pool)
        .await?;
    Ok(())
}
