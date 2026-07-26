//! Session store (dedicated `sessions.db`): connection, migrations,
//! and free-function SQL over `user_session`.
//!
//! There is no wrapper struct here anymore — the raw `SqlitePool` is
//! stored directly on `AppState` (mirroring how `system.rs` handles
//! the master DB). The functions that used to be `SessionStore`
//! methods now live here as free functions and take `&SqlitePool` as
//! their first argument.

use std::str::FromStr;
use std::time::Duration;

use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous};
use sqlx::SqlitePool;

use crate::error::{RepoError, RepoResult};
use crate::models::auth::{NewSession, Session};

// ── Connection + migrations ─────────────────────────────────────────

pub async fn connect_sessions(url: &str) -> RepoResult<SqlitePool> {
    tracing::debug!("connect_sessions: connecting to {}", url);
    let opts = SqliteConnectOptions::from_str(url)
        .map_err(RepoError::from)?
        .create_if_missing(true)
        .foreign_keys(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal)
        .busy_timeout(Duration::from_secs(5));

    Ok(SqlitePoolOptions::new()
        .max_connections(8)
        .connect_with(opts)
        .await?)
}

pub async fn migrate_sessions(pool: &SqlitePool) -> RepoResult<()> {
    tracing::debug!("migrate_sessions: running migrations_session");
    sqlx::migrate!("./migrations_session")
        .run(pool)
        .await
        .map_err(RepoError::Migrate)?;
    Ok(())
}

// ── SQL free functions (were `SessionStore` methods) ────────────────

pub async fn create(pool: &SqlitePool, s: &NewSession) -> RepoResult<Session> {
    let id = sqlx::query_scalar::<_, i64>(
        r#"INSERT INTO user_session (token, user_id, tenant_id, expires_at, user_agent, remote_ip)
           VALUES (?, ?, ?, ?, ?, ?) RETURNING id"#,
    )
    .bind(&s.token)
    .bind(s.user_id)
    .bind(&s.tenant_id)
    .bind(s.expires_at)
    .bind(&s.user_agent)
    .bind(&s.remote_ip)
    .fetch_one(pool)
    .await?;
    get(pool, id).await
}

pub async fn find_active_by_token(pool: &SqlitePool, token: &str) -> RepoResult<Option<Session>> {
    Ok(sqlx::query_as::<_, Session>(
        r#"SELECT * FROM user_session
           WHERE token = ?
             AND revoked_at IS NULL
             AND expires_at > datetime('now')"#,
    )
    .bind(token)
    .fetch_optional(pool)
    .await?)
}

pub async fn touch(pool: &SqlitePool, id: i64) -> RepoResult<()> {
    sqlx::query("UPDATE user_session SET last_seen_at = datetime('now') WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn revoke_by_token(pool: &SqlitePool, token: &str) -> RepoResult<()> {
    sqlx::query(
        r#"UPDATE user_session SET revoked_at = datetime('now')
           WHERE token = ? AND revoked_at IS NULL"#,
    )
    .bind(token)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn revoke_all_for_user(pool: &SqlitePool, user_id: i64) -> RepoResult<()> {
    sqlx::query(
        r#"UPDATE user_session SET revoked_at = datetime('now')
           WHERE user_id = ? AND revoked_at IS NULL"#,
    )
    .bind(user_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn cleanup(pool: &SqlitePool, keep_days: i64) -> RepoResult<u64> {
    let res = sqlx::query(
        r#"DELETE FROM user_session
           WHERE (revoked_at IS NOT NULL AND revoked_at < datetime('now', ?))
              OR (expires_at < datetime('now', ?))"#,
    )
    .bind(format!("-{keep_days} days"))
    .bind(format!("-{keep_days} days"))
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

async fn get(pool: &SqlitePool, id: i64) -> RepoResult<Session> {
    sqlx::query_as::<_, Session>("SELECT * FROM user_session WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or(RepoError::NotFound)
}
