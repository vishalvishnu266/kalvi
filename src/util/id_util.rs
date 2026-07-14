use uuid::Uuid;

pub fn generate_uuid() -> String {
    Uuid::new_v4().to_string()
}

pub fn generate_prefixed_id(prefix: &str) -> String {
    format!("{}_{}", prefix, Uuid::new_v4())
}

pub fn current_timestamp() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as i64
}
