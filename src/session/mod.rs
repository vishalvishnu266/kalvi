use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::Duration;

use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;

use crate::error::{RepoError, RepoResult};
use crate::repositories::auth::{NewSession, Session, SessionRepo};

#[derive(Debug, Clone)]
pub enum SessionBackendConfig {
    TenantDb,
    MemorySqlite { snapshot_root: PathBuf },
}

#[derive(Clone)]
pub enum SessionStore {
    TenantDb(SessionRepo),
    MemorySqlite(MemorySqliteSessionStore),
}

impl SessionStore {
    pub async fn create(&self, s: &NewSession) -> RepoResult<Session> {
        match self {
            Self::TenantDb(repo) => repo.create(s).await,
            Self::MemorySqlite(store) => store.create(s).await,
        }
    }

    pub async fn find_active_by_token(&self, token: &str) -> RepoResult<Option<Session>> {
        match self {
            Self::TenantDb(repo) => repo.find_active_by_token(token).await,
            Self::MemorySqlite(store) => store.find_active_by_token(token).await,
        }
    }

    pub async fn touch(&self, id: i64) -> RepoResult<()> {
        match self {
            Self::TenantDb(repo) => repo.touch(id).await,
            Self::MemorySqlite(store) => store.touch(id).await,
        }
    }

    pub async fn revoke_by_token(&self, token: &str) -> RepoResult<()> {
        match self {
            Self::TenantDb(repo) => repo.revoke_by_token(token).await,
            Self::MemorySqlite(store) => store.revoke_by_token(token).await,
        }
    }

    pub async fn revoke_all_for_user(&self, user_id: i64) -> RepoResult<()> {
        match self {
            Self::TenantDb(repo) => repo.revoke_all_for_user(user_id).await,
            Self::MemorySqlite(store) => store.revoke_all_for_user(user_id).await,
        }
    }

    pub async fn cleanup(&self, keep_days: i64) -> RepoResult<u64> {
        match self {
            Self::TenantDb(repo) => repo.cleanup(keep_days).await,
            Self::MemorySqlite(store) => store.cleanup(keep_days).await,
        }
    }

    pub async fn checkpoint(&self) -> RepoResult<()> {
        match self {
            Self::TenantDb(_) => Ok(()),
            Self::MemorySqlite(store) => store.checkpoint().await,
        }
    }
}

#[derive(Clone)]
pub struct MemorySqliteSessionStore {
    pool: SqlitePool,
    snapshot_path: PathBuf,
}

impl MemorySqliteSessionStore {
    pub async fn open(snapshot_path: PathBuf, namespace: &str) -> RepoResult<Self> {
        let url = format!("sqlite:file:session_{}?mode=memory&cache=shared", namespace);
        let opts = SqliteConnectOptions::from_str(&url)
            .map_err(RepoError::from)?
            .create_if_missing(true)
            .foreign_keys(true)
            .busy_timeout(Duration::from_secs(5));
        let pool = SqlitePoolOptions::new()
            .max_connections(4)
            .connect_with(opts)
            .await?;
        let s = Self { pool, snapshot_path };
        s.ensure_schema().await?;
        s.restore_snapshot().await?;
        Ok(s)
    }

    async fn ensure_schema(&self) -> RepoResult<()> {
        sqlx::query(
            r#"
CREATE TABLE IF NOT EXISTS user_session (
    id            INTEGER PRIMARY KEY,
    token         TEXT    NOT NULL UNIQUE,
    user_id       INTEGER NOT NULL,
    created_at    TEXT    NOT NULL DEFAULT (datetime('now')),
    expires_at    TEXT    NOT NULL,
    last_seen_at  TEXT    NOT NULL DEFAULT (datetime('now')),
    revoked_at    TEXT,
    user_agent    TEXT,
    remote_ip     TEXT
)"#,
        )
        .execute(&self.pool)
        .await?;
        sqlx::query("CREATE INDEX IF NOT EXISTS ix_user_session_user ON user_session(user_id)")
            .execute(&self.pool)
            .await?;
        sqlx::query(
            "CREATE INDEX IF NOT EXISTS ix_user_session_active ON user_session(expires_at) WHERE revoked_at IS NULL",
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn restore_snapshot(&self) -> RepoResult<()> {
        if !self.snapshot_path.exists() {
            return Ok(());
        }
        let escaped = self
            .snapshot_path
            .to_string_lossy()
            .replace('\'', "''");
        let attach_sql = format!("ATTACH DATABASE '{escaped}' AS snapshot_db");
        sqlx::query(&attach_sql).execute(&self.pool).await?;
        let has_table = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM snapshot_db.sqlite_master WHERE type = 'table' AND name = 'user_session'",
        )
        .fetch_one(&self.pool)
        .await?;
        if has_table > 0 {
            sqlx::query("DELETE FROM user_session").execute(&self.pool).await?;
            sqlx::query(
                r#"
INSERT INTO user_session (id, token, user_id, created_at, expires_at, last_seen_at, revoked_at, user_agent, remote_ip)
SELECT id, token, user_id, created_at, expires_at, last_seen_at, revoked_at, user_agent, remote_ip
FROM snapshot_db.user_session
"#,
            )
            .execute(&self.pool)
            .await?;
        }
        sqlx::query("DETACH DATABASE snapshot_db")
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    fn ensure_snapshot_dir(path: &Path) -> RepoResult<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| RepoError::validation(format!("session snapshot dir: {e}")))?;
        }
        Ok(())
    }
}

impl MemorySqliteSessionStore {
    pub async fn create(&self, s: &NewSession) -> RepoResult<Session> {
        let id = sqlx::query_scalar::<_, i64>(
            r#"INSERT INTO user_session (token, user_id, expires_at, user_agent, remote_ip)
               VALUES (?, ?, ?, ?, ?) RETURNING id"#,
        )
        .bind(&s.token)
        .bind(s.user_id)
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
        Self::ensure_snapshot_dir(&self.snapshot_path)?;
        if self.snapshot_path.exists() {
            std::fs::remove_file(&self.snapshot_path)
                .map_err(|e| RepoError::validation(format!("remove old session snapshot: {e}")))?;
        }
        let path = self.snapshot_path.to_string_lossy().to_string();
        sqlx::query("VACUUM INTO ?").bind(path).execute(&self.pool).await?;
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
