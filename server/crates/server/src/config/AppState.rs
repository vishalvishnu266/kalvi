use super::DatabaseManager::TenantDatabaseManager;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db_manager: Arc<TenantDatabaseManager>,
}
