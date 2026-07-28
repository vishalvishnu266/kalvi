use sqlx::SqlitePool;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::config::app_config::AppConfig;
use crate::config::database_config;
use crate::exception::repo_error::RepoResult;
use crate::exception::tenant_error::TenantError;
use crate::tenant::tenant_id::TenantId;

/// Manages per-tenant SqlitePool handles: lazy-open on first request,
/// evict on disable/delete, snapshot at shutdown.
#[derive(Clone)]
pub struct TenantPoolService {
    config: AppConfig,
    tenants: Arc<RwLock<HashMap<TenantId, SqlitePool>>>,
}

impl TenantPoolService {
    pub fn new(config: AppConfig) -> Self {
        Self {
            config,
            tenants: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn pool_for(&self, tenant: &str) -> Result<SqlitePool, TenantError> {
        if let Some(p) = self.tenants.read().await.get(tenant).cloned() {
            return Ok(p);
        }

        let path = self.config.tenant_db_path(tenant);
        if !path.exists() {
            return Err(TenantError::NotFound(tenant.to_string()));
        }

        let mut guard = self.tenants.write().await;
        if let Some(p) = guard.get(tenant).cloned() {
            return Ok(p);
        }

        let url = self.config.tenant_db_url(tenant);
        let pool = database_config::connect_tenant(&url).await?;
        database_config::migrate_tenant(&pool)
            .await
            .map_err(TenantError::from)?;
        guard.insert(tenant.to_string(), pool.clone());
        Ok(pool)
    }

    pub async fn active_pools(&self) -> Vec<(TenantId, SqlitePool)> {
        self.tenants
            .read()
            .await
            .iter()
            .map(|(id, p)| (id.clone(), p.clone()))
            .collect()
    }

    pub async fn provision(&self, tenant: TenantId) -> RepoResult<()> {
        if self.tenants.read().await.contains_key(&tenant) {
            return Ok(());
        }
        let url = self.config.tenant_db_url(&tenant);
        let pool = database_config::connect_tenant(&url).await?;
        database_config::migrate_tenant(&pool).await?;
        let mut guard = self.tenants.write().await;
        guard.entry(tenant).or_insert(pool);
        Ok(())
    }

    pub async fn evict(&self, tenant: &str) {
        if let Some(p) = self.tenants.write().await.remove(tenant) {
            p.close().await;
        }
    }
}
