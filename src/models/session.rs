//! Opaque server-side session tokens (per tenant).
//!
//! Cookies carry only the session id. Everything else — user id, expiry, IP,
//! user-agent — lives in the DB. Revoking a session is a single DELETE.
//!
//! Token generation: 32 bytes from `OsRng`, base64url-encoded (no padding),
//! which yields ~43 chars — well within any cookie limit.

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

use crate::errors::AppError;

/// Session lifetime: 7 days. Adjust in one place if we later add
/// "remember me" toggle.
pub const SESSION_LIFETIME_DAYS: i64 = 7;

/// Name of the session cookie. `Host-` prefix would require HTTPS + Secure
/// + Path=/; keep the plain name for dev over http.
pub const SESSION_COOKIE_NAME: &str = "sd_session";

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Session {
    pub id: String,
    pub user_id: String,
    pub created_at: Option<String>,
    pub expires_at: String, // ISO-8601 datetime; SQLite compares strings correctly.
    pub ip: Option<String>,
    pub user_agent: Option<String>,
}

impl Session {
    /// Generate a fresh, cryptographically-random session id.
    pub fn new_token() -> String {
        let mut bytes = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut bytes);
        URL_SAFE_NO_PAD.encode(bytes)
    }

    /// Insert a new session row. Returns the session id (== cookie value).
    pub async fn create(
        pool: &SqlitePool,
        user_id: &str,
        ip: Option<&str>,
        user_agent: Option<&str>,
    ) -> Result<String, AppError> {
        let id = Self::new_token();
        let expires_at = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::days(SESSION_LIFETIME_DAYS))
            .unwrap_or_else(chrono::Utc::now)
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();
        sqlx::query(
            r#"INSERT INTO sessions (id, user_id, expires_at, ip, user_agent)
               VALUES (?, ?, ?, ?, ?)"#,
        )
        .bind(&id)
        .bind(user_id)
        .bind(&expires_at)
        .bind(ip)
        .bind(user_agent)
        .execute(pool)
        .await?;
        Ok(id)
    }

    /// Fetch a session by id, but only if it hasn't expired.
    pub async fn find_valid(pool: &SqlitePool, id: &str) -> Result<Option<Self>, AppError> {
        Ok(sqlx::query_as::<_, Session>(
            "SELECT * FROM sessions WHERE id = ? AND expires_at > CURRENT_TIMESTAMP",
        )
        .bind(id)
        .fetch_optional(pool)
        .await?)
    }

    /// Delete a specific session (logout).
    pub async fn delete(pool: &SqlitePool, id: &str) -> Result<(), AppError> {
        sqlx::query("DELETE FROM sessions WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// House-keeping: prune expired sessions.
    pub async fn purge_expired(pool: &SqlitePool) -> Result<u64, AppError> {
        let res = sqlx::query("DELETE FROM sessions WHERE expires_at <= CURRENT_TIMESTAMP")
            .execute(pool)
            .await?;
        Ok(res.rows_affected())
    }
}
