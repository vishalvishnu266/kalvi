//! Multi-tenant isolation: two tenants sharing the same process must not see
//! each other's data.

use std::sync::Arc;

use school_erp::repositories::students::NewStudent;
use school_erp::services::people::Admission;
use school_erp::tenancy::{
    AllowAllGuard, InMemoryTenantResolver, TenantId, TenantRegistry, TenantRegistryConfig,
};

fn dt(y: i32, m: u32, d: u32) -> chrono::NaiveDate {
    chrono::NaiveDate::from_ymd_opt(y, m, d).unwrap()
}

#[tokio::test]
async fn two_tenants_do_not_leak_data() {
    let registry = TenantRegistry::new(TenantRegistryConfig {
        resolver: Arc::new(InMemoryTenantResolver),
        guard:    Arc::new(AllowAllGuard),
        auto_migrate: true,
    });

    let acme_id   = TenantId::new("acme").unwrap();
    let globex_id = TenantId::new("globex").unwrap();

    let acme   = registry.services_for(&acme_id).await.unwrap();
    let globex = registry.services_for(&globex_id).await.unwrap();

    // Admit "Alice" only in acme.
    acme.people.admit(Admission {
        student: NewStudent {
            admission_no: "A-1".into(), user_id: None,
            first_name: "Alice".into(), middle_name: None, last_name: "A".into(),
            date_of_birth: dt(2015, 1, 1),
            gender: None, blood_group: None, nationality: None, religion: None,
            photo_path: None, admission_date: dt(2025, 6, 1),
            address_line1: None, address_line2: None,
            city: None, state: None, postal_code: None, country: None,
        },
        guardian: None,
        enroll_into_class_section: None,
        roll_no: None, enrolled_on: None,
    }).await.unwrap();

    // Same admission_no should succeed for globex — separate DBs.
    globex.people.admit(Admission {
        student: NewStudent {
            admission_no: "A-1".into(), user_id: None,
            first_name: "Alice".into(), middle_name: None, last_name: "G".into(),
            date_of_birth: dt(2015, 1, 1),
            gender: None, blood_group: None, nationality: None, religion: None,
            photo_path: None, admission_date: dt(2025, 6, 1),
            address_line1: None, address_line2: None,
            city: None, state: None, postal_code: None, country: None,
        },
        guardian: None,
        enroll_into_class_section: None,
        roll_no: None, enrolled_on: None,
    }).await.unwrap();

    let acme_count   = acme.repos.students.count_by_status("active").await.unwrap();
    let globex_count = globex.repos.students.count_by_status("active").await.unwrap();
    assert_eq!(acme_count, 1);
    assert_eq!(globex_count, 1);

    // Sanity: last_name differs to prove they're distinct rows in distinct DBs.
    let s_a = acme.repos.students.find_by_admission_no("A-1").await.unwrap().unwrap();
    let s_g = globex.repos.students.find_by_admission_no("A-1").await.unwrap().unwrap();
    assert_eq!(s_a.last_name, "A");
    assert_eq!(s_g.last_name, "G");
}
