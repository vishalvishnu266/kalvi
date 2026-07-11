use crate::config::DatabaseConfig::DatabaseConfig;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConfig,
}
