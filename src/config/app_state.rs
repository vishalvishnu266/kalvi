use crate::config::database_config::DatabaseConfig;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConfig,
}

impl AppState {
    pub async fn new() -> Result<Self, sqlx::Error> {
        Ok(Self {
            db: DatabaseConfig::new().await?,
        })
    }
}
