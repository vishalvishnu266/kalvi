use std::str::FromStr;
use std::time::Duration;

use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous};
use sqlx::SqlitePool;

use crate::error::{RepoError, RepoResult};

/// Build a connection pool with sane SQLite defaults for a single-tenant ERP.
///
/// * `foreign_keys = ON`
/// * WAL journal mode + `synchronous = NORMAL` for good write throughput
/// * 5s busy timeout so short lock contention doesn't fail immediately
pub async fn connect(url: &str) -> RepoResult<SqlitePool> {
    let opts = SqliteConnectOptions::from_str(url)
        .map_err(RepoError::from)?
        .create_if_missing(true)
        .foreign_keys(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal)
        .busy_timeout(Duration::from_secs(5));

    let pool = SqlitePoolOptions::new()
        .max_connections(8)
        .connect_with(opts)
        .await?;

    Ok(pool)
}

/// Run all embedded migrations from `./migrations`.
pub async fn migrate(pool: &SqlitePool) -> RepoResult<()> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}
