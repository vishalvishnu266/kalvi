//! Tenant-scoped auth surface — register, login, whoami.
//!
//! Run with: `cargo test --test auth`

mod common;

use common::{assert_status, json_body, Fixture};
use serde_json::json;

#[test]
fn register_returns_user() {
    // Fixture already registers an admin user; this test just
    // asserts the returned row shape by registering a *second* user
    // inside the same tenant.
    let fx = Fixture::new("auth");
    let second = format!("secondary_{}", &fx.tenant[5..]);

    let r = fx
        .client
        .post(fx.api_url("/auth/register"))
        .json(&json!({
            "username": second,
            "password": "another-password-123",
            "roles":    [],
        }))
        .send()
        .expect("POST /auth/register");
    let body = json_body(assert_status(r, 200, "register second user"), "register");
    assert_eq!(body["username"], second);
    assert!(body["id"].as_i64().is_some());
    // password_hash must never be echoed back to clients.
    assert!(
        body.get("password_hash").is_none() || body["password_hash"].is_null(),
        "password_hash should be skipped in JSON responses"
    );
}

#[test]
fn login_sets_session_cookie_and_whoami_sees_tenant() {
    let mut fx = Fixture::new("auth");
    fx.login();

    // whoami is a lightweight surface that just echoes the tenant it
    // resolved from the URL — but it goes through the same
    // `TenantScope` extractor that hydrates session-based auth, so a
    // 200 here proves the cookie was accepted.
    let r = fx
        .client
        .get(fx.api_url("/auth/whoami"))
        .send()
        .expect("GET /auth/whoami");
    let body = json_body(assert_status(r, 200, "whoami"), "whoami");
    assert_eq!(body["tenant"], fx.tenant);
}

#[test]
fn login_rejects_wrong_password() {
    let fx = Fixture::new("auth");
    let r = fx
        .client
        .post(format!("{}/web/{}/login", fx.base, fx.tenant))
        .form(&[
            ("tenant", fx.tenant.as_str()),
            ("identifier", fx.username.as_str()),
            ("password", "definitely-wrong"),
        ])
        .send()
        .expect("POST /web/{tenant}/login (bad pw)");
    // Web login re-renders the form with an error (HTTP 200) rather
    // than a redirect. Assert we did *not* get redirected.
    let sc = r.status().as_u16();
    assert!(
        !(300..400).contains(&sc),
        "bad password should not redirect; got {sc}"
    );
    let body = r.text().unwrap_or_default();
    assert!(
        body.to_lowercase().contains("invalid"),
        "expected 'invalid credentials' hint in body, got:\n{body}"
    );
}
