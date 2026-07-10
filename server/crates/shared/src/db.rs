use sqlx::sqlite::{SqlitePool, SqliteConnectOptions, SqliteJournalMode, SqliteSynchronous};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct TenantDatabaseManager {
    pools: Arc<RwLock<HashMap<String, SqlitePool>>>,
    test_db_path: String,
}

impl TenantDatabaseManager {
    pub fn new(test_db_path: &str) -> Self {
        Self {
            pools: Arc::new(RwLock::new(HashMap::new())),
            test_db_path: test_db_path.to_string(),
        }
    }

    pub async fn get_pool(&self, tenant_id: Option<&str>) -> Result<SqlitePool, sqlx::Error> {
        let tenant_id = tenant_id.unwrap_or(&self.test_db_path);
        
        {
            let pools = self.pools.read().await;
            if let Some(pool) = pools.get(tenant_id) {
                return Ok(pool.clone());
            }
        }
        
        let mut pools = self.pools.write().await;
        // Check again after acquiring write lock
        if let Some(pool) = pools.get(tenant_id) {
            return Ok(pool.clone());
        }

        let db_url = format!("{}.db", tenant_id);
        let options = SqliteConnectOptions::new()
            .filename(db_url)
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal)
            .synchronous(SqliteSynchronous::Normal);

        let pool = SqlitePool::connect_with(options).await?;
        
        run_migrations(&pool).await?;

        pools.insert(tenant_id.to_string(), pool.clone());
        Ok(pool)
    }
}

async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Ensure tables exist
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS students (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL
        )",
    )
    .execute(pool)
    .await?;

    // Insert test data if empty
    sqlx::query(
        "INSERT OR IGNORE INTO students (id, name) VALUES (1, 'vishal Doe')"
    )
    .execute(pool)
    .await?;

    Ok(())
}
