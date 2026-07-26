//! Demo module surface — the reference wiring example.
//!
//! Run with: `cargo test --test demo`

mod common;

use common::{assert_status, json_body, ms_stamp, Fixture};
use serde_json::json;

#[test]
fn ping_is_public_and_echoes_tenant() {
    let fx = Fixture::new("demo");
    let r = fx
        .client
        .get(fx.api_url("/demo/ping"))
        .send()
        .expect("GET /demo/ping");
    let body = json_body(assert_status(r, 200, "demo/ping"), "demo/ping");
    assert_eq!(body["status"], "ok");
    assert_eq!(body["tenant"], fx.tenant);
    assert!(
        body["request_id"].as_str().is_some(),
        "ping should include a request_id"
    );
}

#[test]
fn echo_persists_and_lists_authenticated() {
    let mut fx = Fixture::new("demo");
    fx.login();

    let msg = format!("hello from demo test @ {}", ms_stamp());

    // Write.
    let r = fx
        .client
        .post(fx.api_url("/demo/echo"))
        .json(&json!({ "text": msg }))
        .send()
        .expect("POST /demo/echo");
    let echoed = json_body(assert_status(r, 200, "demo/echo"), "demo/echo");
    assert_eq!(echoed["text"], msg);
    assert!(echoed["id"].as_i64().is_some());

    // Read.
    let r = fx
        .client
        .get(fx.api_url("/demo/messages"))
        .send()
        .expect("GET /demo/messages");
    let list = json_body(assert_status(r, 200, "demo/messages"), "demo/messages");
    let arr = list.as_array().expect("demo/messages is an array");
    assert!(!arr.is_empty(), "list should include our row");
    assert_eq!(arr[0]["text"], msg, "our row should be the most recent");
}

#[test]
fn echo_rejects_empty_text() {
    let mut fx = Fixture::new("demo");
    fx.login();

    let r = fx
        .client
        .post(fx.api_url("/demo/echo"))
        .json(&json!({ "text": "   " }))
        .send()
        .expect("POST /demo/echo (empty)");
    let sc = r.status().as_u16();
    assert_eq!(
        sc, 400,
        "empty text should trip service validation (want 400, got {sc})"
    );
}

#[test]
fn echo_forbidden_without_session() {
    // Fixture registers the admin user but we deliberately skip login.
    let fx = Fixture::new("demo");

    let r = fx
        .client
        .post(fx.api_url("/demo/echo"))
        .json(&json!({ "text": "should be blocked" }))
        .send()
        .expect("POST /demo/echo (anon)");
    let sc = r.status().as_u16();
    assert_eq!(
        sc, 403,
        "unauthenticated POST /demo/echo should be forbidden (want 403, got {sc})"
    );
}

#[test]
fn web_dashboard_shows_demo_tile() {
    let mut fx = Fixture::new("demo");
    fx.login();

    let r = fx
        .client
        .get(fx.web_url("/"))
        .send()
        .expect("GET /web/{tenant}/");
    let r = assert_status(r, 200, "web dashboard");
    let ct = r
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();
    assert!(ct.contains("text/html"), "want HTML, got {ct}");
    let body = r.text().unwrap_or_default();
    assert!(body.contains("Demo"), "dashboard should include Demo tile");
}
