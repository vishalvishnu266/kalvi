use std::str::FromStr;
use std::time::Duration;

use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;

use crate::error::{RepoError, RepoResult};
use crate::repositories::auth::{NewSession, Session};

#[derive(Clone)]
pub struct SessionStore {
    pool: SqlitePool,
}

impl SessionStore {
    pub async fn open(url: &str) -> RepoResult<Self> {
        let opts = SqliteConnectOptions::from_str(url)
            .map_err(RepoError::from)?
            .create_if_missing(true)
            .foreign_keys(true)
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
            .synchronous(sqlx::sqlite::SqliteSynchronous::Normal)
            .busy_timeout(Duration::from_secs(5));
        let pool = SqlitePoolOptions::new()
            .max_connections(8)
            .connect_with(opts)
            .await?;
        let s = Self { pool };
        s.ensure_schema().await?;
        Ok(s)
    }

    pub async fn pool_clone(&self) -> SqlitePool {
        self.pool.clone()
    }

    async fn ensure_schema(&self) -> RepoResult<()> {
        sqlx::migrate!("migrations_session")
            .run(&self.pool)
            .await
            .map_err(RepoError::from)?;
        Ok(())
    }

    pub async fn create(&self, s: &NewSession) -> RepoResult<Session> {
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
        .fetch_one(&self.pool)
        .await?;
        self.get(id).await
    }

    pub async fn find_active_by_token(&self, token: &str) -> RepoResult<Option<Session>> {
        Ok(sqlx::query_as::<_, Session>(
            r#"SELECT * FROM user_session
               WHERE token = ?
                 AND revoked_at IS NULL
                 AND expires_at > datetime('now')"#,
        )
        .bind(token)
        .fetch_optional(&self.pool)
        .await?)
    }

    pub async fn touch(&self, id: i64) -> RepoResult<()> {
        sqlx::query("UPDATE user_session SET last_seen_at = datetime('now') WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn revoke_by_token(&self, token: &str) -> RepoResult<()> {
        sqlx::query(
            r#"UPDATE user_session SET revoked_at = datetime('now')
               WHERE token = ? AND revoked_at IS NULL"#,
        )
        .bind(token)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn revoke_all_for_user(&self, user_id: i64) -> RepoResult<()> {
        sqlx::query(
            r#"UPDATE user_session SET revoked_at = datetime('now')
               WHERE user_id = ? AND revoked_at IS NULL"#,
        )
        .bind(user_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn cleanup(&self, keep_days: i64) -> RepoResult<u64> {
        let res = sqlx::query(
            r#"DELETE FROM user_session
               WHERE (revoked_at IS NOT NULL AND revoked_at < datetime('now', ?))
                  OR (expires_at < datetime('now', ?))"#,
        )
        .bind(format!("-{keep_days} days"))
        .bind(format!("-{keep_days} days"))
        .execute(&self.pool)
        .await?;
        Ok(res.rows_affected())
    }

    pub async fn checkpoint(&self) -> RepoResult<()> {
        Ok(())
    }

    async fn get(&self, id: i64) -> RepoResult<Session> {
        sqlx::query_as::<_, Session>("SELECT * FROM user_session WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?
            .ok_or(RepoError::NotFound)
    }
}
