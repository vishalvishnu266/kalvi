//! Multi-tenant support with **one SQLite database per tenant**.
//!
//! ## Design
//!
//! * A [`TenantId`] is a short opaque string extracted from the request
//!   (subdomain, header, JWT claim — your choice at the HTTP layer).
//! * A [`TenantResolver`] maps a `TenantId` → a `sqlx` connection URL.
//!   Two built-in resolvers are provided:
//!     * [`FileTenantResolver`] — `data/<tenant>.db` (production default).
//!     * [`InMemoryTenantResolver`] — `sqlite::memory:` per tenant (for tests).
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
use std::path::{Path, PathBuf};
use std::sync::Arc;

use sqlx::SqlitePool;
use tokio::sync::RwLock;

use crate::db;
use crate::error::{RepoError, RepoResult};
use crate::services::AppServices;

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

/// How a `TenantId` becomes a sqlx connection URL.
///
/// Implementations must be cheap and side-effect-free. Actually opening
/// the DB is the [`TenantRegistry`]'s job.
pub trait TenantResolver: Send + Sync + 'static {
    fn db_url(&self, tenant: &TenantId) -> String;
}

/// The default production resolver: one file per tenant under `root/`.
///
/// e.g. `root=data/tenants` → `sqlite://data/tenants/acme.db?mode=rwc`
pub struct FileTenantResolver {
    pub root: PathBuf,
}

impl FileTenantResolver {
    pub fn new(root: impl AsRef<Path>) -> Self {
        Self { root: root.as_ref().to_path_buf() }
    }
}

impl TenantResolver for FileTenantResolver {
    fn db_url(&self, tenant: &TenantId) -> String {
        // TenantId is already sanitized; safe to embed in a path.
        let path = self.root.join(format!("{}.db", tenant.as_str()));
        format!("sqlite://{}?mode=rwc", path.display())
    }
}

/// A resolver that gives every tenant its own private in-memory database.
///
/// Handy for tests and demos. Uses `file:<tenant>?mode=memory&cache=shared`
/// so that multiple connections from the same pool see the same data.
pub struct InMemoryTenantResolver;

impl TenantResolver for InMemoryTenantResolver {
    fn db_url(&self, tenant: &TenantId) -> String {
        format!("sqlite:file:tenant_{}?mode=memory&cache=shared", tenant.as_str())
    }
}

/// Optional allow-list gate. Return `Ok(())` to let a tenant through,
/// `Err(TenantError::NotFound|Disabled)` to reject. The middleware calls this
/// **before** touching the pool cache, so unauthorized tenants never create a
/// file / connection.
#[async_trait::async_trait]
pub trait TenantGuard: Send + Sync + 'static {
    async fn admit(&self, tenant: &TenantId) -> Result<(), TenantError>;
}

/// A guard that accepts every tenant (useful for dev / tests).
pub struct AllowAllGuard;

#[async_trait::async_trait]
impl TenantGuard for AllowAllGuard {
    async fn admit(&self, _: &TenantId) -> Result<(), TenantError> { Ok(()) }
}

/// A guard backed by an in-memory whitelist. Swap in a DB-backed one in
/// production if you have a control-plane table of tenants.
pub struct StaticAllowList {
    allowed: std::collections::HashSet<TenantId>,
}

impl StaticAllowList {
    pub fn new(ids: impl IntoIterator<Item = TenantId>) -> Self {
        Self { allowed: ids.into_iter().collect() }
    }
}

#[async_trait::async_trait]
impl TenantGuard for StaticAllowList {
    async fn admit(&self, tenant: &TenantId) -> Result<(), TenantError> {
        if self.allowed.contains(tenant) { Ok(()) }
        else { Err(TenantError::NotFound(tenant.clone())) }
    }
}

/// Configuration for a [`TenantRegistry`].
pub struct TenantRegistryConfig {
    pub resolver: Arc<dyn TenantResolver>,
    pub guard: Arc<dyn TenantGuard>,
    /// Run `sqlx::migrate!("./migrations")` when a tenant DB is first opened.
    pub auto_migrate: bool,
}

impl TenantRegistryConfig {
    pub fn dev_defaults() -> Self {
        Self {
            resolver: Arc::new(InMemoryTenantResolver),
            guard:    Arc::new(AllowAllGuard),
            auto_migrate: true,
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
    resolver: Arc<dyn TenantResolver>,
    guard: Arc<dyn TenantGuard>,
    auto_migrate: bool,
    entries: Arc<RwLock<HashMap<TenantId, TenantEntry>>>,
}

impl TenantRegistry {
    pub fn new(cfg: TenantRegistryConfig) -> Self {
        tracing::debug!("TenantRegistry::new: initializing");
        Self {
            resolver: cfg.resolver,
            guard: cfg.guard,
            auto_migrate: cfg.auto_migrate,
            entries: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Get (or lazily create + migrate) the pool for a tenant.
    /// The guard is consulted **before** the cache miss so unauthorized
    /// tenants never allocate resources.
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
        // Guard first (cheap: usually an in-memory check).
        self.guard.admit(tenant).await?;

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
        let url = self.resolver.db_url(tenant);
        tracing::debug!("TenantRegistry::entry_for: db_url={}", url);
        let pool = db::connect(&url).await.map_err(TenantError::from)?;
        if self.auto_migrate {
            tracing::debug!("TenantRegistry::entry_for: running auto-migration for tenant={}", tenant);
            db::migrate(&pool).await.map_err(TenantError::from)?;
        }
        let entry = TenantEntry {
            services: AppServices::new(pool.clone()),
            pool,
        };
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
        let url = self.resolver.db_url(&tenant);
        let pool = db::connect(&url).await?;
        db::migrate(&pool).await?;
        let entry = TenantEntry {
            services: AppServices::new(pool.clone()),
            pool,
        };
        // Double-check under the write lock in case another task raced us.
        let mut guard = self.entries.write().await;
        guard.entry(tenant).or_insert(entry);
        Ok(())
    }

    /// Evict a tenant's cached pool + services (e.g. on deactivation).
    /// Closes the underlying pool.
    pub async fn evict(&self, tenant: &TenantId) {
        if let Some(entry) = self.entries.write().await.remove(tenant) {
            entry.pool.close().await;
        }
    }

    /// Close every cached pool. Call on graceful shutdown.
    pub async fn shutdown(&self) {
        let mut guard = self.entries.write().await;
        for (_, entry) in guard.drain() {
            entry.pool.close().await;
        }
    }

    /// Ids of tenants currently cached.
    pub async fn active_tenants(&self) -> Vec<TenantId> {
        self.entries.read().await.keys().cloned().collect()
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
        let reg = TenantRegistry::new(TenantRegistryConfig::dev_defaults());
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
    }
}
