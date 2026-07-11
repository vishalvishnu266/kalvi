use sqlx::sqlite::{SqlitePool, SqliteConnectOptions, SqliteJournalMode, SqliteSynchronous};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct TenantDatabaseManager {
    pools: Arc<RwLock<HashMap<String, SqlitePool>>>,
    master_pool: SqlitePool,
}

impl TenantDatabaseManager {
    pub async fn new() -> Result<Self, sqlx::Error> {
        // Create master database for tenant registry
        let master_options = SqliteConnectOptions::new()
            .filename("master.db")
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal)
            .synchronous(SqliteSynchronous::Normal);

        let master_pool = SqlitePool::connect_with(master_options).await?;
        
        // Initialize master database schema
        init_master_database(&master_pool).await?;

        Ok(Self {
            pools: Arc::new(RwLock::new(HashMap::new())),
            master_pool,
        })
    }

    /// Get the master database pool (for tenant registry)
    pub fn get_master_pool(&self) -> SqlitePool {
        self.master_pool.clone()
    }

    /// Get a tenant-specific database pool by database name
    pub async fn get_tenant_pool(&self, database_name: &str) -> Result<SqlitePool, sqlx::Error> {
        // Check if pool already exists
        {
            let pools = self.pools.read().await;
            if let Some(pool) = pools.get(database_name) {
                return Ok(pool.clone());
            }
        }
        
        // Create new pool
        let mut pools = self.pools.write().await;
        // Double-check after acquiring write lock
        if let Some(pool) = pools.get(database_name) {
            return Ok(pool.clone());
        }

        let db_url = format!("{}.db", database_name);
        let options = SqliteConnectOptions::new()
            .filename(db_url)
            .create_if_missing(false) // Don't auto-create tenant DBs
            .journal_mode(SqliteJournalMode::Wal)
            .synchronous(SqliteSynchronous::Normal);

        let pool = SqlitePool::connect_with(options).await?;

        pools.insert(database_name.to_string(), pool.clone());
        Ok(pool)
    }
}

async fn init_master_database(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Create tenants table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tenants (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            slug TEXT NOT NULL UNIQUE,
            name TEXT NOT NULL,
            contact_email TEXT NOT NULL,
            contact_phone TEXT NOT NULL,
            address TEXT NOT NULL,
            database_name TEXT NOT NULL UNIQUE,
            is_active BOOLEAN NOT NULL DEFAULT 1,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        )
        "#,
    )
    .execute(pool)
    .await?;
    
    // Create index on slug for fast lookups
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_tenants_slug ON tenants(slug)")
        .execute(pool)
        .await?;

    Ok(())
}
