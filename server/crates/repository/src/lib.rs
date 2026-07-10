use sqlx::sqlite::{SqlitePool, SqliteConnectOptions, SqliteJournalMode, SqliteSynchronous};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

mod migrations;
pub mod repositories;

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
        
        migrations::run_migrations(&pool).await?;

        pools.insert(tenant_id.to_string(), pool.clone());
        Ok(pool)
    }
}

pub use repositories::student_repository::get_student;
