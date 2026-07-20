//! Reference data: grades, sections, rooms, subjects.

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

use crate::error::{RepoError, RepoResult};

// ---------- Grade ----------

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Grade {
    pub id: i64,
    pub name: String,
    pub level: i64,
}

#[derive(Clone)]
pub struct GradeRepo {
    pool: SqlitePool,
}

impl GradeRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn create(&self, name: &str, level: i64) -> RepoResult<Grade> {
        let id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO grade (name, level) VALUES (?, ?) RETURNING id",
        )
        .bind(name)
        .bind(level)
        .fetch_one(&self.pool)
        .await?;
        self.get(id).await
    }

    pub async fn get(&self, id: i64) -> RepoResult<Grade> {
        sqlx::query_as::<_, Grade>("SELECT * FROM grade WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?
            .ok_or(RepoError::NotFound)
    }

    pub async fn list(&self) -> RepoResult<Vec<Grade>> {
        Ok(sqlx::query_as::<_, Grade>("SELECT * FROM grade ORDER BY level")
            .fetch_all(&self.pool)
            .await?)
    }

    pub async fn delete(&self, id: i64) -> RepoResult<()> {
        let res = sqlx::query("DELETE FROM grade WHERE id = ?")
            .bind(id).execute(&self.pool).await?;
        if res.rows_affected() == 0 { return Err(RepoError::NotFound); }
        Ok(())
    }
}

// ---------- Section ----------

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Section {
    pub id: i64,
    pub name: String,
}

#[derive(Clone)]
pub struct SectionRepo { pool: SqlitePool }

impl SectionRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn create(&self, name: &str) -> RepoResult<Section> {
        let id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO section (name) VALUES (?) RETURNING id",
        )
        .bind(name).fetch_one(&self.pool).await?;
        self.get(id).await
    }

    pub async fn get(&self, id: i64) -> RepoResult<Section> {
        sqlx::query_as::<_, Section>("SELECT * FROM section WHERE id = ?")
            .bind(id).fetch_optional(&self.pool).await?
            .ok_or(RepoError::NotFound)
    }

    pub async fn list(&self) -> RepoResult<Vec<Section>> {
        Ok(sqlx::query_as::<_, Section>("SELECT * FROM section ORDER BY name")
            .fetch_all(&self.pool).await?)
    }
}

// ---------- Room ----------

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Room {
    pub id: i64,
    pub name: String,
    pub capacity: Option<i64>,
    pub kind: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewRoom {
    pub name: String,
    pub capacity: Option<i64>,
    pub kind: Option<String>,
}

#[derive(Clone)]
pub struct RoomRepo { pool: SqlitePool }

impl RoomRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn create(&self, r: &NewRoom) -> RepoResult<Room> {
        let id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO room (name, capacity, kind) VALUES (?, ?, ?) RETURNING id",
        )
        .bind(&r.name).bind(r.capacity).bind(&r.kind)
        .fetch_one(&self.pool).await?;
        self.get(id).await
    }

    pub async fn get(&self, id: i64) -> RepoResult<Room> {
        sqlx::query_as::<_, Room>("SELECT * FROM room WHERE id = ?")
            .bind(id).fetch_optional(&self.pool).await?
            .ok_or(RepoError::NotFound)
    }

    pub async fn list(&self) -> RepoResult<Vec<Room>> {
        Ok(sqlx::query_as::<_, Room>("SELECT * FROM room ORDER BY name")
            .fetch_all(&self.pool).await?)
    }

    pub async fn delete(&self, id: i64) -> RepoResult<()> {
        let res = sqlx::query("DELETE FROM room WHERE id = ?")
            .bind(id).execute(&self.pool).await?;
        if res.rows_affected() == 0 { return Err(RepoError::NotFound); }
        Ok(())
    }
}

// ---------- Subject ----------

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Subject {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub is_elective: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewSubject {
    pub code: String,
    pub name: String,
    pub is_elective: bool,
}

#[derive(Clone)]
pub struct SubjectRepo { pool: SqlitePool }

impl SubjectRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn create(&self, s: &NewSubject) -> RepoResult<Subject> {
        let id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO subject (code, name, is_elective) VALUES (?, ?, ?) RETURNING id",
        )
        .bind(&s.code).bind(&s.name).bind(s.is_elective as i64)
        .fetch_one(&self.pool).await?;
        self.get(id).await
    }

    pub async fn get(&self, id: i64) -> RepoResult<Subject> {
        sqlx::query_as::<_, Subject>("SELECT * FROM subject WHERE id = ?")
            .bind(id).fetch_optional(&self.pool).await?
            .ok_or(RepoError::NotFound)
    }

    pub async fn list(&self) -> RepoResult<Vec<Subject>> {
        Ok(sqlx::query_as::<_, Subject>("SELECT * FROM subject ORDER BY code")
            .fetch_all(&self.pool).await?)
    }

    pub async fn delete(&self, id: i64) -> RepoResult<()> {
        let res = sqlx::query("DELETE FROM subject WHERE id = ?")
            .bind(id).execute(&self.pool).await?;
        if res.rows_affected() == 0 { return Err(RepoError::NotFound); }
        Ok(())
    }
}
