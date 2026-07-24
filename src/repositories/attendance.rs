use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

use crate::error::{RepoError, RepoResult};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct StudentAttendance {
    pub id: i64,
    pub student_id: i64,
    pub class_section_id: i64,
    pub date: NaiveDate,
    pub status: String,
    pub remarks: Option<String>,
    pub marked_by_staff_id: Option<i64>,
    pub marked_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkStudent {
    pub student_id: i64,
    pub class_section_id: i64,
    pub date: NaiveDate,
    pub status: String,
    pub remarks: Option<String>,
    pub marked_by_staff_id: Option<i64>,
}

#[derive(Clone)]
pub struct StudentAttendanceRepo { pool: SqlitePool }

impl StudentAttendanceRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

pub async fn mark(&self, m: &MarkStudent) -> RepoResult<StudentAttendance> {
        let id = sqlx::query_scalar::<_, i64>(
            r#"INSERT INTO student_attendance
                 (student_id, class_section_id, date, status, remarks, marked_by_staff_id)
               VALUES (?, ?, ?, ?, ?, ?)
               ON CONFLICT(student_id, date) DO UPDATE SET
                 class_section_id   = excluded.class_section_id,
                 status             = excluded.status,
                 remarks            = excluded.remarks,
                 marked_by_staff_id = excluded.marked_by_staff_id,
                 marked_at          = datetime('now')
               RETURNING id"#,
        )
        .bind(m.student_id).bind(m.class_section_id).bind(m.date)
        .bind(&m.status).bind(&m.remarks).bind(m.marked_by_staff_id)
        .fetch_one(&self.pool).await?;

        sqlx::query_as::<_, StudentAttendance>("SELECT * FROM student_attendance WHERE id = ?")
            .bind(id).fetch_one(&self.pool).await.map_err(Into::into)
    }

pub async fn mark_bulk(&self, marks: &[MarkStudent]) -> RepoResult<usize> {
        let mut tx = self.pool.begin().await?;
        for m in marks {
            sqlx::query(
                r#"INSERT INTO student_attendance
                     (student_id, class_section_id, date, status, remarks, marked_by_staff_id)
                   VALUES (?, ?, ?, ?, ?, ?)
                   ON CONFLICT(student_id, date) DO UPDATE SET
                     class_section_id   = excluded.class_section_id,
                     status             = excluded.status,
                     remarks            = excluded.remarks,
                     marked_by_staff_id = excluded.marked_by_staff_id,
                     marked_at          = datetime('now')"#,
            )
            .bind(m.student_id).bind(m.class_section_id).bind(m.date)
            .bind(&m.status).bind(&m.remarks).bind(m.marked_by_staff_id)
            .execute(&mut *tx).await?;
        }
        tx.commit().await?;
        Ok(marks.len())
    }

    pub async fn for_student_between(
        &self, student_id: i64, from: NaiveDate, to: NaiveDate,
    ) -> RepoResult<Vec<StudentAttendance>> {
        Ok(sqlx::query_as::<_, StudentAttendance>(
            r#"SELECT * FROM student_attendance
               WHERE student_id = ? AND date BETWEEN ? AND ?
               ORDER BY date"#,
        ).bind(student_id).bind(from).bind(to).fetch_all(&self.pool).await?)
    }

    pub async fn for_class_on(
        &self, class_section_id: i64, date: NaiveDate,
    ) -> RepoResult<Vec<StudentAttendance>> {
        Ok(sqlx::query_as::<_, StudentAttendance>(
            r#"SELECT * FROM student_attendance
               WHERE class_section_id = ? AND date = ?"#,
        ).bind(class_section_id).bind(date).fetch_all(&self.pool).await?)
    }

pub async fn percentage(
        &self, student_id: i64, from: NaiveDate, to: NaiveDate,
    ) -> RepoResult<f64> {
        let (present, total): (Option<i64>, Option<i64>) = sqlx::query_as(
            r#"SELECT
                 SUM(CASE WHEN status IN ('present','late','half_day') THEN 1 ELSE 0 END),
                 COUNT(*)
               FROM student_attendance
               WHERE student_id = ? AND date BETWEEN ? AND ?"#,
        ).bind(student_id).bind(from).bind(to).fetch_one(&self.pool).await?;

        let p = present.unwrap_or(0) as f64;
        let t = total.unwrap_or(0) as f64;
        Ok(if t == 0.0 { 0.0 } else { (p / t) * 100.0 })
    }
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct StaffAttendance {
    pub id: i64,
    pub staff_id: i64,
    pub date: NaiveDate,
    pub status: String,
    pub check_in: Option<NaiveTime>,
    pub check_out: Option<NaiveTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkStaff {
    pub staff_id: i64,
    pub date: NaiveDate,
    pub status: String,
    pub check_in: Option<NaiveTime>,
    pub check_out: Option<NaiveTime>,
}

#[derive(Clone)]
pub struct StaffAttendanceRepo { pool: SqlitePool }

impl StaffAttendanceRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn mark(&self, m: &MarkStaff) -> RepoResult<StaffAttendance> {
        let id = sqlx::query_scalar::<_, i64>(
            r#"INSERT INTO staff_attendance (staff_id, date, status, check_in, check_out)
               VALUES (?, ?, ?, ?, ?)
               ON CONFLICT(staff_id, date) DO UPDATE SET
                 status    = excluded.status,
                 check_in  = COALESCE(excluded.check_in,  staff_attendance.check_in),
                 check_out = COALESCE(excluded.check_out, staff_attendance.check_out)
               RETURNING id"#,
        )
        .bind(m.staff_id).bind(m.date).bind(&m.status).bind(m.check_in).bind(m.check_out)
        .fetch_one(&self.pool).await?;

        sqlx::query_as::<_, StaffAttendance>("SELECT * FROM staff_attendance WHERE id = ?")
            .bind(id).fetch_one(&self.pool).await.map_err(Into::into)
    }

    pub async fn for_staff_between(
        &self, staff_id: i64, from: NaiveDate, to: NaiveDate,
    ) -> RepoResult<Vec<StaffAttendance>> {
        Ok(sqlx::query_as::<_, StaffAttendance>(
            "SELECT * FROM staff_attendance WHERE staff_id = ? AND date BETWEEN ? AND ? ORDER BY date",
        ).bind(staff_id).bind(from).bind(to).fetch_all(&self.pool).await?)
    }
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct LeaveRequest {
    pub id: i64,
    pub subject_id: i64,
    pub from_date: NaiveDate,
    pub to_date: NaiveDate,
    pub reason: Option<String>,
    pub status: String,
    pub approver_id: Option<i64>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Copy)]
pub enum LeaveKind { Student, Staff }

impl LeaveKind {
    fn table(self) -> &'static str {
        match self {
            LeaveKind::Student => "student_leave_request",
            LeaveKind::Staff   => "staff_leave_request",
        }
    }
    fn subject_column(self) -> &'static str {
        match self {
            LeaveKind::Student => "student_id",
            LeaveKind::Staff   => "staff_id",
        }
    }
}

#[derive(Clone)]
pub struct LeaveRepo { pool: SqlitePool }

impl LeaveRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn create(
        &self, kind: LeaveKind, subject_id: i64,
        from: NaiveDate, to: NaiveDate, reason: Option<&str>,
    ) -> RepoResult<i64> {
        if from > to { return Err(RepoError::validation("from_date must be <= to_date")); }
        let sql = format!(
            "INSERT INTO {} ({}, from_date, to_date, reason) VALUES (?, ?, ?, ?) RETURNING id",
            kind.table(), kind.subject_column(),
        );
        let id = sqlx::query_scalar::<_, i64>(&sql)
            .bind(subject_id).bind(from).bind(to).bind(reason)
            .fetch_one(&self.pool).await?;
        Ok(id)
    }

    pub async fn approve(&self, kind: LeaveKind, id: i64, approver_id: i64) -> RepoResult<()> {
        self.transition(kind, id, "approved", Some(approver_id)).await
    }

    pub async fn reject(&self, kind: LeaveKind, id: i64, approver_id: i64) -> RepoResult<()> {
        self.transition(kind, id, "rejected", Some(approver_id)).await
    }

    pub async fn cancel(&self, kind: LeaveKind, id: i64) -> RepoResult<()> {
        self.transition(kind, id, "cancelled", None).await
    }

    async fn transition(
        &self, kind: LeaveKind, id: i64, status: &str, approver: Option<i64>,
    ) -> RepoResult<()> {
        let sql = format!(
            "UPDATE {} SET status = ?, approver_id = COALESCE(?, approver_id) WHERE id = ?",
            kind.table(),
        );
        let res = sqlx::query(&sql)
            .bind(status).bind(approver).bind(id)
            .execute(&self.pool).await?;
        if res.rows_affected() == 0 { return Err(RepoError::NotFound); }
        Ok(())
    }

    pub async fn list_pending(&self, kind: LeaveKind) -> RepoResult<Vec<LeaveRequest>> {
        let col = kind.subject_column();
        let sql = format!(
            r#"SELECT id, {col} AS subject_id, from_date, to_date, reason, status, approver_id, created_at
                 FROM {} WHERE status = 'pending' ORDER BY created_at"#,
            kind.table(),
        );
        Ok(sqlx::query_as::<_, LeaveRequest>(&sql).fetch_all(&self.pool).await?)
    }
}
