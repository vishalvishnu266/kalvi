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
use crate::tenancy::{TenantError, TenantId};
use crate::Config;

/// Global application state.
///
/// Holds only the *pool handles* for the three physical databases:
///
/// * `system`   — the master database (`system.db`), stored as a raw
///   `SqlitePool`. All tenant-registry / portal-user reads and writes
///   go through [`crate::services::system`] free functions.
/// * `sessions` — the shared session store database (`sessions.db`),
///   also stored as a raw `SqlitePool`. Session SQL lives as free
///   functions in [`crate::session`].
/// * `tenants`  — a map of tenant `SqlitePool` handles, populated on
///   first use (`provision` / `pool_for`) and evicted on disable.
///
/// **No services are cached here.** Handlers receive a per-request
/// [`crate::http::TenantScope`] and call free-fn services directly
/// against `scope.pool`, e.g. `services::demo::create_message(&scope.pool, &scope.ctx, body)`.
#[derive(Clone)]
pub struct AppState {
    pub system: SqlitePool,
    pub sessions: SqlitePool,
    pub config: Config,
    pub tenants: Arc<RwLock<HashMap<String, SqlitePool>>>,
}

impl AppState {
    pub fn new(system: SqlitePool, sessions: SqlitePool, config: Config) -> Self {
        Self {
            system,
            sessions,
            config,
            tenants: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

pub use routes::{build_router, ServiceHttpError};
