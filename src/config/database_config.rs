use sqlx::{sqlite::SqlitePool, Pool, Sqlite, Executor};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::fs;
use std::path::Path;
use tracing::{info, warn, error};
use crate::util::errors::AppError;

pub struct DatabaseConfig {
    pub master_pool: SqlitePool,
    pub tenant_pools: Arc<RwLock<HashMap<String, SqlitePool>>>,
}

impl DatabaseConfig {
    pub async fn new(master_db_url: &str) -> Result<Self, AppError> {
        // Ensure data directories exist
        fs::create_dir_all("data/tenant").map_err(|e| AppError::RuntimeException(e.to_string()))?;

        let master_pool = SqlitePool::connect(master_db_url)
            .await
            .map_err(|e| AppError::RuntimeException(e.to_string()))?;

        // Run master migrations
        Self::run_manual_migrations(&master_pool, "resources/migrations/master").await?;

        Ok(Self {
            master_pool,
            tenant_pools: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    pub async fn get_tenant_pool(&self, slug: &str) -> Result<SqlitePool, AppError> {
        // 1. Check if pool is already in registry (Read Lock)
        {
            let registry = self.tenant_pools.read().await;
            if let Some(pool) = registry.get(slug) {
                return Ok(pool.clone());
            }
        }

        // 2. Not found, create new pool (Write Lock)
        let mut registry = self.tenant_pools.write().await;
        
        // Double-check after acquiring write lock
        if let Some(pool) = registry.get(slug) {
            return Ok(pool.clone());
        }

        let db_path = format!("data/tenant/{}.db", slug);
        let db_url = format!("sqlite://{}?mode=rwc", db_path);

        info!("Initializing database for tenant: {}", slug);
        
        let pool = SqlitePool::connect(&db_url)
            .await
            .map_err(|e| AppError::RuntimeException(e.to_string()))?;

        // Run tenant migrations
        Self::run_manual_migrations(&pool, "resources/migrations/tenant").await?;

        registry.insert(slug.to_string(), pool.clone());
        Ok(pool)
    }

    pub async fn run_manual_migrations(pool: &SqlitePool, path: &str) -> Result<(), AppError> {
        // 1. Create migration tracking table
        sqlx::query("CREATE TABLE IF NOT EXISTS _manual_migrations (version TEXT PRIMARY KEY)")
            .execute(pool)
            .await
            .map_err(|e| AppError::RuntimeException(e.to_string()))?;

        let mut entries = fs::read_dir(path)
            .map_err(|e| AppError::RuntimeException(e.to_string()))?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()
            .map_err(|e| AppError::RuntimeException(e.to_string()))?;

        entries.sort();

        for entry in entries {
            if let Some(filename) = entry.file_name().and_then(|n| n.to_str()) {
                if !filename.ends_with(".sql") {
                    continue;
                }

                // Check if already applied
                let row: Option<(String,)> = sqlx::query_as("SELECT version FROM _manual_migrations WHERE version = ?")
                    .bind(filename)
                    .fetch_optional(pool)
                    .await
                    .map_err(|e| AppError::RuntimeException(e.to_string()))?;

                if row.is_none() {
                    info!("Applying migration: {}", filename);
                    let content = fs::read_to_string(&entry)
                        .map_err(|e| AppError::RuntimeException(e.to_string()))?;

                    let mut tx = pool.begin().await
                        .map_err(|e| AppError::RuntimeException(e.to_string()))?;

                    // Split by ; and execute (simplistic parser)
                    for statement in content.split(';') {
                        let trimmed = statement.trim();
                        if !trimmed.is_empty() {
                            tx.execute(trimmed).await
                                .map_err(|e| AppError::RuntimeException(format!("Error in {}: {}", filename, e)))?;
                        }
                    }

                    sqlx::query("INSERT INTO _manual_migrations (version) VALUES (?)")
                        .bind(filename)
                        .execute(&mut *tx)
                        .await
                        .map_err(|e| AppError::RuntimeException(e.to_string()))?;

                    tx.commit().await
                        .map_err(|e| AppError::RuntimeException(e.to_string()))?;
                }
            }
        }

        Ok(())
    }
}
