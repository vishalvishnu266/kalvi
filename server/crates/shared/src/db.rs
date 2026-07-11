use sqlx::migrate::Migrator;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePool, SqliteSynchronous};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Migrations for the master database (tenant registry).
/// Located at `<workspace-root>/migrations/master` (i.e. `server/migrations/master`).
/// The path is resolved at compile time relative to this crate's `Cargo.toml`
/// (`server/crates/shared/Cargo.toml`), hence the `../../` prefix.
pub static MASTER_MIGRATOR: Migrator = sqlx::migrate!("../../migrations/master");

/// Migrations for each per-tenant database.
/// Located at `<workspace-root>/migrations/tenant`.
pub static TENANT_MIGRATOR: Migrator = sqlx::migrate!("../../migrations/tenant");

/// Manages one master SQLite pool and a cache of per-tenant pools.
///
/// Master migrations are applied on construction. Tenant migrations are
/// applied lazily the first time a tenant pool is opened, and are
/// idempotent (sqlx tracks applied versions in `_sqlx_migrations`).
pub struct TenantDatabaseManager {
    master_pool: SqlitePool,
    pools: Arc<RwLock<HashMap<String, SqlitePool>>>,
}

impl TenantDatabaseManager {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let master_options = SqliteConnectOptions::new()
            .filename("master.db")
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal)
            .synchronous(SqliteSynchronous::Normal);

        let master_pool = SqlitePool::connect_with(master_options).await?;

        MASTER_MIGRATOR
            .run(&master_pool)
            .await
            .map_err(|e| sqlx::Error::Configuration(Box::new(e)))?;

        Ok(Self {
            master_pool,
            pools: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Master pool (tenant registry).
    pub fn master_pool(&self) -> SqlitePool {
        self.master_pool.clone()
    }

    /// Get or create a tenant pool by `database_name` (a filename stem).
    /// The DB file is created if missing, then tenant migrations are applied.
    pub async fn tenant_pool(&self, database_name: &str) -> Result<SqlitePool, sqlx::Error> {
        {
            let pools = self.pools.read().await;
            if let Some(pool) = pools.get(database_name) {
                return Ok(pool.clone());
            }
        }

        let mut pools = self.pools.write().await;
        if let Some(pool) = pools.get(database_name) {
            return Ok(pool.clone());
        }

        let filename = format!("{}.db", database_name);
        let options = SqliteConnectOptions::new()
            .filename(Path::new(&filename))
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal)
            .synchronous(SqliteSynchronous::Normal);

        let pool = SqlitePool::connect_with(options).await?;

        TENANT_MIGRATOR
            .run(&pool)
            .await
            .map_err(|e| sqlx::Error::Configuration(Box::new(e)))?;

        pools.insert(database_name.to_string(), pool.clone());
        Ok(pool)
    }
}
