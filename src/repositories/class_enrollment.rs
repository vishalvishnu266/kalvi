//! Class sections, class-subject assignments, and student enrollment.

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

use crate::error::{RepoError, RepoResult};

// ---------- class_section ----------

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ClassSection {
    pub id: i64,
    pub academic_year_id: i64,
    pub grade_id: i64,
    pub section_id: i64,
    pub class_teacher_id: Option<i64>,
    pub room_id: Option<i64>,
    pub capacity: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewClassSection {
    pub academic_year_id: i64,
    pub grade_id: i64,
    pub section_id: i64,
    pub class_teacher_id: Option<i64>,
    pub room_id: Option<i64>,
    pub capacity: Option<i64>,
}

#[derive(Clone)]
pub struct ClassSectionRepo { pool: SqlitePool }

impl ClassSectionRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn create(&self, c: &NewClassSection) -> RepoResult<ClassSection> {
        let id = sqlx::query_scalar::<_, i64>(
            r#"INSERT INTO class_section
                 (academic_year_id, grade_id, section_id, class_teacher_id, room_id, capacity)
               VALUES (?, ?, ?, ?, ?, ?) RETURNING id"#,
        )
        .bind(c.academic_year_id).bind(c.grade_id).bind(c.section_id)
        .bind(c.class_teacher_id).bind(c.room_id).bind(c.capacity)
        .fetch_one(&self.pool).await?;
        self.get(id).await
    }

    pub async fn get(&self, id: i64) -> RepoResult<ClassSection> {
        sqlx::query_as::<_, ClassSection>("SELECT * FROM class_section WHERE id = ?")
            .bind(id).fetch_optional(&self.pool).await?
            .ok_or(RepoError::NotFound)
    }

    pub async fn list_for_year(&self, year_id: i64) -> RepoResult<Vec<ClassSection>> {
        Ok(sqlx::query_as::<_, ClassSection>(
            "SELECT * FROM class_section WHERE academic_year_id = ? ORDER BY grade_id, section_id",
        ).bind(year_id).fetch_all(&self.pool).await?)
    }

    pub async fn set_class_teacher(&self, id: i64, teacher_id: Option<i64>) -> RepoResult<()> {
        sqlx::query("UPDATE class_section SET class_teacher_id = ? WHERE id = ?")
            .bind(teacher_id).bind(id).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn student_count(&self, id: i64) -> RepoResult<i64> {
        Ok(sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM enrollment WHERE class_section_id = ? AND left_on IS NULL",
        ).bind(id).fetch_one(&self.pool).await?)
    }

    pub async fn delete(&self, id: i64) -> RepoResult<()> {
        let res = sqlx::query("DELETE FROM class_section WHERE id = ?")
            .bind(id).execute(&self.pool).await?;
        if res.rows_affected() == 0 { return Err(RepoError::NotFound); }
        Ok(())
    }
}

// ---------- class_subject ----------

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ClassSubject {
    pub id: i64,
    pub class_section_id: i64,
    pub subject_id: i64,
    pub teacher_id: Option<i64>,
}

#[derive(Clone)]
pub struct ClassSubjectRepo { pool: SqlitePool }

impl ClassSubjectRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn assign(&self, class_section_id: i64, subject_id: i64, teacher_id: Option<i64>)
        -> RepoResult<ClassSubject>
    {
        let id = sqlx::query_scalar::<_, i64>(
            r#"INSERT INTO class_subject (class_section_id, subject_id, teacher_id)
               VALUES (?, ?, ?)
               ON CONFLICT(class_section_id, subject_id) DO UPDATE SET teacher_id = excluded.teacher_id
               RETURNING id"#,
        )
        .bind(class_section_id).bind(subject_id).bind(teacher_id)
        .fetch_one(&self.pool).await?;
        self.get(id).await
    }

    pub async fn get(&self, id: i64) -> RepoResult<ClassSubject> {
        sqlx::query_as::<_, ClassSubject>("SELECT * FROM class_subject WHERE id = ?")
            .bind(id).fetch_optional(&self.pool).await?
            .ok_or(RepoError::NotFound)
    }

    pub async fn list_for_class(&self, class_section_id: i64) -> RepoResult<Vec<ClassSubject>> {
        Ok(sqlx::query_as::<_, ClassSubject>(
            "SELECT * FROM class_subject WHERE class_section_id = ?",
        ).bind(class_section_id).fetch_all(&self.pool).await?)
    }

    pub async fn list_for_teacher(&self, teacher_id: i64) -> RepoResult<Vec<ClassSubject>> {
        Ok(sqlx::query_as::<_, ClassSubject>(
            "SELECT * FROM class_subject WHERE teacher_id = ?",
        ).bind(teacher_id).fetch_all(&self.pool).await?)
    }

    pub async fn unassign(&self, id: i64) -> RepoResult<()> {
        let res = sqlx::query("DELETE FROM class_subject WHERE id = ?")
            .bind(id).execute(&self.pool).await?;
        if res.rows_affected() == 0 { return Err(RepoError::NotFound); }
        Ok(())
    }
}

// ---------- enrollment ----------

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Enrollment {
    pub id: i64,
    pub student_id: i64,
    pub class_section_id: i64,
    pub academic_year_id: i64,
    pub roll_no: Option<i64>,
    pub enrolled_on: NaiveDate,
    pub left_on: Option<NaiveDate>,
    pub result: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewEnrollment {
    pub student_id: i64,
    pub class_section_id: i64,
    pub academic_year_id: i64,
    pub roll_no: Option<i64>,
    pub enrolled_on: NaiveDate,
}

#[derive(Clone)]
pub struct EnrollmentRepo { pool: SqlitePool }

impl EnrollmentRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn enroll(&self, e: &NewEnrollment) -> RepoResult<Enrollment> {
        let id = sqlx::query_scalar::<_, i64>(
            r#"INSERT INTO enrollment
                 (student_id, class_section_id, academic_year_id, roll_no, enrolled_on)
               VALUES (?, ?, ?, ?, ?) RETURNING id"#,
        )
        .bind(e.student_id).bind(e.class_section_id)
        .bind(e.academic_year_id).bind(e.roll_no).bind(e.enrolled_on)
        .fetch_one(&self.pool).await?;
        self.get(id).await
    }

    pub async fn get(&self, id: i64) -> RepoResult<Enrollment> {
        sqlx::query_as::<_, Enrollment>("SELECT * FROM enrollment WHERE id = ?")
            .bind(id).fetch_optional(&self.pool).await?
            .ok_or(RepoError::NotFound)
    }

    pub async fn current_for_student(&self, student_id: i64, academic_year_id: i64)
        -> RepoResult<Option<Enrollment>>
    {
        Ok(sqlx::query_as::<_, Enrollment>(
            "SELECT * FROM enrollment WHERE student_id = ? AND academic_year_id = ?",
        ).bind(student_id).bind(academic_year_id)
        .fetch_optional(&self.pool).await?)
    }

    pub async fn history_for_student(&self, student_id: i64) -> RepoResult<Vec<Enrollment>> {
        Ok(sqlx::query_as::<_, Enrollment>(
            "SELECT * FROM enrollment WHERE student_id = ? ORDER BY enrolled_on DESC",
        ).bind(student_id).fetch_all(&self.pool).await?)
    }

    pub async fn roster(&self, class_section_id: i64) -> RepoResult<Vec<Enrollment>> {
        Ok(sqlx::query_as::<_, Enrollment>(
            r#"SELECT * FROM enrollment
               WHERE class_section_id = ? AND left_on IS NULL
               ORDER BY roll_no NULLS LAST"#,
        ).bind(class_section_id).fetch_all(&self.pool).await?)
    }

    pub async fn mark_result(&self, id: i64, result: &str) -> RepoResult<()> {
        sqlx::query("UPDATE enrollment SET result = ? WHERE id = ?")
            .bind(result).bind(id).execute(&self.pool).await?;
        Ok(())
    }

    /// Close an enrollment (student left / transferred / graduated).
    pub async fn close(&self, id: i64, left_on: NaiveDate, result: Option<&str>) -> RepoResult<()> {
        sqlx::query("UPDATE enrollment SET left_on = ?, result = COALESCE(?, result) WHERE id = ?")
            .bind(left_on).bind(result).bind(id).execute(&self.pool).await?;
        Ok(())
    }

    /// Promote all "promoted" enrollments in `from_class` to `to_class`
    /// (creates fresh enrollments in `to_year`).
    pub async fn promote_class(
        &self,
        from_class_id: i64,
        to_class_id: i64,
        to_year_id: i64,
        enrolled_on: NaiveDate,
    ) -> RepoResult<u64> {
        let res = sqlx::query(
            r#"INSERT INTO enrollment
                 (student_id, class_section_id, academic_year_id, enrolled_on)
               SELECT student_id, ?, ?, ?
                 FROM enrollment
                WHERE class_section_id = ? AND result = 'promoted'"#,
        )
        .bind(to_class_id).bind(to_year_id).bind(enrolled_on).bind(from_class_id)
        .execute(&self.pool).await?;
        Ok(res.rows_affected())
    }
}
