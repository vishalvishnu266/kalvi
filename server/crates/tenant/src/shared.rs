use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use validator::Validate;
use once_cell::sync::Lazy;
use regex::Regex;

/// Core Tenant model
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Tenant {
    pub id: i64,
    pub slug: String,           // URL-friendly identifier (e.g., "greenwood-school")
    pub name: String,           // Full name (e.g., "Greenwood International School")
    pub contact_email: String,
    pub contact_phone: String,
    pub address: String,
    pub database_name: String,  // SQLite database filename
    pub is_active: bool,
    pub created_at: String,
}

/// Tenant onboarding form data
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct TenantOnboardingForm {
    #[validate(length(min = 3, max = 50, message = "Slug must be 3-50 characters"))]
    #[validate(regex(path = "SLUG_REGEX", message = "Slug can only contain lowercase letters, numbers, and hyphens"))]
    pub slug: String,
    
    #[validate(length(min = 3, max = 200, message = "Name must be 3-200 characters"))]
    pub name: String,
    
    #[validate(email(message = "Invalid email address"))]
    pub contact_email: String,
    
    #[validate(length(min = 10, max = 20, message = "Phone must be 10-20 characters"))]
    pub contact_phone: String,
    
    #[validate(length(min = 10, max = 500, message = "Address must be 10-500 characters"))]
    pub address: String,
}

static SLUG_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[a-z0-9]+(?:-[a-z0-9]+)*$").unwrap());

/// Database operations for tenants (stored in master database)
pub mod db {
    use super::*;
    
    /// Create tenants table in master database
    pub async fn create_tenant_table(pool: &SqlitePool) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS tenants (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                slug TEXT NOT NULL UNIQUE,
                name TEXT NOT NULL,
                contact_email TEXT NOT NULL,
                contact_phone TEXT NOT NULL,
                address TEXT NOT NULL,
                database_name TEXT NOT NULL UNIQUE,
                is_active BOOLEAN NOT NULL DEFAULT 1,
                created_at TEXT NOT NULL DEFAULT (datetime('now'))
            )
            "#,
        )
        .execute(pool)
        .await?;
        
        // Create index on slug for fast lookups
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_tenants_slug ON tenants(slug)")
            .execute(pool)
            .await?;
        
        Ok(())
    }
    
    /// Check if slug already exists
    pub async fn slug_exists(pool: &SqlitePool, slug: &str) -> Result<bool, sqlx::Error> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM tenants WHERE slug = ?")
            .bind(slug)
            .fetch_one(pool)
            .await?;
        Ok(count.0 > 0)
    }
    
    /// Create a new tenant
    pub async fn create_tenant(
        pool: &SqlitePool,
        form: &TenantOnboardingForm,
    ) -> Result<Tenant, sqlx::Error> {
        let database_name = format!("tenant_{}", form.slug);
        
        let result = sqlx::query(
            r#"
            INSERT INTO tenants (slug, name, contact_email, contact_phone, address, database_name)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&form.slug)
        .bind(&form.name)
        .bind(&form.contact_email)
        .bind(&form.contact_phone)
        .bind(&form.address)
        .bind(&database_name)
        .execute(pool)
        .await?;
        
        let tenant_id = result.last_insert_rowid();
        
        // Fetch the created tenant
        get_tenant_by_id(pool, tenant_id).await
            .and_then(|opt| opt.ok_or(sqlx::Error::RowNotFound))
    }
    
    /// Get tenant by ID
    pub async fn get_tenant_by_id(pool: &SqlitePool, id: i64) -> Result<Option<Tenant>, sqlx::Error> {
        sqlx::query_as::<_, Tenant>("SELECT * FROM tenants WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
    }
    
    /// Get tenant by slug
    pub async fn get_tenant_by_slug(pool: &SqlitePool, slug: &str) -> Result<Option<Tenant>, sqlx::Error> {
        sqlx::query_as::<_, Tenant>("SELECT * FROM tenants WHERE slug = ?")
            .bind(slug)
            .fetch_optional(pool)
            .await
    }
    
    /// List all active tenants
    pub async fn list_active_tenants(pool: &SqlitePool) -> Result<Vec<Tenant>, sqlx::Error> {
        sqlx::query_as::<_, Tenant>("SELECT * FROM tenants WHERE is_active = 1 ORDER BY name")
            .fetch_all(pool)
            .await
    }
}
