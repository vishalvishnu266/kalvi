use sqlx::{sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous}, SqlitePool};
use std::{collections::HashMap, path::Path, sync::Arc};
use tokio::sync::RwLock;
use tracing::info;

#[derive(Clone)]
pub struct DatabaseConfig {
    pub master_pool: SqlitePool,
    tenant_pools: Arc<RwLock<HashMap<String, SqlitePool>>>,
}

impl DatabaseConfig {
    pub async fn new() -> Result<Self, sqlx::Error> {
        info!("Initializing Master Database Registry...");
        
        // Ensure directory structure
        if !Path::new("data/tenant").exists() {
            std::fs::create_dir_all("data/tenant").map_err(|e| sqlx::Error::Configuration(Box::new(e)))?;
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

        // 1. Run Master Migrations
        Self::run_manual_migrations(&master_pool, "./resources/migrations/master").await?;

        Ok(Self {
            master_pool,
            tenant_pools: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Dynamically resolves or opens an isolated SQLite database for a specific tenant.
    pub async fn get_tenant_pool(&self, tenant_name: &str) -> Result<SqlitePool, sqlx::Error> {
        // Cache Check
        {
            let pools = self.tenant_pools.read().await;
            if let Some(pool) = pools.get(tenant_name) {
                return Ok(pool.clone());
            }
        }

        let mut pools = self.tenant_pools.write().await;
        if let Some(pool) = pools.get(tenant_name) {
            return Ok(pool.clone());
        }

        info!("Opening isolated DB for tenant: {}", tenant_name);
        
        let options = SqliteConnectOptions::new()
            .filename(format!("data/tenant/{}.db", tenant_name))
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal)
            .synchronous(SqliteSynchronous::Normal);

        let pool = SqlitePoolOptions::new()
            .max_connections(10)
            .connect_with(options)
            .await?;

        // 2. Run Tenant Migrations
        Self::run_manual_migrations(&pool, "./resources/migrations/tenant").await?;

        pools.insert(tenant_name.to_string(), pool.clone());
        Ok(pool)
    }

    /// A macro-free migration runner that reads SQL files from disk.
    async fn run_manual_migrations(pool: &SqlitePool, path: &str) -> Result<(), sqlx::Error> {
        info!("Scanning for migrations in: {}", path);
        
        sqlx::query("CREATE TABLE IF NOT EXISTS _manual_migrations (version TEXT PRIMARY KEY)").execute(pool).await?;

        let entries = std::fs::read_dir(path).map_err(|e| sqlx::Error::Configuration(Box::new(e)))?;
        let mut files: Vec<_> = entries
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("sql"))
            .collect();

        files.sort_by_key(|e| e.file_name());

        for file in files {
            let filename = file.file_name().into_string().unwrap();
            
            let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM _manual_migrations WHERE version = ?)")
                .bind(&filename)
                .fetch_one(pool)
                .await?;

            if !exists {
                info!("Manual Migration -> Applying: {}", filename);
                let sql = std::fs::read_to_string(file.path()).map_err(|e| sqlx::Error::Configuration(Box::new(e)))?;
                
                let mut tx = pool.begin().await?;
                
                // Naive but effective splitter for standard schema SQL
                for statement in sql.split(';').map(|s| s.trim()).filter(|s| !s.is_empty()) {
                    sqlx::query(statement).execute(&mut *tx).await?;
                }
                
                sqlx::query("INSERT INTO _manual_migrations (version) VALUES (?)").bind(&filename).execute(&mut *tx).await?;
                tx.commit().await?;
            }
        }
        Ok(())
    }
}
