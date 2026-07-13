use crate::config::DatabaseConfig;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConfig,
}
