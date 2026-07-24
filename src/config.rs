use std::time::Duration;
use std::path::PathBuf;

pub struct Config {
    pub system_db_url: String,
    pub tenant_db_root: PathBuf,
    pub session_db_url: String,
    pub bind_addr: String,
    pub shutdown_timeout: Duration,
}

impl Config {
    pub fn from_env() -> Self {
        let system_db_url = std::env::var("SYSTEM_DB_URL")
            .unwrap_or_else(|_| "sqlite://data/system.db?mode=rwc".to_string());
        
        let tenant_db_root = PathBuf::from(
            std::env::var("TENANT_DB_ROOT")
                .unwrap_or_else(|_| "data/tenants".to_string())
        );

        let session_db_url = std::env::var("SESSION_DB_URL")
            .unwrap_or_else(|_| "sqlite://data/sessions.db?mode=rwc".to_string());

        let bind_addr = std::env::var("BIND")
            .unwrap_or_else(|_| "0.0.0.0:3000".to_string());

        let shutdown_timeout = std::env::var("SHUTDOWN_TIMEOUT_S")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .map(Duration::from_secs)
            .unwrap_or(Duration::from_secs(30));

        Self {
            system_db_url,
            tenant_db_root,
            session_db_url,
            bind_addr,
            shutdown_timeout,
        }
    }
}
