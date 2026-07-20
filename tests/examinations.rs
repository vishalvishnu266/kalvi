mod common;

use common::{date, fixture};
use school_erp::repositories::academic_structure::NewSubject;
use school_erp::repositories::examinations::{
    EnterResult, GradeBand, NewExam, NewSchedule,
};

#[tokio::test]
async fn enter_result_auto_derives_grade_letter() {
    let fx = fixture().await;

    // Create a simple A/B/C scale.
    let scale = fx.app.repos.grading_scales.create_scale("Simple").await.unwrap();
    for band in [
        GradeBand { id: 0, grading_scale_id: scale.id, letter: "A".into(),
                    min_percent: 80.0, max_percent: 100.0, grade_point: Some(4.0), remarks: None },
        GradeBand { id: 0, grading_scale_id: scale.id, letter: "B".into(),
                    min_percent: 60.0, max_percent: 79.99, grade_point: Some(3.0), remarks: None },
        GradeBand { id: 0, grading_scale_id: scale.id, letter: "C".into(),
                    min_percent: 0.0,  max_percent: 59.99, grade_point: Some(2.0), remarks: None },
    ] { fx.app.repos.grading_scales.add_band(&band).await.unwrap(); }

    let exam = fx.app.examinations.create_exam(NewExam {
        term_id: fx.term_id,
        name: "Mid-term".into(),
        weightage: 1.0,
        grading_scale_id: Some(scale.id),
    }).await.unwrap();

    let sched = fx.app.examinations.schedule(NewSchedule {
        exam_id: exam.id,
        class_section_id: fx.class_section_id,
        subject_id: fx.subject_id,
        exam_date: date(2025, 9, 1),
        start_time: None, end_time: None,
        max_marks: 100.0, pass_marks: 35.0,
        room_id: Some(fx.room_id),
    }).await.unwrap();

    // 85 → should map to "A"
    let r = fx.app.examinations.enter_result(EnterResult {
        exam_schedule_id: sched.id,
        student_id: fx.student_id,
        marks_obtained: Some(85.0),
        grade_letter: None,
        is_absent: false, remarks: None,
        entered_by_staff_id: Some(fx.teacher_id),
    }).await.unwrap();
    assert_eq!(r.grade_letter.as_deref(), Some("A"));
}

#[tokio::test]
async fn report_card_aggregates_across_subjects() {
    let fx = fixture().await;

    // Second subject on the class
    let subj_b = fx.app.repos.subjects.create(&NewSubject {
        code: "ENG-05".into(), name: "English".into(), is_elective: false,
    }).await.unwrap();
    fx.app.repos.class_subjects.assign(fx.class_section_id, subj_b.id, Some(fx.teacher_id))
        .await.unwrap();

    let exam = fx.app.examinations.create_exam(NewExam {
        term_id: fx.term_id, name: "Final".into(),
        weightage: 1.0, grading_scale_id: None,
    }).await.unwrap();

    let sched_a = fx.app.examinations.schedule(NewSchedule {
        exam_id: exam.id, class_section_id: fx.class_section_id, subject_id: fx.subject_id,
        exam_date: date(2025, 10, 1), start_time: None, end_time: None,
        max_marks: 100.0, pass_marks: 35.0, room_id: None,
    }).await.unwrap();
    let sched_b = fx.app.examinations.schedule(NewSchedule {
        exam_id: exam.id, class_section_id: fx.class_section_id, subject_id: subj_b.id,
        exam_date: date(2025, 10, 2), start_time: None, end_time: None,
        max_marks: 50.0, pass_marks: 20.0, room_id: None,
    }).await.unwrap();

    for (sid, marks) in [(sched_a.id, 80.0), (sched_b.id, 40.0)] {
        fx.app.examinations.enter_result(EnterResult {
            exam_schedule_id: sid, student_id: fx.student_id,
            marks_obtained: Some(marks),
            grade_letter: None, is_absent: false, remarks: None,
            entered_by_staff_id: Some(fx.teacher_id),
        }).await.unwrap();
    }

    let rc = fx.app.examinations.report_card(fx.student_id, exam.id).await.unwrap();
    assert_eq!(rc.rows.len(), 2);
    assert!((rc.total_max - 150.0).abs() < 1e-6);
    assert!((rc.total_obtained - 120.0).abs() < 1e-6);
    assert!((rc.overall_percent - 80.0).abs() < 1e-6);
}
