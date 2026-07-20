//! Shared test scaffolding for service-layer integration tests.
//!
//! Every helper builds a **fresh in-memory SQLite database** with all
//! migrations applied, and returns a ready-to-use [`AppServices`]. Because
//! we use `?cache=shared` with a unique name per test, each test gets full
//! isolation while allowing the sqlx pool to open multiple connections to
//! the *same* in-memory database.

use std::sync::atomic::{AtomicU64, Ordering};

use chrono::NaiveDate;
use school_erp::db;
use school_erp::repositories::academic_structure::{NewRoom, NewSubject};
use school_erp::repositories::class_enrollment::NewClassSection;
use school_erp::repositories::core::{NewAcademicYear, NewTerm};
use school_erp::repositories::staff::NewStaff;
use school_erp::repositories::students::NewStudent;
use school_erp::services::AppServices;

static COUNTER: AtomicU64 = AtomicU64::new(0);

fn unique_db_url() -> String {
    // A unique in-memory DB per test that supports multiple connections.
    let n = COUNTER.fetch_add(1, Ordering::SeqCst);
    format!(
        "sqlite:file:erp_test_{}_{}?mode=memory&cache=shared",
        std::process::id(),
        n
    )
}

/// Build an empty-but-migrated `AppServices`.
pub async fn fresh() -> AppServices {
    let pool = db::connect(&unique_db_url()).await.expect("connect");
    db::migrate(&pool).await.expect("migrate");
    AppServices::new(pool)
}

/// Convenience date helpers.
pub fn date(y: i32, m: u32, d: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(y, m, d).unwrap()
}

/// A rich fixture with a year, term, a class section, one subject, one room,
/// one active teacher and one active student already enrolled.
pub struct Fixture {
    pub app: AppServices,
    pub year_id: i64,
    pub term_id: i64,
    pub grade_id: i64,
    pub section_id: i64,
    pub subject_id: i64,
    pub room_id: i64,
    pub teacher_id: i64,
    pub class_section_id: i64,
    pub student_id: i64,
}

pub async fn fixture() -> Fixture {
    let app = fresh().await;

    // Year & term
    let year = app.academic.rollover(
        NewAcademicYear {
            name: "2025-2026".into(),
            start_date: date(2025, 6, 1),
            end_date:   date(2026, 5, 31),
            is_current: true,
        },
        vec![NewTerm {
            academic_year_id: 0, // filled by service
            name: "Term 1".into(),
            start_date: date(2025, 6, 1),
            end_date:   date(2025, 10, 31),
        }],
    ).await.unwrap();

    let terms = app.repos.terms.list_for_year(year.id).await.unwrap();
    let term_id = terms[0].id;

    // Reference data — seed migration already gives us grades/sections/roles.
    let grade   = app.repos.grades.list().await.unwrap().into_iter()
        .find(|g| g.name == "Grade 5").unwrap();
    let section = app.repos.sections.list().await.unwrap().into_iter()
        .find(|s| s.name == "A").unwrap();

    let room = app.repos.rooms.create(&NewRoom {
        name: "Room 101".into(),
        capacity: Some(35),
        kind: Some("classroom".into()),
    }).await.unwrap();

    let subject = app.repos.subjects.create(&NewSubject {
        code: "MATH-05".into(),
        name: "Mathematics".into(),
        is_elective: false,
    }).await.unwrap();

    // A teacher
    let teacher = app.people.hire_staff(NewStaff {
        employee_no: "EMP-001".into(),
        user_id: None,
        department_id: None,
        first_name: "Alice".into(),
        last_name:  "Teacher".into(),
        date_of_birth: Some(date(1990, 1, 1)),
        gender: Some("female".into()),
        phone: None,
        email: Some("alice@school.local".into()),
        designation: Some("Teacher".into()),
        employment_type: Some("full_time".into()),
        date_of_joining: date(2025, 5, 1),
        photo_path: None,
    }).await.unwrap();

    // Class section
    let cs = app.repos.class_sections.create(&NewClassSection {
        academic_year_id: year.id,
        grade_id: grade.id,
        section_id: section.id,
        class_teacher_id: Some(teacher.id),
        room_id: Some(room.id),
        capacity: Some(30),
    }).await.unwrap();

    // Attach subject to class with the teacher as instructor.
    app.repos.class_subjects.assign(cs.id, subject.id, Some(teacher.id)).await.unwrap();

    // Admit a student & auto-enroll them.
    let res = app.people.admit(school_erp::services::people::Admission {
        student: NewStudent {
            admission_no: "ADM-001".into(),
            user_id: None,
            first_name: "Bob".into(),
            middle_name: None,
            last_name:  "Student".into(),
            date_of_birth: date(2015, 1, 1),
            gender: Some("male".into()),
            blood_group: None,
            nationality: None,
            religion: None,
            photo_path: None,
            admission_date: date(2025, 6, 1),
            address_line1: None,
            address_line2: None,
            city: None,
            state: None,
            postal_code: None,
            country: None,
        },
        guardian: None,
        enroll_into_class_section: Some(cs.id),
        roll_no: Some(1),
        enrolled_on: Some(date(2025, 6, 1)),
    }).await.unwrap();

    Fixture {
        app,
        year_id: year.id,
        term_id,
        grade_id: grade.id,
        section_id: section.id,
        subject_id: subject.id,
        room_id: room.id,
        teacher_id: teacher.id,
        class_section_id: cs.id,
        student_id: res.student.id,
    }
}
