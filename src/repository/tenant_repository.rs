use sqlx::SqlitePool;
use serde::Serialize;
use crate::util::errors::AppError;

use sqlx::Row;

#[derive(Debug, Clone, Serialize)]
pub struct Tenant {
    pub id: i64,
    pub slug: String,
    pub name: String,
    pub database_name: String,
    pub created_at: i64,
}

pub struct TenantRepository;

impl TenantRepository {
    pub async fn find_by_slug(pool: &SqlitePool, slug: &str) -> Result<Option<Tenant>, AppError> {
        let row = sqlx::query("SELECT id, slug, name, database_name, created_at FROM tenants WHERE slug = ?")
            .bind(slug)
            .fetch_optional(pool)
            .await
            .map_err(|e| AppError::RuntimeException(e.to_string()))?;

        match row {
            Some(row) => Ok(Some(Tenant {
                id: row.get("id"),
                slug: row.get("slug"),
                name: row.get("name"),
                database_name: row.get("database_name"),
                created_at: row.get("created_at"),
            })),
            None => Ok(None),
        }
    }
}
