use sqlx::{sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous}, migrate::Migrator, SqlitePool};
use std::{collections::HashMap, path::Path, sync::Arc};
use tokio::sync::RwLock;
use tracing::info;

pub static MASTER_MIGRATOR: Migrator = sqlx::migrate!("./resources/migrations/master");
pub static TENANT_MIGRATOR: Migrator = sqlx::migrate!("./resources/migrations/tenant");

#[derive(Clone)]
pub struct DatabaseConfig {
    pub master_pool: SqlitePool,
    tenant_pools: Arc<RwLock<HashMap<String, SqlitePool>>>,
}

impl DatabaseConfig {
    pub async fn check_health(&self) -> bool {
        sqlx::query("SELECT 1").execute(&self.master_pool).await.is_ok()
    }

    pub async fn new() -> Result<Self, sqlx::Error> {
        info!("Initializing Master Database...");
        if !Path::new("data").exists() {
            std::fs::create_dir_all("data").map_err(|e| sqlx::Error::Configuration(Box::new(e)))?;
        }

        let master_options = SqliteConnectOptions::new()
            .filename("data/master.db")
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal)
            .synchronous(SqliteSynchronous::Normal);

        let master_pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(master_options)
            .await?;

        MASTER_MIGRATOR.run(&master_pool).await?;

        Ok(Self {
            master_pool,
            tenant_pools: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    pub async fn get_tenant_pool(&self, db_name: &str) -> Result<SqlitePool, sqlx::Error> {
        info!("Resolving Database Pool for: {}", db_name);
        {
            let pools = self.tenant_pools.read().await;
            if let Some(pool) = pools.get(db_name) {
                return Ok(pool.clone());
            }
        }

        let mut pools = self.tenant_pools.write().await;
        if let Some(pool) = pools.get(db_name) {
            return Ok(pool.clone());
        }

        let options = SqliteConnectOptions::new()
            .filename(format!("data/{}.db", db_name))
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal)
            .synchronous(SqliteSynchronous::Normal);

        let pool = SqlitePoolOptions::new()
            .max_connections(10)
            .connect_with(options)
            .await?;

        TENANT_MIGRATOR.run(&pool).await?;

        pools.insert(db_name.to_string(), pool.clone());
        Ok(pool)
    }
}
