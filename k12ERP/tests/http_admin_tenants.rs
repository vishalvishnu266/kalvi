//! HTTP integration tests for `/api/admin/tenants/*`.

mod http_common;

use http_common::spawn;
use serde_json::json;

#[tokio::test]
async fn health_endpoint_works() {
    let s = spawn().await;
    let res = s.get("/api/health").await;
    res.assert_status_ok();
    assert_eq!(res.text(), "ok");
}

#[tokio::test]
async fn create_list_get_disable_enable_delete() {
    let s = spawn().await;

    // Initially empty
    let list = s.get("/api/admin/tenants").await;
    list.assert_status_ok();
    assert_eq!(list.json::<serde_json::Value>().as_array().unwrap().len(), 0);

    // Create
    let created = s.post("/api/admin/tenants")
        .json(&json!({ "tenant_id":"acme", "name":"Acme School" }))
        .await;
    created.assert_status_ok();
    let created_body: serde_json::Value = created.json();
    assert_eq!(created_body["tenant_id"], "acme");
    assert_eq!(created_body["status"], "active");

    // List → 1
    let list2 = s.get("/api/admin/tenants").await;
    assert_eq!(list2.json::<serde_json::Value>().as_array().unwrap().len(), 1);

    // Get by tenant_id
    let one = s.get("/api/admin/tenants/acme").await;
    one.assert_status_ok();
    assert_eq!(one.json::<serde_json::Value>()["name"], "Acme School");

    // Disable
    let disabled = s.post("/api/admin/tenants/acme/disable").await;
    disabled.assert_status_ok();
    assert_eq!(disabled.json::<serde_json::Value>()["status"], "disabled");

    // Enable
    let enabled = s.post("/api/admin/tenants/acme/enable").await;
    enabled.assert_status_ok();
    assert_eq!(enabled.json::<serde_json::Value>()["status"], "active");

    // Soft delete
    let del = s.delete("/api/admin/tenants/acme").await;
    del.assert_status(http::StatusCode::NO_CONTENT);

    // Now hidden from list (soft-deleted rows excluded)
    let list3 = s.get("/api/admin/tenants").await;
    assert_eq!(list3.json::<serde_json::Value>().as_array().unwrap().len(), 0);
}

#[tokio::test]
async fn invalid_tenant_id_is_rejected() {
    let s = spawn().await;
    let bad = s.post("/api/admin/tenants")
        .json(&json!({ "tenant_id":"has space", "name":"X" }))
        .await;
    bad.assert_status(http::StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn disabled_tenant_cannot_use_business_api() {
    let s = spawn().await;

    s.post("/api/admin/tenants").json(&json!({
        "tenant_id":"acme", "name":"A"
    })).await.assert_status_ok();

    // Works while active
    s.get("/api/tenant/auth/whoami")
        .add_header("x-tenant-id".parse().unwrap(), "acme".parse().unwrap())
        .await
        .assert_status_ok();

    // Disable it
    s.post("/api/admin/tenants/acme/disable").await.assert_status_ok();

    // Now the tenant guard should reject
    let rejected = s.get("/api/tenant/auth/whoami")
        .add_header("x-tenant-id".parse().unwrap(), "acme".parse().unwrap())
        .await;
    rejected.assert_status(http::StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn unknown_tenant_is_404() {
    let s = spawn().await;
    let rej = s.get("/api/tenant/auth/whoami")
        .add_header("x-tenant-id".parse().unwrap(), "ghost".parse().unwrap())
        .await;
    rej.assert_status(http::StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn missing_tenant_header_is_400() {
    let s = spawn().await;
    let rej = s.get("/api/tenant/auth/whoami").await;
    rej.assert_status(http::StatusCode::BAD_REQUEST);
}
