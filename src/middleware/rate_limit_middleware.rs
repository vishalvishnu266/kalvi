use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;
use std::time::{Duration, Instant};

struct RateInfo {
    count: u32,
    last_reset: Instant,
}

pub struct RateLimiter {
    clients: Mutex<HashMap<String, RateInfo>>,
    max_requests: u32,
    window: Duration,
}

impl RateLimiter {
    pub fn new(max_requests: u32, window: Duration) -> Arc<Self> {
        Arc::new(Self {
            clients: Mutex::new(HashMap::new()),
            max_requests,
            window,
        })
    }

    pub async fn check(&self, ip: String) -> bool {
        let mut clients = self.clients.lock().await;
        let info = clients.entry(ip).or_insert(RateInfo {
            count: 0,
            last_reset: Instant::now(),
        });

        if info.last_reset.elapsed() > self.window {
            info.count = 1;
            info.last_reset = Instant::now();
            return true;
        }

        if info.count < self.max_requests {
            info.count += 1;
            return true;
        }

        false
    }
}

pub async fn rate_limit_middleware(
    axum::extract::State(state): axum::extract::State<crate::config::AppState>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, crate::util::AppError> {
    let ip = req.headers()
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.split(',').next().unwrap_or(s).trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    if state.limiter.check(ip).await {
        Ok(next.run(req).await)
    } else {
        Err(crate::util::AppError::TooManyRequests(format!("IP: {}", ip)))
    }
}
