use sqlx::{sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous}, SqlitePool};
use std::{collections::HashMap, path::Path, sync::Arc};
use tokio::sync::RwLock;
use tracing::{info, warn};

#[derive(Clone)]
pub struct DatabaseConfig {
    pub master_pool: SqlitePool,
    tenant_pools: Arc<RwLock<HashMap<String, SqlitePool>>>,
}

impl DatabaseConfig {
    async fn run_manual_migrations(pool: &SqlitePool, path: &str) -> Result<(), sqlx::Error> {
        info!("Running migrations from: {}", path);
        
        // 1. Create migration table if not exists
        sqlx::query("CREATE TABLE IF NOT EXISTS _manual_migrations (version TEXT PRIMARY KEY)").execute(pool).await?;

        // 2. Read migration files
        let entries = std::fs::read_dir(path).map_err(|e| sqlx::Error::Configuration(Box::new(e)))?;
        let mut files: Vec<_> = entries
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("sql"))
            .collect();

        // Sort by filename to ensure sequential application
        files.sort_by_key(|e| e.file_name());

        for file in files {
            let filename = file.file_name().into_string().unwrap();
            
            // 3. Check if migration already applied
            let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM _manual_migrations WHERE version = ?)")
                .bind(&filename)
                .fetch_one(pool)
                .await?;

            if !exists {
                info!("Applying migration: {}", filename);
                let sql = std::fs::read_to_string(file.path()).map_err(|e| sqlx::Error::Configuration(Box::new(e)))?;
                
                // Execute SQL (SQLx split queries by semicolon isn't perfect for all dialects, but for these it works)
                // We use a transaction for safety
                let mut tx = pool.begin().await?;
                sqlx::query(&sql).execute(&mut *tx).await?;
                sqlx::query("INSERT INTO _manual_migrations (version) VALUES (?)").bind(&filename).execute(&mut *tx).await?;
                tx.commit().await?;
            }
        }
        Ok(())
    }

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

        Self::run_manual_migrations(&master_pool, "./resources/migrations/master").await?;

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

        Self::run_manual_migrations(&pool, "./resources/migrations/tenant").await?;

        pools.insert(db_name.to_string(), pool.clone());
        Ok(pool)
    }
}
