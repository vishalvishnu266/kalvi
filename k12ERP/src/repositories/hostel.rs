//! Hostel: buildings, rooms, and student allocations.

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

use crate::error::{RepoError, RepoResult};

// ---------- Hostel ----------

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Hostel {
    pub id: i64,
    pub name: String,
    pub r#type: Option<String>,
}

#[derive(Clone)]
pub struct HostelRepo { pool: SqlitePool }

impl HostelRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn create(&self, name: &str, kind: Option<&str>) -> RepoResult<Hostel> {
        let id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO hostel (name, type) VALUES (?, ?) RETURNING id",
        ).bind(name).bind(kind).fetch_one(&self.pool).await?;
        self.get(id).await
    }

    pub async fn get(&self, id: i64) -> RepoResult<Hostel> {
        sqlx::query_as::<_, Hostel>("SELECT id, name, type FROM hostel WHERE id = ?")
            .bind(id).fetch_optional(&self.pool).await?
            .ok_or(RepoError::NotFound)
    }

    pub async fn list(&self) -> RepoResult<Vec<Hostel>> {
        Ok(sqlx::query_as::<_, Hostel>("SELECT id, name, type FROM hostel ORDER BY name")
            .fetch_all(&self.pool).await?)
    }
}

// ---------- Hostel room ----------

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct HostelRoom {
    pub id: i64,
    pub hostel_id: i64,
    pub room_no: String,
    pub capacity: i64,
}

#[derive(Clone)]
pub struct HostelRoomRepo { pool: SqlitePool }

impl HostelRoomRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn create(&self, hostel_id: i64, room_no: &str, capacity: i64)
        -> RepoResult<HostelRoom>
    {
        if capacity <= 0 { return Err(RepoError::validation("capacity must be > 0")); }
        let id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO hostel_room (hostel_id, room_no, capacity) VALUES (?, ?, ?) RETURNING id",
        ).bind(hostel_id).bind(room_no).bind(capacity).fetch_one(&self.pool).await?;
        self.get(id).await
    }

    pub async fn get(&self, id: i64) -> RepoResult<HostelRoom> {
        sqlx::query_as::<_, HostelRoom>("SELECT * FROM hostel_room WHERE id = ?")
            .bind(id).fetch_optional(&self.pool).await?
            .ok_or(RepoError::NotFound)
    }

    pub async fn rooms_in(&self, hostel_id: i64) -> RepoResult<Vec<HostelRoom>> {
        Ok(sqlx::query_as::<_, HostelRoom>(
            "SELECT * FROM hostel_room WHERE hostel_id = ? ORDER BY room_no",
        ).bind(hostel_id).fetch_all(&self.pool).await?)
    }

    pub async fn occupancy(&self, room_id: i64) -> RepoResult<i64> {
        Ok(sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM hostel_allocation WHERE hostel_room_id = ? AND to_date IS NULL",
        ).bind(room_id).fetch_one(&self.pool).await?)
    }
}

// ---------- Allocation ----------

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct HostelAllocation {
    pub id: i64,
    pub student_id: i64,
    pub hostel_room_id: i64,
    pub from_date: NaiveDate,
    pub to_date: Option<NaiveDate>,
}

#[derive(Clone)]
pub struct HostelAllocationRepo { pool: SqlitePool }

impl HostelAllocationRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn allocate(
        &self, student_id: i64, hostel_room_id: i64, from_date: NaiveDate,
    ) -> RepoResult<HostelAllocation> {
        let mut tx = self.pool.begin().await?;

        // Check room capacity.
        let (cap, occ): (i64, i64) = sqlx::query_as(
            r#"SELECT hr.capacity,
                      (SELECT COUNT(*) FROM hostel_allocation
                        WHERE hostel_room_id = hr.id AND to_date IS NULL)
                 FROM hostel_room hr WHERE hr.id = ?"#,
        ).bind(hostel_room_id).fetch_optional(&mut *tx).await?
         .ok_or(RepoError::NotFound)?;
        if occ >= cap { return Err(RepoError::conflict("room is full")); }

        // Close any existing active allocation for the student.
        sqlx::query(
            r#"UPDATE hostel_allocation
                 SET to_date = date(?, '-1 day')
               WHERE student_id = ? AND to_date IS NULL"#,
        ).bind(from_date).bind(student_id).execute(&mut *tx).await?;

        let id = sqlx::query_scalar::<_, i64>(
            r#"INSERT INTO hostel_allocation (student_id, hostel_room_id, from_date)
               VALUES (?, ?, ?) RETURNING id"#,
        ).bind(student_id).bind(hostel_room_id).bind(from_date)
         .fetch_one(&mut *tx).await?;

        tx.commit().await?;

        sqlx::query_as::<_, HostelAllocation>("SELECT * FROM hostel_allocation WHERE id = ?")
            .bind(id).fetch_one(&self.pool).await.map_err(Into::into)
    }

    pub async fn vacate(&self, id: i64, to_date: NaiveDate) -> RepoResult<()> {
        sqlx::query("UPDATE hostel_allocation SET to_date = ? WHERE id = ?")
            .bind(to_date).bind(id).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn active_for_student(&self, student_id: i64) -> RepoResult<Option<HostelAllocation>> {
        Ok(sqlx::query_as::<_, HostelAllocation>(
            "SELECT * FROM hostel_allocation WHERE student_id = ? AND to_date IS NULL",
        ).bind(student_id).fetch_optional(&self.pool).await?)
    }
}
