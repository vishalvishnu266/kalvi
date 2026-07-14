use sqlx::SqlitePool;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    pub master_db: SqlitePool,
    pub tenant_pools: Arc<RwLock<HashMap<String, SqlitePool>>>,
}
