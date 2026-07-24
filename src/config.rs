use std::time::Duration;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Config {
    pub db_dir: PathBuf,
    pub bind_addr: String,
    pub shutdown_timeout: Duration,
}

impl Config {
    pub fn from_env() -> Self {
        let db_dir = PathBuf::from(
            std::env::var("DB_DIR").unwrap_or_else(|_| "data".to_string())
        );
        let bind_addr = std::env::var("BIND")
            .unwrap_or_else(|_| "0.0.0.0:3000".to_string());
        let shutdown_timeout = std::env::var("SHUTDOWN_TIMEOUT_S")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .map(Duration::from_secs)
            .unwrap_or(Duration::from_secs(30));
        Self {
            db_dir,
            bind_addr,
            shutdown_timeout,
        }
    }

    pub fn system_db_url(&self) -> String {
        format!("sqlite://{}/system.db?mode=rwc", self.db_dir.display())
    }

    pub fn session_db_url(&self) -> String {
        format!("sqlite://{}/sessions.db?mode=rwc", self.db_dir.display())
    }

    pub fn tenant_db_root(&self) -> PathBuf {
        self.db_dir.join("tenants")
    }

    pub fn tenant_db_path(&self, tenant_id: &crate::tenancy::TenantId) -> PathBuf {
        self.tenant_db_root().join(format!("{}.db", tenant_id.as_str()))
    }

    pub fn tenant_db_url(&self, tenant_id: &crate::tenancy::TenantId) -> String {
        format!("sqlite://{}", self.tenant_db_path(tenant_id).display())
    }
}
