mod common;

use common::fixture;
use school_erp::repositories::timetable::NewSlot;
use school_erp::ServiceError;

fn new_period(app: &school_erp::services::AppServices, name: &str) -> impl std::future::Future<Output=i64> {
    let app = app.clone();
    let name = name.to_string();
    async move {
        let p = app.repos.periods.create(&school_erp::repositories::timetable::NewPeriod {
            name,
            start_time: chrono::NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
            end_time:   chrono::NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            is_break: false,
        }).await.unwrap();
        p.id
    }
}

#[tokio::test]
async fn subject_must_be_assigned_to_class_before_slotting() {
    let fx = fixture().await;
    let p1 = new_period(&fx.app, "P1").await;

    // Create another subject NOT attached to the class section.
    let other = fx.app.repos.subjects.create(
        &school_erp::repositories::academic_structure::NewSubject {
            code: "SCI-05".into(), name: "Science".into(), is_elective: false,
        },
    ).await.unwrap();

    let err = fx.app.timetable.set_slot(NewSlot {
        class_section_id: fx.class_section_id,
        subject_id: Some(other.id),   // not in class_subject
        teacher_id: Some(fx.teacher_id),
        room_id: Some(fx.room_id),
        day_of_week: 1, period_id: p1,
    }).await.unwrap_err();

    assert!(matches!(err, ServiceError::Validation(_)));
}

#[tokio::test]
async fn teacher_double_booking_is_prevented_by_db_index() {
    let fx = fixture().await;
    let p1 = new_period(&fx.app, "P1").await;

    // Book teacher for class A, day 1, P1
    fx.app.timetable.set_slot(NewSlot {
        class_section_id: fx.class_section_id,
        subject_id: Some(fx.subject_id),
        teacher_id: Some(fx.teacher_id),
        room_id: Some(fx.room_id),
        day_of_week: 1, period_id: p1,
    }).await.unwrap();

    // Second class needs same teacher at same slot → the DB partial-unique
    // index should reject the attempt.
    let section_b = fx.app.repos.sections.list().await.unwrap()
        .into_iter().find(|s| s.name == "B").unwrap();
    let cs_b = fx.app.repos.class_sections.create(
        &school_erp::repositories::class_enrollment::NewClassSection {
            academic_year_id: fx.year_id,
            grade_id: fx.grade_id,
            section_id: section_b.id,
            class_teacher_id: None,
            room_id: None,
            capacity: None,
        },
    ).await.unwrap();
    // Attach subject to B (so validation passes)
    fx.app.repos.class_subjects.assign(cs_b.id, fx.subject_id, Some(fx.teacher_id))
        .await.unwrap();

    let err = fx.app.timetable.set_slot(NewSlot {
        class_section_id: cs_b.id,
        subject_id: Some(fx.subject_id),
        teacher_id: Some(fx.teacher_id),
        room_id: None,
        day_of_week: 1, period_id: p1,
    }).await.unwrap_err();

    // DB uniqueness violation surfaces as a repo error inside ServiceError.
    match err {
        ServiceError::Repo(_) | ServiceError::Sqlx(_) => {}
        other => panic!("expected DB conflict, got {other:?}"),
    }
}
