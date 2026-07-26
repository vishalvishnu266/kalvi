//! Control-plane admin API — tenant CRUD.
//!
//! Run with: `cargo test --test admin`

mod common;

use common::{
    assert_status, base_url, client, json_body, random_tenant_id, require_server, soft_delete_tenant,
};
use serde_json::json;

#[test]
fn create_and_list_tenant() {
    let c = client();
    if !require_server(&c) {
        return;
    }
    let base = base_url();
    let tid = random_tenant_id("adm");

    // Create.
    let r = c
        .post(format!("{base}/admin/api/tenants"))
        .json(&json!({
            "tenant_id": tid,
            "name": "Admin CRUD test",
        }))
        .send()
        .expect("POST /admin/api/tenants");
    let created = json_body(assert_status(r, 200, "create tenant"), "create tenant");
    assert_eq!(created["tenant_id"], tid);
    assert_eq!(created["status"], "active");

    // Show up in list.
    let r = c
        .get(format!("{base}/admin/api/tenants"))
        .send()
        .expect("GET /admin/api/tenants");
    let list = json_body(assert_status(r, 200, "list tenants"), "list tenants");
    let arr = list.as_array().expect("list is an array");
    let hit = arr.iter().any(|t| t["tenant_id"] == tid);
    assert!(hit, "newly-created tenant {tid} missing from list");

    // Cleanup.
    soft_delete_tenant(&c, &base, &tid);
}

#[test]
fn create_tenant_rejects_invalid_id() {
    let c = client();
    if !require_server(&c) {
        return;
    }
    let r = c
        .post(format!("{}/admin/api/tenants", base_url()))
        .json(&json!({
            "tenant_id": "has spaces and !!!",
            "name": "invalid",
        }))
        .send()
        .expect("POST /admin/api/tenants (invalid id)");
    let sc = r.status().as_u16();
    assert!(
        sc == 400 || sc == 422,
        "expected client error for invalid tenant id, got {sc}"
    );
}

#[test]
fn disable_evicts_tenant() {
    let c = client();
    if !require_server(&c) {
        return;
    }
    let base = base_url();
    let tid = random_tenant_id("adm");

    let _ = json_body(
        assert_status(
            c.post(format!("{base}/admin/api/tenants"))
                .json(&json!({ "tenant_id": tid, "name": "disable test" }))
                .send()
                .expect("create"),
            200,
            "create tenant",
        ),
        "create tenant",
    );

    let r = c
        .post(format!("{base}/admin/api/tenants/{tid}/disable"))
        .send()
        .expect("POST /disable");
    let disabled = json_body(assert_status(r, 200, "disable"), "disable");
    assert_eq!(disabled["status"], "disabled");

    // Re-enable so cleanup can delete cleanly.
    let _ = c
        .post(format!("{base}/admin/api/tenants/{tid}/enable"))
        .send();
    soft_delete_tenant(&c, &base, &tid);
}
