use std::time::{SystemTime, UNIX_EPOCH};

pub fn current_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as i64
}

pub fn generate_random_id(prefix: &str) -> String {
    let ts = current_timestamp();
    // Simple pseudo-random using timestamp and a few bits of entropy from a pointer address
    // In a real production app, you'd use getrandom, but since you want ZERO magical libraries
    // this is a functional alternative for non-cryptographic IDs.
    let addr = &ts as *const i64 as usize;
    format!("{}_{:x}_{:x}", prefix, ts, addr)
}
