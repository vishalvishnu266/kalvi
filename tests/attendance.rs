mod common;

use common::{date, fixture};
use school_erp::services::attendance::BulkMark;
use school_erp::ServiceError;

#[tokio::test]
async fn mark_class_only_accepts_students_in_roster() {
    let fx = fixture().await;

    // Real student
    fx.app.attendance.mark_class(
        fx.class_section_id, date(2025, 6, 10),
        vec![BulkMark { student_id: fx.student_id, status: "present".into(), remarks: None }],
        Some(fx.teacher_id),
    ).await.unwrap();

    // Unknown student id
    let err = fx.app.attendance.mark_class(
        fx.class_section_id, date(2025, 6, 11),
        vec![BulkMark { student_id: 999_999, status: "present".into(), remarks: None }],
        Some(fx.teacher_id),
    ).await.unwrap_err();
    assert!(matches!(err, ServiceError::Validation(_)));
}

#[tokio::test]
async fn attendance_percentage_ignores_absent() {
    let fx = fixture().await;

    // 3 present, 1 absent → 75%
    for (i, status) in ["present", "present", "present", "absent"].iter().enumerate() {
        fx.app.attendance.mark_class(
            fx.class_section_id,
            date(2025, 6, 10 + i as u32),
            vec![BulkMark { student_id: fx.student_id, status: (*status).into(), remarks: None }],
            Some(fx.teacher_id),
        ).await.unwrap();
    }

    let pct = fx.app.attendance
        .percentage(fx.student_id, date(2025, 6, 10), date(2025, 6, 13))
        .await.unwrap();
    assert!((pct - 75.0).abs() < 1e-6, "expected 75.0, got {pct}");
}

#[tokio::test]
async fn class_absentees_lists_only_absent() {
    let fx = fixture().await;
    fx.app.attendance.mark_class(
        fx.class_section_id, date(2025, 6, 10),
        vec![BulkMark { student_id: fx.student_id, status: "absent".into(), remarks: Some("sick".into()) }],
        Some(fx.teacher_id),
    ).await.unwrap();

    let list = fx.app.attendance
        .class_absentees(fx.class_section_id, date(2025, 6, 10)).await.unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].status, "absent");
}
