//! Application-wide state, cloned into every handler by axum.

use super::db::TenantDatabaseManager;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db_manager: Arc<TenantDatabaseManager>,
}
