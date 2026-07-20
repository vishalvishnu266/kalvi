mod common;

use common::{date, fresh};
use school_erp::repositories::core::{NewAcademicYear, NewTerm};
use school_erp::repositories::students::NewStudent;
use school_erp::services::people::Admission;
use school_erp::ServiceError;

#[tokio::test]
async fn rollover_creates_year_and_terms() {
    let app = fresh().await;

    let year = app.academic.rollover(
        NewAcademicYear {
            name: "2025-2026".into(),
            start_date: date(2025, 6, 1),
            end_date: date(2026, 5, 31),
            is_current: true,
        },
        vec![
            NewTerm { academic_year_id: 0, name: "T1".into(),
                     start_date: date(2025, 6, 1),  end_date: date(2025, 10, 31) },
            NewTerm { academic_year_id: 0, name: "T2".into(),
                     start_date: date(2025, 11, 1), end_date: date(2026, 5, 31) },
        ],
    ).await.unwrap();

    assert_eq!(year.name, "2025-2026");
    assert!(year.is_current);

    let terms = app.academic.list_terms(year.id).await.unwrap();
    assert_eq!(terms.len(), 2);
}

#[tokio::test]
async fn only_one_academic_year_is_current() {
    let app = fresh().await;
    app.academic.create_year(NewAcademicYear {
        name: "2024-2025".into(),
        start_date: date(2024, 6, 1),
        end_date: date(2025, 5, 31),
        is_current: true,
    }).await.unwrap();

    app.academic.create_year(NewAcademicYear {
        name: "2025-2026".into(),
        start_date: date(2025, 6, 1),
        end_date: date(2026, 5, 31),
        is_current: true, // rollover should demote the previous one
    }).await.unwrap();

    let current = app.academic.current_year().await.unwrap();
    assert_eq!(current.name, "2025-2026");
}

#[tokio::test]
async fn register_login_and_change_password() {
    let app = fresh().await;

    let user = app.auth
        .register("admin", Some("admin@example.com"), "supersecret", &["admin"])
        .await.unwrap();
    assert!(user.is_active);

    // Wrong password → Unauthorized
    let err = app.auth.login("admin", "wrong").await.unwrap_err();
    assert!(matches!(err, ServiceError::Unauthorized));

    let ok = app.auth.login("admin", "supersecret").await.unwrap();
    assert_eq!(ok.id, user.id);

    // Change password
    app.auth.change_password(user.id, "supersecret", "another-secret").await.unwrap();
    // Old password no longer works
    let err = app.auth.login("admin", "supersecret").await.unwrap_err();
    assert!(matches!(err, ServiceError::Unauthorized));
    // New one does
    app.auth.login("admin", "another-secret").await.unwrap();
}

#[tokio::test]
async fn short_password_is_rejected() {
    let app = fresh().await;
    let err = app.auth.register("x", None, "short", &[]).await.unwrap_err();
    assert!(matches!(err, ServiceError::Validation(_)));
}

#[tokio::test]
async fn duplicate_username_conflicts() {
    let app = fresh().await;
    app.auth.register("bob", None, "supersecret", &[]).await.unwrap();
    let err = app.auth.register("bob", None, "supersecret", &[]).await.unwrap_err();
    assert!(matches!(err, ServiceError::Conflict(_)));
}

#[tokio::test]
async fn duplicate_admission_no_is_rejected() {
    let fx = common::fixture().await;

    let dup = fx.app.people.admit(Admission {
        student: NewStudent {
            admission_no: "ADM-001".into(),   // conflict with fixture
            user_id: None,
            first_name: "Charlie".into(),
            middle_name: None,
            last_name: "Student".into(),
            date_of_birth: date(2015, 1, 1),
            gender: None,
            blood_group: None,
            nationality: None,
            religion: None,
            photo_path: None,
            admission_date: date(2025, 6, 1),
            address_line1: None, address_line2: None,
            city: None, state: None, postal_code: None, country: None,
        },
        guardian: None,
        enroll_into_class_section: None,
        roll_no: None,
        enrolled_on: None,
    }).await.unwrap_err();

    assert!(matches!(dup, ServiceError::Conflict(_)));
}
