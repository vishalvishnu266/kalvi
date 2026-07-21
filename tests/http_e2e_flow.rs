//! Full end-to-end business flow through the REST API only.
//!
//! Walks: provision tenant → register admin → login → create academic year
//! → create class section → admit student → generate invoice → record
//! payment → check outstanding + trial balance.
//!
//! All per-tenant URLs use the path-based tenant scheme:
//! `/api/tenant/{tenant}/…`.

mod http_common;

use http_common::spawn;
use serde_json::{json, Value};

#[tokio::test]
async fn full_business_flow_via_rest() {
    let s = spawn().await;
    let t = "acme";
    let base = format!("/api/tenant/{t}");

    // 1. Provision tenant
    s.post("/api/admin/tenants")
        .json(&json!({ "tenant_id": t, "name":"Acme School" }))
        .await.assert_status_ok();

    // 2. Register admin + login
    s.post(&format!("{base}/auth/register"))
        .json(&json!({
            "username":"admin", "email":"admin@acme.local",
            "password":"supersecret", "roles":["admin"]
        }))
        .await.assert_status_ok();

    let login: Value = s.post(&format!("{base}/auth/login"))
        .json(&json!({ "identifier":"admin", "password":"supersecret" }))
        .await.json();
    assert_eq!(login["username"], "admin");

    // 3. Academic year
    let year: Value = s.post(&format!("{base}/academic/years"))
        .json(&json!({
            "name":"2025-2026",
            "start_date":"2025-06-01","end_date":"2026-05-31",
            "is_current": true
        }))
        .await.json();
    let year_id = year["id"].as_i64().unwrap();

    // 4. Pick Grade 5 + Section A (seeded), and create class section.
    let grades: Value  = s.get(&format!("{base}/academic/grades")).await.json();
    let sections: Value = s.get(&format!("{base}/academic/sections")).await.json();
    let grade_id = grades.as_array().unwrap().iter()
        .find(|g| g["name"] == "Grade 5").unwrap()["id"].as_i64().unwrap();
    let section_id = sections.as_array().unwrap().iter()
        .find(|se| se["name"] == "A").unwrap()["id"].as_i64().unwrap();

    let cs: Value = s.post(&format!("{base}/academic/class-sections"))
        .json(&json!({
            "academic_year_id": year_id,
            "grade_id": grade_id,
            "section_id": section_id,
            "class_teacher_id": null,
            "room_id": null,
            "capacity": 30
        }))
        .await.json();
    let cs_id = cs["id"].as_i64().unwrap();

    // 5. Admit + auto-enroll a student
    let admit: Value = s.post(&format!("{base}/people/students/admit"))
        .json(&json!({
            "student": {
                "admission_no":"ADM-001","user_id": null,
                "first_name":"Bob","middle_name": null,"last_name":"Kid",
                "date_of_birth":"2015-01-01","gender":"male",
                "blood_group": null,"nationality": null,"religion": null,
                "photo_path": null,"admission_date":"2025-06-01",
                "address_line1": null,"address_line2": null,
                "city": null,"state": null,"postal_code": null,"country": null
            },
            "guardian": null,
            "enroll_into_class_section": cs_id,
            "roll_no": 1,
            "enrolled_on": "2025-06-01"
        }))
        .await.json();
    let student_id = admit["student"]["id"].as_i64().unwrap();
    assert!(admit["enrollment_id"].is_i64());

    // 6. Fee structure + item (Tuition seeded)
    let cats: Value = s.get(&format!("{base}/fees/categories")).await.json();
    let tuition_id = cats.as_array().unwrap().iter()
        .find(|c| c["name"] == "Tuition").unwrap()["id"].as_i64().unwrap();

    let fs: Value = s.post(&format!("{base}/fees/structures"))
        .json(&json!({
            "academic_year_id": year_id,
            "grade_id": grade_id,
            "name": "Standard"
        }))
        .await.json();
    let fs_id = fs["id"].as_i64().unwrap();

    s.post(&format!("{base}/fees/structures/{fs_id}/items"))
        .json(&json!({
            "fee_category_id": tuition_id,
            "amount_cents": 5000,
            "frequency": "one_time",
            "due_day": null
        }))
        .await.assert_status_ok();

    // 7. Generate invoice
    let inv: Value = s.post(&format!("{base}/fees/invoices/generate"))
        .json(&json!({
            "student_id": student_id,
            "fee_structure_id": fs_id,
            "invoice_no": "INV-0001",
            "issue_date": "2025-06-01",
            "due_date":   "2025-06-30",
            "tax_cents": 0
        }))
        .await.json();
    let inv_id = inv["id"].as_i64().unwrap();
    assert_eq!(inv["total_cents"], 5000);
    assert_eq!(inv["status"], "unpaid");

    // 8. Record payment
    let paid: Value = s.post(&format!("{base}/fees/payments"))
        .json(&json!({
            "receipt_no":"R-1",
            "invoice_id": inv_id,
            "paid_on": "2025-06-05",
            "amount_cents": 5000,
            "method": "cash",
            "reference": null,
            "received_by_staff_id": null
        }))
        .await.json();
    assert_eq!(paid["amount_cents"], 5000);

    // 9. Invoice now paid; outstanding = 0
    let inv_after: Value = s.get(&format!("{base}/fees/invoices/{inv_id}")).await.json();
    assert_eq!(inv_after["status"], "paid");

    let out: Value = s.get(&format!("{base}/fees/outstanding/{student_id}")).await.json();
    assert_eq!(out["outstanding_cents"], 0);

    // 10. Trial balance is balanced (debits == credits, both = 5000)
    let tb: Value = s.get(&format!("{base}/fees/ledger/trial-balance?as_of=2026-03-31")).await.json();
    let rows = tb.as_array().unwrap();
    let (dr, cr): (i64, i64) = rows.iter()
        .map(|r| (r["debit_cents"].as_i64().unwrap(), r["credit_cents"].as_i64().unwrap()))
        .fold((0, 0), |(a, b), (d, c)| (a + d, b + c));
    assert_eq!(dr, cr);
    assert_eq!(dr, 5000);
}
