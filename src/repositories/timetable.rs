//! Timetable: periods + timetable slots.

use chrono::NaiveTime;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

use crate::error::{RepoError, RepoResult};

// ---------- Period ----------

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Period {
    pub id: i64,
    pub name: String,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub is_break: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewPeriod {
    pub name: String,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub is_break: bool,
}

#[derive(Clone)]
pub struct PeriodRepo { pool: SqlitePool }

impl PeriodRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn create(&self, p: &NewPeriod) -> RepoResult<Period> {
        if p.start_time >= p.end_time {
            return Err(RepoError::validation("start_time must be < end_time"));
        }
        let id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO period (name, start_time, end_time, is_break) VALUES (?, ?, ?, ?) RETURNING id",
        )
        .bind(&p.name).bind(p.start_time).bind(p.end_time).bind(p.is_break as i64)
        .fetch_one(&self.pool).await?;
        self.get(id).await
    }

    pub async fn get(&self, id: i64) -> RepoResult<Period> {
        sqlx::query_as::<_, Period>("SELECT * FROM period WHERE id = ?")
            .bind(id).fetch_optional(&self.pool).await?
            .ok_or(RepoError::NotFound)
    }

    pub async fn list(&self) -> RepoResult<Vec<Period>> {
        Ok(sqlx::query_as::<_, Period>("SELECT * FROM period ORDER BY start_time")
            .fetch_all(&self.pool).await?)
    }
}

// ---------- Timetable slot ----------

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct TimetableSlot {
    pub id: i64,
    pub class_section_id: i64,
    pub subject_id: Option<i64>,
    pub teacher_id: Option<i64>,
    pub room_id: Option<i64>,
    pub day_of_week: i64,
    pub period_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewSlot {
    pub class_section_id: i64,
    pub subject_id: Option<i64>,
    pub teacher_id: Option<i64>,
    pub room_id: Option<i64>,
    pub day_of_week: i64,   // 1..7
    pub period_id: i64,
}

#[derive(Clone)]
pub struct TimetableRepo { pool: SqlitePool }

impl TimetableRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn upsert(&self, s: &NewSlot) -> RepoResult<TimetableSlot> {
        if !(1..=7).contains(&s.day_of_week) {
            return Err(RepoError::validation("day_of_week must be 1..7"));
        }
        // Unique on (class_section_id, day_of_week, period_id)
        let id = sqlx::query_scalar::<_, i64>(
            r#"INSERT INTO timetable_slot
                 (class_section_id, subject_id, teacher_id, room_id, day_of_week, period_id)
               VALUES (?, ?, ?, ?, ?, ?)
               ON CONFLICT(class_section_id, day_of_week, period_id) DO UPDATE SET
                 subject_id = excluded.subject_id,
                 teacher_id = excluded.teacher_id,
                 room_id    = excluded.room_id
               RETURNING id"#,
        )
        .bind(s.class_section_id).bind(s.subject_id).bind(s.teacher_id).bind(s.room_id)
        .bind(s.day_of_week).bind(s.period_id)
        .fetch_one(&self.pool).await?;

        sqlx::query_as::<_, TimetableSlot>("SELECT * FROM timetable_slot WHERE id = ?")
            .bind(id).fetch_one(&self.pool).await.map_err(Into::into)
    }

    pub async fn for_class(&self, class_section_id: i64) -> RepoResult<Vec<TimetableSlot>> {
        Ok(sqlx::query_as::<_, TimetableSlot>(
            r#"SELECT * FROM timetable_slot
               WHERE class_section_id = ? ORDER BY day_of_week, period_id"#,
        ).bind(class_section_id).fetch_all(&self.pool).await?)
    }

    pub async fn for_teacher(&self, teacher_id: i64) -> RepoResult<Vec<TimetableSlot>> {
        Ok(sqlx::query_as::<_, TimetableSlot>(
            r#"SELECT * FROM timetable_slot
               WHERE teacher_id = ? ORDER BY day_of_week, period_id"#,
        ).bind(teacher_id).fetch_all(&self.pool).await?)
    }

    pub async fn for_room(&self, room_id: i64) -> RepoResult<Vec<TimetableSlot>> {
        Ok(sqlx::query_as::<_, TimetableSlot>(
            r#"SELECT * FROM timetable_slot
               WHERE room_id = ? ORDER BY day_of_week, period_id"#,
        ).bind(room_id).fetch_all(&self.pool).await?)
    }

    pub async fn delete(&self, id: i64) -> RepoResult<()> {
        let res = sqlx::query("DELETE FROM timetable_slot WHERE id = ?")
            .bind(id).execute(&self.pool).await?;
        if res.rows_affected() == 0 { return Err(RepoError::NotFound); }
        Ok(())
    }
}
