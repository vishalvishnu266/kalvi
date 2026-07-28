use std::str::FromStr;
use std::time::Duration;

use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous};
use sqlx::SqlitePool;

use crate::exception::repo_error::{RepoError, RepoResult};

// ── Tenant DB (per tenant) ─────────────────────────────────────────

pub async fn connect_tenant(url: &str) -> RepoResult<SqlitePool> {
    tracing::debug!("database_config::connect_tenant: {}", url);
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

pub async fn migrate_tenant(pool: &SqlitePool) -> RepoResult<()> {
    tracing::debug!("database_config::migrate_tenant");
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}

// ── System (master) DB ─────────────────────────────────────────────

pub async fn connect_system(url: &str) -> RepoResult<SqlitePool> {
    tracing::debug!("database_config::connect_system: {}", url);
    let opts = SqliteConnectOptions::from_str(url)
        .map_err(RepoError::from)?
        .create_if_missing(true)
        .foreign_keys(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal)
        .busy_timeout(Duration::from_secs(5));

    Ok(SqlitePoolOptions::new()
        .max_connections(4)
        .connect_with(opts)
        .await?)
}

pub async fn migrate_system(pool: &SqlitePool) -> RepoResult<()> {
    tracing::debug!("database_config::migrate_system");
    sqlx::migrate!("./migrations_system").run(pool).await?;
    Ok(())
}

// ── Session store DB ───────────────────────────────────────────────

pub async fn connect_sessions(url: &str) -> RepoResult<SqlitePool> {
    tracing::debug!("database_config::connect_sessions: {}", url);
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
    tracing::debug!("database_config::migrate_sessions");
    sqlx::migrate!("./migrations_session")
        .run(pool)
        .await
        .map_err(RepoError::Migrate)?;
    Ok(())
}
