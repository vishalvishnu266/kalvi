pub mod api_routes;
pub mod routes;
pub mod admin;
pub mod web;
pub mod portal;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use sqlx::SqlitePool;

use crate::session::SessionStore;
use crate::tenancy::{TenantId, TenantError, build_tenant_services};
use crate::services::AppServices;
use crate::db;
use crate::system::SystemRegistry;

#[derive(Clone)]
struct TenantEntry {
    pool: SqlitePool,
    services: AppServices,
}

use crate::Config;

#[derive(Clone)]
pub struct AppState {
    pub system: SystemRegistry,
    pub sessions: SessionStore,
    pub config: Config,
    pub tenants: Arc<RwLock<HashMap<TenantId, TenantEntry>>>,
}

impl AppState {
    pub fn new(system: SystemRegistry, sessions: SessionStore, config: Config) -> Self {
        Self {
            system,
            sessions,
            config,
            tenants: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn services_for(&self, tenant: &TenantId) -> Result<AppServices, TenantError> {
        if let Some(entry) = self.tenants.read().await.get(tenant).cloned() {
            return Ok(entry.services);
        }

        let path = self.config.tenant_db_path(tenant);
        if !path.exists() {
            return Err(TenantError::NotFound(tenant.clone()));
        }

        let mut guard = self.tenants.write().await;

        if let Some(entry) = guard.get(tenant).cloned() {
            return Ok(entry.services);
        }

        let url = self.config.tenant_db_url(&tenant);
        let pool = db::connect(&url).await?;

db::migrate(&pool).await.map_err(TenantError::from)?;

        let services = build_tenant_services(&pool, self.sessions.clone()).await.map_err(TenantError::from)?;
        let entry = TenantEntry { pool, services: services.clone() };

        guard.insert(tenant.clone(), entry);
        Ok(services)
    }

    pub async fn active_tenant_pools(&self) -> Vec<(TenantId, SqlitePool)> {
        self.tenants.read().await
            .iter()
            .map(|(id, entry)| (id.clone(), entry.pool.clone()))
            .collect()
    }

    pub async fn provision(&self, tenant: TenantId) -> crate::error::RepoResult<()> {
        if self.tenants.read().await.contains_key(&tenant) {
            return Ok(());
        }
        let url = self.config.tenant_db_url(&tenant);
        let pool = db::connect(&url).await?;
        db::migrate(&pool).await?;
        let services = build_tenant_services(&pool, self.sessions.clone()).await?;
        let entry = TenantEntry { pool, services };
        let mut guard = self.tenants.write().await;
        guard.entry(tenant).or_insert(entry);
        Ok(())
    }

    pub async fn evict(&self, tenant: &TenantId) {
        if let Some(entry) = self.tenants.write().await.remove(tenant) {
            entry.pool.close().await;
        }
    }
}

pub use routes::{build_router, ServiceHttpError, TenantScope};
