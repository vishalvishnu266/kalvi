use sqlx::migrate::Migrator;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePool, SqliteSynchronous};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Migrations for the master database (tenant registry).
pub static MASTER_MIGRATOR: Migrator = sqlx::migrate!("./db/migrations/master");

/// Migrations for each per-tenant database.
pub static TENANT_MIGRATOR: Migrator = sqlx::migrate!("./db/migrations/tenant");

pub struct TenantDatabaseManager {
    master_pool: SqlitePool,
    pools: Arc<RwLock<HashMap<String, SqlitePool>>>,
}

impl TenantDatabaseManager {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let master_options = SqliteConnectOptions::new()
            .filename("db/master.db")
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

    pub fn master_pool(&self) -> SqlitePool {
        self.master_pool.clone()
    }

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

        let filename = format!("db/{}.db", database_name);
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
