//! System (master) database: connection, migrations, and shared row types.
//!
//! There is no wrapper struct here anymore — the raw `SqlitePool` is
//! stored directly on `AppState`, and the read/write functions that
//! used to be `SystemRegistry` methods now live in
//! [`crate::services::system`] as free functions.

use std::str::FromStr;
use std::time::Duration;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous};
use sqlx::{FromRow, SqlitePool};

use crate::error::{RepoError, RepoResult};

pub async fn connect_system(url: &str) -> RepoResult<SqlitePool> {
    tracing::debug!("connect_system: connecting to {}", url);
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
    tracing::debug!("migrate_system: running migrations_system");
    sqlx::migrate!("./migrations_system").run(pool).await?;
    Ok(())
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Tenant {
    pub id: i64,
    pub tenant_id: String,
    pub name: String,
    pub status: String,
    pub plan: Option<String>,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NewTenant {
    pub tenant_id: String,
    pub name: String,
    pub plan: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UpdateTenant {
    pub name: Option<String>,
    pub plan: Option<String>,
    pub notes: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PortalUser {
    pub id: i64,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NewPortalUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PortalMembership {
    pub id: i64,
    pub portal_user_id: i64,
    pub tenant_id: String,
    pub tenant_user_id: i64,
    pub role: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NewPortalMembership {
    pub portal_user_id: i64,
    pub tenant_id: String,
    pub tenant_user_id: i64,
    pub role: String,
}
