use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub fn generate_uuid() -> String {
    Uuid::new_v4().to_string()
}

pub fn current_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as i64
}
