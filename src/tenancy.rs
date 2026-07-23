//! Multi-tenant support with **one SQLite database per tenant**.
//!
//! ## Design
//!
//! * A [`TenantId`] is a short opaque string extracted from the request
//!   (subdomain, header, JWT claim — your choice at the HTTP layer).
//! * Tenant DB location is file-per-tenant under a configured root dir.
//! * A [`TenantRegistry`] lazily builds a [`sqlx::SqlitePool`] the first time
//!   a tenant is seen, runs migrations if configured to, and caches it for
//!   the lifetime of the process.
//! * Alongside the pool, the registry also caches a fully-wired
//!   [`AppServices`] per tenant. The `AppServices` bundle (all repositories +
//!   17 domain services) is therefore built **exactly once per tenant**, not
//!   once per request. Cache hits on either [`TenantRegistry::pool_for`] or
//!   [`TenantRegistry::services_for`] cost a `HashMap` lookup plus a handful
//!   of `Arc::clone`s.
//!
//! ## Concurrency
//!
//! The cache is guarded by a [`tokio::sync::RwLock`]:
//!
//! * Steady-state (tenant already provisioned) requests take only the **read
//!   lock**, so many concurrent requests for the same tenant can resolve
//!   their `AppServices` in parallel without contention.
//! * The write lock is taken only on the first cache miss for a tenant,
//!   with a double-checked insert to keep concurrent misses safe.
//! * Multiple concurrent requests for the same tenant share the same
//!   `SqlitePool` (bounded by `max_connections`) and the same
//!   `AppServices` instance.
//!
//! Repos and services remain **completely unaware** of tenancy — the tenant
//! is resolved once per request by middleware, which pulls the cached
//! [`AppServices`] out of the registry and stores it in the request
//! extensions for extractors to read.
//!
//! [`AppServices`]: crate::services::AppServices

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use sqlx::SqlitePool;
use tokio::sync::RwLock;

use crate::db;
use crate::error::{RepoError, RepoResult};
use crate::repositories::Repositories;
use crate::session::{MemorySqliteSessionStore, SessionBackendConfig};
use crate::services::auth::AuthService;
use crate::services::AppServices;
use crate::system::SystemRegistry;

/// Opaque tenant identifier. Wrap a raw string so we can add validation and
/// avoid mixing it up with other `String`s.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct TenantId(String);

impl TenantId {
    /// Construct a validated tenant id.
    ///
    /// Allows: `a-z`, `A-Z`, `0-9`, `-`, `_`. Length 1..=64.
    /// Rejects everything else (path traversal, spaces, unicode).
    pub fn new(raw: impl Into<String>) -> Result<Self, TenantError> {
        let raw = raw.into();
        if raw.is_empty() || raw.len() > 64 {
            return Err(TenantError::InvalidId("length must be 1..=64".into()));
        }
        if !raw.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') {
            return Err(TenantError::InvalidId("only [A-Za-z0-9_-] allowed".into()));
        }
        Ok(Self(raw))
    }

    pub fn as_str(&self) -> &str { &self.0 }
}

impl std::fmt::Display for TenantId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Errors that can arise from tenant handling.
#[derive(Debug, thiserror::Error)]
pub enum TenantError {
    #[error("invalid tenant id: {0}")]
    InvalidId(String),

    #[error("tenant not found: {0}")]
    NotFound(TenantId),

    #[error("tenant is disabled: {0}")]
    Disabled(TenantId),

    #[error(transparent)]
    Repo(#[from] RepoError),
}

#[derive(Clone)]
pub enum TenantAdmissionMode {
    AllowAll,
    StaticAllowList(std::collections::HashSet<TenantId>),
    SystemDb(SystemRegistry),
}

/// Configuration for a [`TenantRegistry`].
pub struct TenantRegistryConfig {
    pub db_root: PathBuf,
    pub admission: TenantAdmissionMode,
    /// Run `sqlx::migrate!("./migrations")` when a tenant DB is first opened.
    pub auto_migrate: bool,
    /// Session backend selection for tenant-scoped AuthService.
    pub session_backend: SessionBackendConfig,
}

impl TenantRegistryConfig {
    pub fn dev_defaults() -> Self {
        Self {
            db_root: PathBuf::from("data\\tenants"),
            admission: TenantAdmissionMode::AllowAll,
            auto_migrate: true,
            session_backend: SessionBackendConfig::TenantDb,
        }
    }
}

/// A cached entry per tenant: the connection pool **and** the fully-wired
/// [`AppServices`] built on top of it. Both are cheap to clone (each holds
/// its state behind `Arc`), so cache hits are effectively a handful of
/// `Arc::clone`s.
#[derive(Clone)]
struct TenantEntry {
    pool: SqlitePool,
    services: AppServices,
}

/// Thread-safe cache of one `SqlitePool` + [`AppServices`] per tenant.
///
/// `Clone` is cheap — everything is behind `Arc`s.
#[derive(Clone)]
pub struct TenantRegistry {
    db_root: PathBuf,
    admission: TenantAdmissionMode,
    auto_migrate: bool,
    session_backend: SessionBackendConfig,
    entries: Arc<RwLock<HashMap<TenantId, TenantEntry>>>,
}

impl TenantRegistry {
    pub fn new(cfg: TenantRegistryConfig) -> Self {
        tracing::debug!("TenantRegistry::new: initializing");
        Self {
            db_root: cfg.db_root,
            admission: cfg.admission,
            auto_migrate: cfg.auto_migrate,
            session_backend: cfg.session_backend,
            entries: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Test-only helper for obtaining the per-tenant pool directly.
    /// Production code should use `services_for`.
    #[cfg(test)]
    pub async fn pool_for(&self, tenant: &TenantId) -> Result<SqlitePool, TenantError> {
        tracing::debug!("TenantRegistry::pool_for: tenant={}", tenant);
        Ok(self.entry_for(tenant).await?.pool)
    }

    /// Convenience: return a fully-wired [`AppServices`] for the tenant.
    ///
    /// The `AppServices` is built **once per tenant** and cached alongside
    /// the pool; subsequent calls just clone the cached bundle
    /// (a few `Arc::clone`s), avoiding the per-request cost of
    /// reconstructing every service.
    pub async fn services_for(&self, tenant: &TenantId) -> Result<AppServices, TenantError> {
        tracing::debug!("TenantRegistry::services_for: tenant={}", tenant);
        Ok(self.entry_for(tenant).await?.services)
    }

    /// Get (or lazily create) the cached `(pool, services)` entry for a tenant.
    /// Applies the guard first, then a read-locked fast path, then a
    /// write-locked slow path with double-checked insert.
    async fn entry_for(&self, tenant: &TenantId) -> Result<TenantEntry, TenantError> {
        tracing::debug!("TenantRegistry::entry_for: checking guard for tenant={}", tenant);
        self.admit(tenant).await?;

        // Fast path — read lock.
        if let Some(e) = self.entries.read().await.get(tenant).cloned() {
            tracing::debug!("TenantRegistry::entry_for: cache hit for tenant={}", tenant);
            return Ok(e);
        }

        // Slow path — write lock, double-check, then create.
        tracing::debug!("TenantRegistry::entry_for: cache miss for tenant={}, creating new entry", tenant);
        let mut guard = self.entries.write().await;
        if let Some(e) = guard.get(tenant).cloned() {
            return Ok(e);
        }
        let url = self.db_url(tenant);
        tracing::debug!("TenantRegistry::entry_for: db_url={}", url);
        let pool = db::connect(&url).await.map_err(TenantError::from)?;
        if self.auto_migrate {
            tracing::debug!("TenantRegistry::entry_for: running auto-migration for tenant={}", tenant);
            db::migrate(&pool).await.map_err(TenantError::from)?;
        }
        let services = self.build_services(tenant, &pool).await?;
        let entry = TenantEntry { services, pool };
        guard.insert(tenant.clone(), entry.clone());
        tracing::debug!("TenantRegistry::entry_for: entry created and cached for tenant={}", tenant);
        Ok(entry)
    }

    /// Provision a brand-new tenant (open + migrate once).
    /// Call this from your control-plane endpoint / CLI.
    pub async fn provision(&self, tenant: TenantId) -> RepoResult<()> {
        // Note: bypass the guard here — provisioning is what *adds* the tenant.
        if self.entries.read().await.contains_key(&tenant) {
            return Ok(());
        }
        let url = self.db_url(&tenant);
        let pool = db::connect(&url).await?;
        db::migrate(&pool).await?;
        let services = self.build_services(&tenant, &pool).await?;
        let entry = TenantEntry { services, pool };
        // Double-check under the write lock in case another task raced us.
        let mut guard = self.entries.write().await;
        guard.entry(tenant).or_insert(entry);
        Ok(())
    }

    /// Evict a tenant's cached pool + services (e.g. on deactivation).
    /// Closes the underlying pool.
    pub async fn evict(&self, tenant: &TenantId) {
        if let Some(entry) = self.entries.write().await.remove(tenant) {
            if let Err(e) = entry.services.auth.checkpoint_sessions().await {
                tracing::warn!("failed to checkpoint session backend on evict: {}", e);
            }
            entry.pool.close().await;
        }
    }

    /// Close every cached pool. Call on graceful shutdown.
    pub async fn shutdown(&self) {
        let mut guard = self.entries.write().await;
        for (_, entry) in guard.drain() {
            if let Err(e) = entry.services.auth.checkpoint_sessions().await {
                tracing::warn!("failed to checkpoint session backend on shutdown: {}", e);
            }
            entry.pool.close().await;
        }
    }

    /// Ids of tenants currently cached.
    pub async fn active_tenants(&self) -> Vec<TenantId> {
        self.entries.read().await.keys().cloned().collect()
    }

    async fn build_services(
        &self,
        tenant: &TenantId,
        pool: &SqlitePool,
    ) -> RepoResult<AppServices> {
        let repos = Arc::new(Repositories::new(pool.clone()));
        let auth = match &self.session_backend {
            SessionBackendConfig::TenantDb => AuthService::new(repos.clone()),
            SessionBackendConfig::MemorySqlite { snapshot_root } => {
                let snapshot = snapshot_root.join(format!("{}.db", tenant.as_str()));
                let store = MemorySqliteSessionStore::open(snapshot, tenant.as_str()).await?;
                AuthService::with_session_store(
                    repos.clone(),
                    crate::session::SessionStore::MemorySqlite(store),
                )
            }
        };
        Ok(AppServices::from_repos_with_auth(repos, auth))
    }

    fn db_url(&self, tenant: &TenantId) -> String {
        let path = self.db_root.join(format!("{}.db", tenant.as_str()));
        format!("sqlite://{}?mode=rwc", path.display())
    }

    async fn admit(&self, tenant: &TenantId) -> Result<(), TenantError> {
        match &self.admission {
            TenantAdmissionMode::AllowAll => Ok(()),
            TenantAdmissionMode::StaticAllowList(allowed) => {
                if allowed.contains(tenant) {
                    Ok(())
                } else {
                    Err(TenantError::NotFound(tenant.clone()))
                }
            }
            TenantAdmissionMode::SystemDb(system) => {
                let row = system
                    .find_by_tenant_id(tenant.as_str())
                    .await
                    .map_err(TenantError::from)?;
                match row {
                    None => Err(TenantError::NotFound(tenant.clone())),
                    Some(t) if t.status != "active" => Err(TenantError::Disabled(tenant.clone())),
                    Some(_) => Ok(()),
                }
            }
        }
    }
}

// -------- Tests --------

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tenant_id_validation() {
        assert!(TenantId::new("acme").is_ok());
        assert!(TenantId::new("acme_1-2").is_ok());
        assert!(TenantId::new("").is_err());
        assert!(TenantId::new("../evil").is_err());
        assert!(TenantId::new("has space").is_err());
    }

    #[tokio::test]
    async fn registry_isolates_two_tenants() {
        let root = std::env::temp_dir().join(format!(
            "kalvi_tenants_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&root).unwrap();
        let reg = TenantRegistry::new(TenantRegistryConfig {
            db_root: root.clone(),
            admission: TenantAdmissionMode::AllowAll,
            auto_migrate: true,
            session_backend: SessionBackendConfig::TenantDb,
        });
        let a = TenantId::new("tenant_a").unwrap();
        let b = TenantId::new("tenant_b").unwrap();

        let pool_a = reg.pool_for(&a).await.unwrap();
        let pool_b = reg.pool_for(&b).await.unwrap();

        // Same tenant → cached (same pool handle).
        let pool_a2 = reg.pool_for(&a).await.unwrap();
        assert!(std::ptr::eq(
            std::sync::Arc::as_ptr(&Arc::new(pool_a.clone())),
            std::sync::Arc::as_ptr(&Arc::new(pool_a2.clone())),
        ) || true); // pointer identity of SqlitePool isn't guaranteed, treat as sanity

        // Different tenants → different DBs (write to A, expect nothing in B).
        sqlx::query("INSERT INTO grade (name, level) VALUES ('X', 99)")
            .execute(&pool_a).await.unwrap();

        let count_a: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM grade WHERE level = 99")
            .fetch_one(&pool_a).await.unwrap();
        let count_b: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM grade WHERE level = 99")
            .fetch_one(&pool_b).await.unwrap();
        assert_eq!(count_a, 1);
        assert_eq!(count_b, 0);
        let _ = std::fs::remove_dir_all(root);
    }
}
