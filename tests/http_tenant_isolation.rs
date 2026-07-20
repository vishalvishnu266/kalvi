//! Tenant isolation proved via the real HTTP surface.

mod http_common;

use http_common::spawn;
use serde_json::{json, Value};

fn tenant_hdr(id: &str) -> (http::HeaderName, http::HeaderValue) {
    ("x-tenant-id".parse().unwrap(), id.parse().unwrap())
}

#[tokio::test]
async fn two_tenants_have_independent_data() {
    let s = spawn().await;

    // Provision two tenants
    for tid in ["acme", "globex"] {
        s.post("/api/admin/tenants")
            .json(&json!({ "tenant_id": tid, "name": format!("{tid} School") }))
            .await
            .assert_status_ok();
    }

    let (h_a, v_a) = tenant_hdr("acme");
    let (h_g, v_g) = tenant_hdr("globex");

    // Admit the same admission_no in each tenant — should both succeed
    // because the DBs are separate.
    let admit_body = |first: &str, last: &str| json!({
        "student": {
            "admission_no": "ADM-001",
            "user_id": null,
            "first_name": first, "middle_name": null, "last_name": last,
            "date_of_birth": "2015-01-01", "gender": "male",
            "blood_group": null, "nationality": null, "religion": null,
            "photo_path": null, "admission_date": "2025-06-01",
            "address_line1": null, "address_line2": null,
            "city": null, "state": null, "postal_code": null, "country": null
        },
        "guardian": null,
        "enroll_into_class_section": null,
        "roll_no": null,
        "enrolled_on": null
    });

    s.post("/api/tenant/people/students/admit")
        .add_header(h_a.clone(), v_a.clone())
        .json(&admit_body("Alice", "Acme"))
        .await.assert_status_ok();

    s.post("/api/tenant/people/students/admit")
        .add_header(h_g.clone(), v_g.clone())
        .json(&admit_body("Alice", "Globex"))
        .await.assert_status_ok();

    // List students separately.
    let acme_list: Value = s.get("/api/tenant/people/students")
        .add_header(h_a.clone(), v_a.clone())
        .await.json();
    let globex_list: Value = s.get("/api/tenant/people/students")
        .add_header(h_g, v_g)
        .await.json();

    let acme_students   = acme_list.as_array().unwrap();
    let globex_students = globex_list.as_array().unwrap();

    assert_eq!(acme_students.len(), 1);
    assert_eq!(globex_students.len(), 1);
    assert_eq!(acme_students[0]["last_name"],   "Acme");
    assert_eq!(globex_students[0]["last_name"], "Globex");
}

#[tokio::test]
async fn openapi_and_docs_are_served() {
    let s = spawn().await;

    let spec = s.get("/api/openapi.json").await;
    spec.assert_status_ok();
    let json: Value = spec.json();
    assert_eq!(json["info"]["title"], "School ERP API");

    // Swagger UI HTML entry point (index.html served under /api/docs/).
    let docs = s.get("/api/docs/").await;
    // utoipa-swagger-ui returns a redirect or 200 depending on version;
    // just assert it's not a hard error.
    assert!(docs.status_code().is_success() || docs.status_code().is_redirection());
}
