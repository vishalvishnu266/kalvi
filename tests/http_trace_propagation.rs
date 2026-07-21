//! Verifies that `x-request-id` is propagated end-to-end.
//!
//! * When a client omits `x-request-id`, the server generates one and echoes it
//!   in the response.
//! * When a client sends `x-request-id`, the server preserves it verbatim.
//!
//! (The `tenant_id` field recorded into the tracing span isn't visible to
//! HTTP clients — it shows up in server logs. We assert its side channel by
//! confirming the tenant middleware still works, which we already cover in
//! `http_admin_tenants.rs` and `http_tenant_isolation.rs`.)

mod http_common;

use http_common::spawn;
use serde_json::json;

#[tokio::test]
async fn server_generates_request_id_when_client_omits_it() {
    let s = spawn().await;

    let res = s.get("/api/health").await;
    res.assert_status_ok();

    let id = res.header("x-request-id");
    let id_str = id.to_str().expect("valid utf-8");
    assert!(!id_str.is_empty(), "server must always emit x-request-id");
    // UUID v4 canonical form is 36 chars incl. dashes.
    assert_eq!(id_str.len(), 36, "expected UUID v4 shape, got {id_str}");
}

#[tokio::test]
async fn server_preserves_client_request_id() {
    let s = spawn().await;

    let custom = "test-abc-123";
    let res = s.get("/api/health")
        .add_header("x-request-id".parse().unwrap(), custom.parse().unwrap())
        .await;
    res.assert_status_ok();

    assert_eq!(res.header("x-request-id").to_str().unwrap(), custom);
}

#[tokio::test]
async fn request_id_flows_through_tenant_scope() {
    let s = spawn().await;

    // Provision a tenant so we can hit tenant-scoped routes.
    s.post("/api/admin/tenants")
        .json(&json!({ "tenant_id":"acme", "name":"Acme" }))
        .await.assert_status_ok();

    let custom = "trace-tenant-42";
    let res = s.get("/api/tenant/acme/auth/whoami")
        .add_header("x-request-id".parse().unwrap(), custom.parse().unwrap())
        .await;
    res.assert_status_ok();

    // Middleware must not lose the request id.
    assert_eq!(res.header("x-request-id").to_str().unwrap(), custom);
}
