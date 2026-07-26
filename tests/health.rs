//! Health probe surface — no auth, no tenant, no state.
//!
//! Run with: `cargo test --test health`

mod common;

use common::{assert_status, base_url, client, json_body, require_server};

#[test]
fn health_endpoint_returns_ok() {
    let c = client();
    if !require_server(&c) {
        return;
    }
    let r = c
        .get(format!("{}/api/health", base_url()))
        .send()
        .expect("GET /api/health");
    let r = assert_status(r, 200, "GET /api/health");
    assert_eq!(r.text().unwrap_or_default().trim(), "ok");
}

#[test]
fn live_probe_returns_alive() {
    let c = client();
    if !require_server(&c) {
        return;
    }
    let r = c
        .get(format!("{}/api/live", base_url()))
        .send()
        .expect("GET /api/live");
    let body = json_body(assert_status(r, 200, "GET /api/live"), "GET /api/live");
    assert_eq!(body["status"], "alive");
}

#[test]
fn ready_probe_returns_ready_or_draining() {
    let c = client();
    if !require_server(&c) {
        return;
    }
    let r = c
        .get(format!("{}/api/ready", base_url()))
        .send()
        .expect("GET /api/ready");
    let sc = r.status().as_u16();
    assert!(
        sc == 200 || sc == 503,
        "GET /api/ready expected 200 or 503, got {sc}"
    );
}
