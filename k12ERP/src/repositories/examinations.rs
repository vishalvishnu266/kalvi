//! Exams, exam schedules, grading scales, and results.

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

use crate::error::{RepoError, RepoResult};

// ---------- Grading Scale ----------

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct GradingScale {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct GradeBand {
    pub id: i64,
    pub grading_scale_id: i64,
    pub letter: String,
    pub min_percent: f64,
    pub max_percent: f64,
    pub grade_point: Option<f64>,
    pub remarks: Option<String>,
}

#[derive(Clone)]
pub struct GradingScaleRepo { pool: SqlitePool }

impl GradingScaleRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn create_scale(&self, name: &str) -> RepoResult<GradingScale> {
        let id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO grading_scale (name) VALUES (?) RETURNING id",
        ).bind(name).fetch_one(&self.pool).await?;
        Ok(GradingScale { id, name: name.to_string() })
    }

    pub async fn add_band(&self, b: &GradeBand) -> RepoResult<i64> {
        Ok(sqlx::query_scalar::<_, i64>(
            r#"INSERT INTO grade_band
                 (grading_scale_id, letter, min_percent, max_percent, grade_point, remarks)
               VALUES (?, ?, ?, ?, ?, ?) RETURNING id"#,
        )
        .bind(b.grading_scale_id).bind(&b.letter)
        .bind(b.min_percent).bind(b.max_percent)
        .bind(b.grade_point).bind(&b.remarks)
        .fetch_one(&self.pool).await?)
    }

    pub async fn bands(&self, scale_id: i64) -> RepoResult<Vec<GradeBand>> {
        Ok(sqlx::query_as::<_, GradeBand>(
            "SELECT * FROM grade_band WHERE grading_scale_id = ? ORDER BY min_percent DESC",
        ).bind(scale_id).fetch_all(&self.pool).await?)
    }

    pub async fn letter_for(&self, scale_id: i64, percent: f64) -> RepoResult<Option<String>> {
        Ok(sqlx::query_scalar::<_, String>(
            r#"SELECT letter FROM grade_band
               WHERE grading_scale_id = ? AND ? BETWEEN min_percent AND max_percent
               ORDER BY min_percent DESC LIMIT 1"#,
        ).bind(scale_id).bind(percent).fetch_optional(&self.pool).await?)
    }
}

// ---------- Exam ----------

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Exam {
    pub id: i64,
    pub term_id: i64,
    pub name: String,
    pub weightage: f64,
    pub grading_scale_id: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct NewExam {
    pub term_id: i64,
    pub name: String,
    pub weightage: f64,
    pub grading_scale_id: Option<i64>,
}

#[derive(Clone)]
pub struct ExamRepo { pool: SqlitePool }

impl ExamRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn create(&self, e: &NewExam) -> RepoResult<Exam> {
        let id = sqlx::query_scalar::<_, i64>(
            r#"INSERT INTO exam (term_id, name, weightage, grading_scale_id)
               VALUES (?, ?, ?, ?) RETURNING id"#,
        )
        .bind(e.term_id).bind(&e.name).bind(e.weightage).bind(e.grading_scale_id)
        .fetch_one(&self.pool).await?;
        self.get(id).await
    }

    pub async fn get(&self, id: i64) -> RepoResult<Exam> {
        sqlx::query_as::<_, Exam>("SELECT * FROM exam WHERE id = ?")
            .bind(id).fetch_optional(&self.pool).await?
            .ok_or(RepoError::NotFound)
    }

    pub async fn list_for_term(&self, term_id: i64) -> RepoResult<Vec<Exam>> {
        Ok(sqlx::query_as::<_, Exam>(
            "SELECT * FROM exam WHERE term_id = ? ORDER BY name",
        ).bind(term_id).fetch_all(&self.pool).await?)
    }
}

// ---------- Exam schedule ----------

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ExamSchedule {
    pub id: i64,
    pub exam_id: i64,
    pub class_section_id: i64,
    pub subject_id: i64,
    pub exam_date: NaiveDate,
    pub start_time: Option<NaiveTime>,
    pub end_time: Option<NaiveTime>,
    pub max_marks: f64,
    pub pass_marks: f64,
    pub room_id: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct NewSchedule {
    pub exam_id: i64,
    pub class_section_id: i64,
    pub subject_id: i64,
    pub exam_date: NaiveDate,
    pub start_time: Option<NaiveTime>,
    pub end_time: Option<NaiveTime>,
    pub max_marks: f64,
    pub pass_marks: f64,
    pub room_id: Option<i64>,
}

#[derive(Clone)]
pub struct ExamScheduleRepo { pool: SqlitePool }

impl ExamScheduleRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn create(&self, s: &NewSchedule) -> RepoResult<ExamSchedule> {
        if s.max_marks <= 0.0 { return Err(RepoError::validation("max_marks must be > 0")); }
        if s.pass_marks < 0.0 || s.pass_marks > s.max_marks {
            return Err(RepoError::validation("pass_marks must be within [0, max_marks]"));
        }
        let id = sqlx::query_scalar::<_, i64>(
            r#"INSERT INTO exam_schedule
                 (exam_id, class_section_id, subject_id, exam_date, start_time, end_time,
                  max_marks, pass_marks, room_id)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?) RETURNING id"#,
        )
        .bind(s.exam_id).bind(s.class_section_id).bind(s.subject_id)
        .bind(s.exam_date).bind(s.start_time).bind(s.end_time)
        .bind(s.max_marks).bind(s.pass_marks).bind(s.room_id)
        .fetch_one(&self.pool).await?;
        self.get(id).await
    }

    pub async fn get(&self, id: i64) -> RepoResult<ExamSchedule> {
        sqlx::query_as::<_, ExamSchedule>("SELECT * FROM exam_schedule WHERE id = ?")
            .bind(id).fetch_optional(&self.pool).await?
            .ok_or(RepoError::NotFound)
    }

    pub async fn for_exam(&self, exam_id: i64) -> RepoResult<Vec<ExamSchedule>> {
        Ok(sqlx::query_as::<_, ExamSchedule>(
            "SELECT * FROM exam_schedule WHERE exam_id = ? ORDER BY exam_date, start_time",
        ).bind(exam_id).fetch_all(&self.pool).await?)
    }

    pub async fn for_class(&self, class_section_id: i64) -> RepoResult<Vec<ExamSchedule>> {
        Ok(sqlx::query_as::<_, ExamSchedule>(
            "SELECT * FROM exam_schedule WHERE class_section_id = ? ORDER BY exam_date",
        ).bind(class_section_id).fetch_all(&self.pool).await?)
    }
}

// ---------- Result ----------

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ExamResult {
    pub id: i64,
    pub exam_schedule_id: i64,
    pub student_id: i64,
    pub marks_obtained: Option<f64>,
    pub grade_letter: Option<String>,
    pub is_absent: bool,
    pub remarks: Option<String>,
    pub entered_by_staff_id: Option<i64>,
    pub entered_at: NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct EnterResult {
    pub exam_schedule_id: i64,
    pub student_id: i64,
    pub marks_obtained: Option<f64>,
    pub grade_letter: Option<String>,
    pub is_absent: bool,
    pub remarks: Option<String>,
    pub entered_by_staff_id: Option<i64>,
}

#[derive(Clone)]
pub struct ExamResultRepo { pool: SqlitePool }

impl ExamResultRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn upsert(&self, r: &EnterResult) -> RepoResult<ExamResult> {
        let id = sqlx::query_scalar::<_, i64>(
            r#"INSERT INTO exam_result
                 (exam_schedule_id, student_id, marks_obtained, grade_letter,
                  is_absent, remarks, entered_by_staff_id)
               VALUES (?, ?, ?, ?, ?, ?, ?)
               ON CONFLICT(exam_schedule_id, student_id) DO UPDATE SET
                 marks_obtained      = excluded.marks_obtained,
                 grade_letter        = excluded.grade_letter,
                 is_absent           = excluded.is_absent,
                 remarks             = excluded.remarks,
                 entered_by_staff_id = excluded.entered_by_staff_id,
                 entered_at          = datetime('now')
               RETURNING id"#,
        )
        .bind(r.exam_schedule_id).bind(r.student_id)
        .bind(r.marks_obtained).bind(&r.grade_letter)
        .bind(r.is_absent as i64).bind(&r.remarks)
        .bind(r.entered_by_staff_id)
        .fetch_one(&self.pool).await?;

        sqlx::query_as::<_, ExamResult>("SELECT * FROM exam_result WHERE id = ?")
            .bind(id).fetch_one(&self.pool).await.map_err(Into::into)
    }

    pub async fn for_student(&self, student_id: i64) -> RepoResult<Vec<ExamResult>> {
        Ok(sqlx::query_as::<_, ExamResult>(
            "SELECT * FROM exam_result WHERE student_id = ? ORDER BY entered_at DESC",
        ).bind(student_id).fetch_all(&self.pool).await?)
    }

    pub async fn for_schedule(&self, exam_schedule_id: i64) -> RepoResult<Vec<ExamResult>> {
        Ok(sqlx::query_as::<_, ExamResult>(
            "SELECT * FROM exam_result WHERE exam_schedule_id = ?",
        ).bind(exam_schedule_id).fetch_all(&self.pool).await?)
    }

    /// Simple report card: (subject_id, marks, max) for a whole exam.
    pub async fn report_card(
        &self, student_id: i64, exam_id: i64,
    ) -> RepoResult<Vec<(i64, Option<f64>, f64)>> {
        let rows: Vec<(i64, Option<f64>, f64)> = sqlx::query_as(
            r#"SELECT es.subject_id, er.marks_obtained, es.max_marks
               FROM exam_schedule es
               LEFT JOIN exam_result er
                 ON er.exam_schedule_id = es.id AND er.student_id = ?
               WHERE es.exam_id = ?
               ORDER BY es.subject_id"#,
        ).bind(student_id).bind(exam_id).fetch_all(&self.pool).await?;
        Ok(rows)
    }
}
