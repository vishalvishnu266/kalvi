//! Document attachments (polymorphic owner).
//!
//! The `owner_type/owner_id` pair is validated here at the Rust layer since
//! SQLite cannot express polymorphic foreign keys.

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

use crate::error::{RepoError, RepoResult};

pub const OWNER_TYPES: &[&str] = &[
    "student", "staff", "guardian", "invoice", "payment", "other",
];

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Document {
    pub id: i64,
    pub owner_type: String,
    pub owner_id: i64,
    pub kind: Option<String>,
    pub file_path: String,
    pub mime_type: Option<String>,
    pub size_bytes: Option<i64>,
    pub uploaded_by_user_id: Option<i64>,
    pub uploaded_at: NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct NewDocument {
    pub owner_type: String,
    pub owner_id: i64,
    pub kind: Option<String>,
    pub file_path: String,
    pub mime_type: Option<String>,
    pub size_bytes: Option<i64>,
    pub uploaded_by_user_id: Option<i64>,
}

#[derive(Clone)]
pub struct DocumentRepo { pool: SqlitePool }

impl DocumentRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn attach(&self, d: &NewDocument) -> RepoResult<Document> {
        if !OWNER_TYPES.contains(&d.owner_type.as_str()) {
            return Err(RepoError::validation("invalid owner_type"));
        }
        // Optional soft-check that the owner row exists (best-effort).
        let owner_exists = match d.owner_type.as_str() {
            "student"  => Self::exists(&self.pool, "student", d.owner_id).await?,
            "staff"    => Self::exists(&self.pool, "staff", d.owner_id).await?,
            "guardian" => Self::exists(&self.pool, "guardian", d.owner_id).await?,
            "invoice"  => Self::exists(&self.pool, "fee_invoice", d.owner_id).await?,
            "payment"  => Self::exists(&self.pool, "fee_payment", d.owner_id).await?,
            _ => true,
        };
        if !owner_exists {
            return Err(RepoError::NotFound);
        }

        let id = sqlx::query_scalar::<_, i64>(
            r#"INSERT INTO document
                 (owner_type, owner_id, kind, file_path, mime_type, size_bytes, uploaded_by_user_id)
               VALUES (?, ?, ?, ?, ?, ?, ?) RETURNING id"#,
        )
        .bind(&d.owner_type).bind(d.owner_id).bind(&d.kind).bind(&d.file_path)
        .bind(&d.mime_type).bind(d.size_bytes).bind(d.uploaded_by_user_id)
        .fetch_one(&self.pool).await?;

        self.get(id).await
    }

    async fn exists(pool: &SqlitePool, table: &str, id: i64) -> RepoResult<bool> {
        let sql = format!("SELECT 1 FROM {} WHERE id = ?", table);
        Ok(sqlx::query_scalar::<_, i64>(&sql).bind(id).fetch_optional(pool).await?.is_some())
    }

    pub async fn get(&self, id: i64) -> RepoResult<Document> {
        sqlx::query_as::<_, Document>("SELECT * FROM document WHERE id = ?")
            .bind(id).fetch_optional(&self.pool).await?
            .ok_or(RepoError::NotFound)
    }

    pub async fn for_owner(&self, owner_type: &str, owner_id: i64) -> RepoResult<Vec<Document>> {
        Ok(sqlx::query_as::<_, Document>(
            "SELECT * FROM document WHERE owner_type = ? AND owner_id = ? ORDER BY uploaded_at DESC",
        ).bind(owner_type).bind(owner_id).fetch_all(&self.pool).await?)
    }

    pub async fn delete(&self, id: i64) -> RepoResult<()> {
        let res = sqlx::query("DELETE FROM document WHERE id = ?")
            .bind(id).execute(&self.pool).await?;
        if res.rows_affected() == 0 { return Err(RepoError::NotFound); }
        Ok(())
    }
}
