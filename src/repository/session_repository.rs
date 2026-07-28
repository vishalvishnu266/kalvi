use sqlx::SqlitePool;

use crate::entity::session::{NewSession, Session};
use crate::exception::repo_error::{RepoError, RepoResult};

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

pub async fn get(pool: &SqlitePool, id: i64) -> RepoResult<Session> {
    sqlx::query_as::<_, Session>("SELECT * FROM user_session WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or(RepoError::NotFound)
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
