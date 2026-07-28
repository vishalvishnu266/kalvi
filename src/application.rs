//! Application-wide state (Spring Boot's `ApplicationContext`
//! equivalent) — a small aggregate over the pool handles + config
//! + tenant pool manager shared by every request handler.

use sqlx::SqlitePool;

use crate::config::app_config::AppConfig;
use crate::exception::repo_error::RepoResult;
use crate::exception::tenant_error::TenantError;
use crate::service::tenant_pool_service::TenantPoolService;
use crate::tenant::tenant_id::TenantId;

#[derive(Clone)]
pub struct AppState {
    pub system: SqlitePool,
    pub sessions: SqlitePool,
    pub config: AppConfig,
    pub tenant_pools: TenantPoolService,
}

impl AppState {
    pub fn new(system: SqlitePool, sessions: SqlitePool, config: AppConfig) -> Self {
        let tenant_pools = TenantPoolService::new(config.clone());
        Self {
            system,
            sessions,
            config,
            tenant_pools,
        }
    }

    pub async fn pool_for(&self, tenant: &str) -> Result<SqlitePool, TenantError> {
        self.tenant_pools.pool_for(tenant).await
    }

    pub async fn active_tenant_pools(&self) -> Vec<(TenantId, SqlitePool)> {
        self.tenant_pools.active_pools().await
    }

    pub async fn provision(&self, tenant: TenantId) -> RepoResult<()> {
        self.tenant_pools.provision(tenant).await
    }

    pub async fn evict(&self, tenant: &str) {
        self.tenant_pools.evict(tenant).await
    }
}
