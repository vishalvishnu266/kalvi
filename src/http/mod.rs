pub mod admin;
pub mod api_routes;
pub mod portal;
pub mod routes;
pub mod web;

use sqlx::SqlitePool;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::db;
use crate::session::SessionStore;
use crate::tenancy::{TenantError, TenantId};
use crate::Config;

/// Global application state.
///
/// Holds only the *pool handles* for the three physical databases:
///
/// * `system`   — the master database (`system.db`), stored as a raw
///   `SqlitePool`. All tenant-registry / portal-user reads and writes
///   go through [`crate::services::system`] free functions.
/// * `sessions` — the shared session store database (`sessions.db`).
/// * `tenants`  — a map of tenant `SqlitePool` handles, populated on
///   first use (`provision` / `pool_for`) and evicted on disable.
///
/// **No services are cached here.** Handlers receive a per-request
/// [`crate::http::TenantScope`] and call free-fn services directly
/// against `scope.pool`, e.g. `services::people::hire_staff(&scope.pool, &scope.ctx, body)`.
#[derive(Clone)]
pub struct AppState {
    pub system: SqlitePool,
    pub sessions: SessionStore,
    pub config: Config,
    pub tenants: Arc<RwLock<HashMap<TenantId, SqlitePool>>>,
}

impl AppState {
    pub fn new(system: SqlitePool, sessions: SessionStore, config: Config) -> Self {
        Self {
            system,
            sessions,
            config,
            tenants: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Resolve (or lazily open + migrate) the tenant pool. Cheap on the
    /// fast path — just a read-lock lookup + clone of the underlying
    /// `SqlitePool` (which itself is `Arc`-based).
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
        let pool = db::connect(&url).await?;
        db::migrate(&pool).await.map_err(TenantError::from)?;
        guard.insert(tenant.to_string(), pool.clone());
        Ok(pool)
    }

    /// Snapshot of currently cached tenant pools (used at shutdown).
    pub async fn active_tenant_pools(&self) -> Vec<(TenantId, SqlitePool)> {
        self.tenants
            .read()
            .await
            .iter()
            .map(|(id, p)| (id.clone(), p.clone()))
            .collect()
    }

    /// Create the tenant database, run migrations, and cache the pool.
    pub async fn provision(&self, tenant: TenantId) -> crate::error::RepoResult<()> {
        if self.tenants.read().await.contains_key(&tenant) {
            return Ok(());
        }
        let url = self.config.tenant_db_url(&tenant);
        let pool = db::connect(&url).await?;
        db::migrate(&pool).await?;
        let mut guard = self.tenants.write().await;
        guard.entry(tenant).or_insert(pool);
        Ok(())
    }

    /// Drop the cached pool for a tenant (called when a tenant is
    /// disabled or deleted). The physical database file is left intact.
    pub async fn evict(&self, tenant: &str) {
        if let Some(p) = self.tenants.write().await.remove(tenant) {
            p.close().await;
        }
    }
}

pub use routes::{build_router, ServiceHttpError, TenantScope};
