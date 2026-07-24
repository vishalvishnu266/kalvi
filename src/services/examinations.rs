use std::sync::Arc;

use crate::repositories::Repositories;
use crate::repositories::examinations::{
    EnterResult, ExamResult, ExamSchedule, NewExam, NewSchedule,
};
use crate::services::{ServiceError, ServiceResult};

#[derive(Debug, Clone)]
pub struct ReportCardRow {
    pub subject_id: i64,
    pub max_marks: f64,
    pub marks_obtained: Option<f64>,
    pub percent: Option<f64>,
    pub letter: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ReportCard {
    pub student_id: i64,
    pub exam_id: i64,
    pub rows: Vec<ReportCardRow>,
    pub total_max: f64,
    pub total_obtained: f64,
    pub overall_percent: f64,
    pub overall_letter: Option<String>,
}

#[derive(Clone)]
pub struct ExaminationService {
    repos: Arc<Repositories>,
}

impl ExaminationService {
    pub fn new(repos: Arc<Repositories>) -> Self { Self { repos } }

    pub async fn create_exam(&self, e: NewExam) -> ServiceResult<crate::repositories::examinations::Exam> {
        if e.weightage <= 0.0 {
            return Err(ServiceError::validation("weightage must be > 0"));
        }
        Ok(self.repos.exams.create(&e).await?)
    }

    pub async fn schedule(&self, s: NewSchedule) -> ServiceResult<ExamSchedule> {
        Ok(self.repos.exam_schedules.create(&s).await?)
    }

pub async fn enter_result(&self, mut r: EnterResult) -> ServiceResult<ExamResult> {
        let sched = self.repos.exam_schedules.get(r.exam_schedule_id).await?;
        let exam  = self.repos.exams.get(sched.exam_id).await?;

        if r.grade_letter.is_none() {
            if let (Some(marks), Some(scale_id)) = (r.marks_obtained, exam.grading_scale_id) {
                let percent = (marks / sched.max_marks) * 100.0;
                if let Some(l) = self.repos.grading_scales.letter_for(scale_id, percent).await? {
                    r.grade_letter = Some(l);
                }
            }
        }

        Ok(self.repos.exam_results.upsert(&r).await?)
    }

pub async fn report_card(&self, student_id: i64, exam_id: i64) -> ServiceResult<ReportCard> {
        let raw = self.repos.exam_results.report_card(student_id, exam_id).await?;
        let exam = self.repos.exams.get(exam_id).await?;

        let mut rows = Vec::with_capacity(raw.len());
        let (mut total_max, mut total_obtained) = (0.0_f64, 0.0_f64);

        for (subject_id, marks_obtained, max_marks) in raw {
            total_max += max_marks;
            let percent = marks_obtained.map(|m| (m / max_marks) * 100.0);
            let letter = if let (Some(p), Some(sid)) = (percent, exam.grading_scale_id) {
                self.repos.grading_scales.letter_for(sid, p).await?
            } else { None };
            if let Some(m) = marks_obtained { total_obtained += m; }
            rows.push(ReportCardRow { subject_id, max_marks, marks_obtained, percent, letter });
        }

        let overall_percent = if total_max == 0.0 { 0.0 } else { (total_obtained / total_max) * 100.0 };
        let overall_letter = if let Some(sid) = exam.grading_scale_id {
            self.repos.grading_scales.letter_for(sid, overall_percent).await?
        } else { None };

        Ok(ReportCard {
            student_id, exam_id, rows,
            total_max, total_obtained, overall_percent, overall_letter,
        })
    }
}
