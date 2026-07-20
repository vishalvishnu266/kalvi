mod common;

use common::{date, fixture};
use school_erp::repositories::auth::NewUser;
use school_erp::repositories::communication::NewAnnouncement;
use school_erp::repositories::discipline::NewIncident;
use school_erp::repositories::guardians::{NewGuardian, StudentGuardianLink};

#[tokio::test]
async fn broadcast_pushes_notifications_to_students_with_user() {
    let fx = fixture().await;

    // Attach a login to the fixture student.
    let user = fx.app.repos.users.create(&NewUser {
        username: "bob.student".into(),
        email: None,
        password_hash: "not-a-real-hash".into(),
        is_active: true,
    }).await.unwrap();
    sqlx::query("UPDATE student SET user_id = ? WHERE id = ?")
        .bind(user.id).bind(fx.student_id)
        .execute(&fx.app.repos.pool).await.unwrap();

    let (ann, pushed) = fx.app.communication.broadcast(NewAnnouncement {
        title: "Holiday".into(),
        body: "No school tomorrow".into(),
        audience: "students".into(),
        class_section_id: None,
        expires_at: None,
        created_by_user_id: None,
    }).await.unwrap();

    assert!(!ann.title.is_empty());
    assert_eq!(pushed, 1);

    let inbox = fx.app.repos.notifications.for_user(user.id, true, 10).await.unwrap();
    assert_eq!(inbox.len(), 1);
    assert_eq!(inbox[0].title, "Holiday");
}

#[tokio::test]
async fn discipline_report_notifies_linked_guardian() {
    let fx = fixture().await;

    // Guardian with a user account
    let guardian_user = fx.app.repos.users.create(&NewUser {
        username: "father".into(), email: None,
        password_hash: "hash".into(), is_active: true,
    }).await.unwrap();
    let g = fx.app.repos.guardians.create(&NewGuardian {
        user_id: Some(guardian_user.id),
        first_name: "Dan".into(), last_name: "Kid".into(),
        phone: None, email: None, occupation: None, address: None,
    }).await.unwrap();
    fx.app.repos.guardians.link(&StudentGuardianLink {
        student_id: fx.student_id, guardian_id: g.id,
        relationship: "father".into(),
        is_primary: true, is_emergency: true, can_pickup: true,
    }).await.unwrap();

    fx.app.discipline.report(NewIncident {
        student_id: fx.student_id,
        date: date(2025, 6, 15),
        description: "Late to class".into(),
        severity: Some("low".into()),
        action_taken: Some("Warning".into()),
        reported_by_staff_id: Some(fx.teacher_id),
    }, true).await.unwrap();

    let notes = fx.app.repos.notifications
        .for_user(guardian_user.id, true, 10).await.unwrap();
    assert_eq!(notes.len(), 1);
    assert_eq!(notes[0].kind.as_deref(), Some("discipline"));
}

#[tokio::test]
async fn health_bmi_categories() {
    let fx = fixture().await;
    fx.app.health.upsert_vitals(
        fx.student_id, Some(160.0), Some(60.0), None, None,
    ).await.unwrap();
    let bmi = fx.app.health.bmi_for(fx.student_id).await.unwrap().unwrap();
    assert!((bmi.bmi - 23.4375).abs() < 0.01);
    assert_eq!(bmi.category, "normal");
}
