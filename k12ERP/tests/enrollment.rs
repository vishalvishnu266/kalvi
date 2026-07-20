mod common;

use common::{date, fixture};
use school_erp::repositories::class_enrollment::{NewClassSection, NewEnrollment};
use school_erp::repositories::students::NewStudent;
use school_erp::services::people::Admission;
use school_erp::ServiceError;

#[tokio::test]
async fn transfer_reuses_row_and_writes_audit() {
    let fx = fixture().await;

    // Create a second class section (Grade 5, section B).
    let section_b = fx.app.repos.sections.list().await.unwrap()
        .into_iter().find(|s| s.name == "B").unwrap();
    let cs_b = fx.app.repos.class_sections.create(&NewClassSection {
        academic_year_id: fx.year_id,
        grade_id: fx.grade_id,
        section_id: section_b.id,
        class_teacher_id: None,
        room_id: None,
        capacity: Some(10),
    }).await.unwrap();

    let updated = fx.app.enrollment
        .transfer(fx.student_id, cs_b.id, date(2025, 8, 1)).await.unwrap();
    assert_eq!(updated.class_section_id, cs_b.id);

    // Audit trail should record the transfer.
    let entries = fx.app.repos.audit_log
        .for_entity("enrollment", updated.id, 10).await.unwrap();
    assert!(entries.iter().any(|e| e.action == "update"));
}

#[tokio::test]
async fn transferring_to_same_class_is_rejected() {
    let fx = fixture().await;
    let err = fx.app.enrollment
        .transfer(fx.student_id, fx.class_section_id, date(2025, 8, 1))
        .await.unwrap_err();
    assert!(matches!(err, ServiceError::Validation(_)));
}

#[tokio::test]
async fn enrollment_respects_class_capacity() {
    let fx = fixture().await;

    // Shrink capacity to 1 (fixture already has 1 student enrolled).
    sqlx::query("UPDATE class_section SET capacity = 1 WHERE id = ?")
        .bind(fx.class_section_id)
        .execute(&fx.app.repos.pool).await.unwrap();

    // Add another student, try to enroll -> should conflict.
    let extra = fx.app.repos.students.create(&NewStudent {
        admission_no: "ADM-002".into(),
        user_id: None,
        first_name: "Cara".into(), middle_name: None, last_name: "Kid".into(),
        date_of_birth: date(2015, 2, 2),
        gender: None, blood_group: None, nationality: None, religion: None,
        photo_path: None,
        admission_date: date(2025, 6, 1),
        address_line1: None, address_line2: None,
        city: None, state: None, postal_code: None, country: None,
    }).await.unwrap();

    let err = fx.app.enrollment.enroll(NewEnrollment {
        student_id: extra.id,
        class_section_id: fx.class_section_id,
        academic_year_id: fx.year_id,
        roll_no: Some(2),
        enrolled_on: date(2025, 6, 5),
    }).await.unwrap_err();
    assert!(matches!(err, ServiceError::Conflict(_)));
}

#[tokio::test]
async fn admission_with_guardian_creates_link() {
    let fx = fixture().await;

    let res = fx.app.people.admit(Admission {
        student: NewStudent {
            admission_no: "ADM-101".into(),
            user_id: None,
            first_name: "Dave".into(), middle_name: None, last_name: "Kid".into(),
            date_of_birth: date(2015, 3, 3),
            gender: None, blood_group: None, nationality: None, religion: None,
            photo_path: None,
            admission_date: date(2025, 6, 1),
            address_line1: None, address_line2: None,
            city: None, state: None, postal_code: None, country: None,
        },
        guardian: Some((
            school_erp::repositories::guardians::NewGuardian {
                user_id: None,
                first_name: "Dan".into(),
                last_name: "Kid".into(),
                phone: Some("555".into()),
                email: None,
                occupation: None,
                address: None,
            },
            "father".into(),
            true,
        )),
        enroll_into_class_section: Some(fx.class_section_id),
        roll_no: Some(2),
        enrolled_on: Some(date(2025, 6, 2)),
    }).await.unwrap();

    let gid = res.guardian_id.expect("guardian created");
    let students = fx.app.repos.guardians.students_of_guardian(gid).await.unwrap();
    assert!(students.contains(&res.student.id));

    let guardians = fx.app.repos.guardians.guardians_of_student(res.student.id).await.unwrap();
    assert_eq!(guardians.len(), 1);
}
