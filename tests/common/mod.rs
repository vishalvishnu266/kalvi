//! Shared helpers for the out-of-process integration tests.
//!
//! Every test in `tests/` picks up `BASE_URL` from the environment
//! (default `http://127.0.0.1:3000`) and issues real HTTP requests
//! against the running `school_erp` binary. The framework must be
//! booted separately, e.g. via `bash scripts/dev_reset.sh`.
//!
//! Fixture pattern
//! ---------------
//! Most tests just need "a tenant with an admin logged in". Use
//! [`Fixture::new`] at the top of a test and let the returned guard
//! soft-delete the tenant on drop — that way an interrupted run
//! doesn't leave behind orphan rows on your dev server.

// Some helpers are only used by some of the surface-specific test
// binaries; silence dead-code warnings when compiling any one of them.
#![allow(dead_code)]

use std::time::Duration;

use reqwest::blocking::{Client, Response};
use serde_json::{json, Value};

// ─── Environment ────────────────────────────────────────────────────

/// Read `BASE_URL` from the environment. Trailing slash is stripped so
/// callers can safely do `format!("{base}/api/health")`.
pub fn base_url() -> String {
    let raw = std::env::var("BASE_URL").unwrap_or_else(|_| "http://127.0.0.1:3000".to_string());
    raw.trim_end_matches('/').to_string()
}

// ─── Client construction ────────────────────────────────────────────

/// Build a blocking client that:
///   * enables the cookie jar (so `login` followed by an authenticated
///     request just works),
///   * has a short-ish timeout so a dead server fails loudly instead
///     of hanging the test run.
pub fn client() -> Client {
    Client::builder()
        .cookie_store(true)
        .timeout(Duration::from_secs(10))
        .build()
        .expect("build reqwest client")
}

/// Assert the server is up before doing anything else. Returns `false`
/// with a helpful message when it isn't — keeps the suite from
/// spewing noise if someone forgets to start the server.
pub fn require_server(client: &Client) -> bool {
    let url = format!("{}/api/live", base_url());
    match client.get(&url).send() {
        Ok(r) if r.status().is_success() => true,
        Ok(r) => {
            eprintln!(
                "\nSKIP: server at {} is not ready (GET /api/live -> {})",
                base_url(),
                r.status()
            );
            false
        }
        Err(e) => {
            eprintln!(
                "\nSKIP: cannot reach server at {}: {e}\n\
                 Start it with `bash scripts/dev_reset.sh` (or set BASE_URL).",
                base_url()
            );
            false
        }
    }
}

// ─── Assertions with better failure messages ────────────────────────

/// Assert an expected status code, printing the response body on
/// mismatch so failures are copy-pasteable.
#[track_caller]
pub fn assert_status(resp: Response, expected: u16, ctx: &str) -> Response {
    let actual = resp.status().as_u16();
    if actual == expected {
        return resp;
    }
    let body = resp.text().unwrap_or_else(|_| "<unreadable body>".into());
    panic!("[{ctx}] expected HTTP {expected}, got {actual}. body:\n{body}");
}

/// Parse a JSON response into a `serde_json::Value`, panicking with
/// context on failure.
#[track_caller]
pub fn json_body(resp: Response, ctx: &str) -> Value {
    let status = resp.status();
    let text = resp.text().unwrap_or_default();
    serde_json::from_str::<Value>(&text)
        .unwrap_or_else(|e| panic!("[{ctx}] status={status}, invalid JSON: {e}\nbody:\n{text}"))
}

// ─── Identifiers ────────────────────────────────────────────────────

/// Generate a short random tenant id so each test run is isolated
/// from previous ones on the same server.
pub fn random_tenant_id(prefix: &str) -> String {
    let suffix = uuid::Uuid::new_v4().simple().to_string();
    format!("{prefix}_{}", &suffix[..8])
}

/// Millisecond-resolution unique-per-call string. Handy for building
/// distinctive test payload text without pulling in `chrono`.
pub fn ms_stamp() -> String {
    let ns = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    format!("t{ns}")
}

// ─── Low-level admin helpers ────────────────────────────────────────

/// `POST /admin/api/tenants` — creates a tenant and asserts success.
pub fn create_tenant(client: &Client, base: &str, tenant_id: &str) -> Value {
    let r = client
        .post(format!("{base}/admin/api/tenants"))
        .json(&json!({
            "tenant_id": tenant_id,
            "name": format!("Integration Test {tenant_id}"),
            "plan": "test",
            "notes": "created by tests/ — safe to delete",
        }))
        .send()
        .expect("POST /admin/api/tenants");
    json_body(assert_status(r, 200, "create tenant"), "create tenant")
}

/// `DELETE /admin/api/tenants/{tid}` — best-effort soft delete. Never
/// panics; teardown must not mask the original test failure.
pub fn soft_delete_tenant(client: &Client, base: &str, tenant_id: &str) {
    let url = format!("{base}/admin/api/tenants/{tenant_id}");
    match client.delete(&url).send() {
        Ok(r) if r.status().is_success() || r.status().as_u16() == 204 => {}
        Ok(r) => eprintln!(
            "teardown: DELETE {url} returned {} — leaving tenant in place",
            r.status()
        ),
        Err(e) => eprintln!("teardown: DELETE {url} failed: {e}"),
    }
}

// ─── High-level fixture ─────────────────────────────────────────────

/// A ready-to-use tenant + admin user. Created on construction,
/// **soft-deleted on drop** so an interrupted or panicking test still
/// tidies up after itself.
///
/// Fields are public so tests can read `tenant`, `username`, etc.
/// without going through getters.
pub struct Fixture {
    pub client: Client,
    pub base: String,
    pub tenant: String,
    pub username: String,
    pub password: String,
    /// `true` once `logged_in()` has been called successfully.
    pub authenticated: bool,
}

impl Fixture {
    /// Create a fresh tenant with a random id + register an admin
    /// user in it. Does **not** log the user in — call
    /// [`Fixture::login`] when the test needs the session cookie.
    pub fn new(prefix: &str) -> Self {
        let client = client();
        assert!(
            require_server(&client),
            "server at {} is not reachable — start it first",
            base_url()
        );
        let base = base_url();
        let tenant = random_tenant_id(prefix);

        let _created = create_tenant(&client, &base, &tenant);

        let username = format!("admin_{}", &tenant[prefix.len() + 1..]);
        let password = "integration-test-password".to_string();

        let r = client
            .post(format!("{base}/api/{tenant}/auth/register"))
            .json(&json!({
                "username": username,
                "email":    format!("{username}@integration.test"),
                "password": password,
                "roles":    ["admin"],
            }))
            .send()
            .expect("POST /auth/register");
        let _ = json_body(assert_status(r, 200, "register admin"), "register admin");

        Self {
            client,
            base,
            tenant,
            username,
            password,
            authenticated: false,
        }
    }

    /// Log the admin user in via the web form flow. The session
    /// cookie is captured by the client's cookie jar and replayed
    /// automatically on subsequent requests.
    pub fn login(&mut self) {
        let r = self
            .client
            .post(format!("{}/web/{}/login", self.base, self.tenant))
            .form(&[
                ("tenant", self.tenant.as_str()),
                ("identifier", self.username.as_str()),
                ("password", self.password.as_str()),
            ])
            .send()
            .expect("POST /web/{tenant}/login");
        let sc = r.status().as_u16();
        assert!(
            (300..400).contains(&sc),
            "expected redirect from login, got {sc}\nbody:\n{}",
            r.text().unwrap_or_default()
        );
        self.authenticated = true;
    }

    /// Convenience: `<base>/api/{tenant}/...`.
    pub fn api_url(&self, path: &str) -> String {
        format!("{}/api/{}{}", self.base, self.tenant, path)
    }

    /// Convenience: `<base>/web/{tenant}/...`.
    pub fn web_url(&self, path: &str) -> String {
        format!("{}/web/{}{}", self.base, self.tenant, path)
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        // Best-effort teardown. We deliberately reach for a fresh
        // client so a poisoned cookie jar can't stop us cleaning up.
        let admin = client();
        soft_delete_tenant(&admin, &self.base, &self.tenant);
    }
}
