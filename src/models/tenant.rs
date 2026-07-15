//! Master-DB `tenants` table model + lookup helpers.
//!
//! A tenant is one institution (a school or university). Each tenant has its
//! own SQLite database under `data/tenant/{id}.db`. This model lives in the
//! master DB and is the source of truth for tenant metadata.

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

use crate::errors::AppError;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Default)]
pub struct Tenant {
    pub id: String,               // slug (URL segment)
    pub name: String,             // display name
    pub institution_type: String, // "school" | "university"
    pub status: String,           // "active" | "suspended" | "archived"
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl Tenant {
    pub fn is_school(&self) -> bool {
        self.institution_type == "school"
    }
    pub fn is_university(&self) -> bool {
        self.institution_type == "university"
    }
    pub fn is_active(&self) -> bool {
        self.status == "active"
    }

    /// Fetch a single tenant by id (slug). Returns `None` if not found.
    pub async fn find(master: &SqlitePool, id: &str) -> Result<Option<Self>, AppError> {
        let row = sqlx::query_as::<_, Tenant>("SELECT * FROM tenants WHERE id = ?")
            .bind(id)
            .fetch_optional(master)
            .await?;
        Ok(row)
    }

    /// List all tenants ordered by name.
    pub async fn list_all(master: &SqlitePool) -> Result<Vec<Self>, AppError> {
        let rows = sqlx::query_as::<_, Tenant>("SELECT * FROM tenants ORDER BY name")
            .fetch_all(master)
            .await?;
        Ok(rows)
    }
}
