use crate::config::DatabaseConfig;
use crate::middleware::rate_limit_middleware::RateLimiter;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConfig,
    pub limiter: Arc<RateLimiter>,
}
