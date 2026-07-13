use sqlx::SqlitePool;
use crate::model::Tenant;

pub struct TenantRepository;

impl TenantRepository {
    pub async fn find_by_slug(pool: &SqlitePool, slug: &str) -> Result<Option<Tenant>, sqlx::Error> {
        sqlx::query_as::<_, Tenant>("SELECT * FROM tenants WHERE slug = ?")
            .bind(slug)
            .fetch_optional(pool)
            .await
    }

    pub async fn save(pool: &SqlitePool, slug: &str, name: &str, db_name: &str) -> Result<Tenant, sqlx::Error> {
        sqlx::query("INSERT INTO tenants (slug, name, database_name) VALUES (?, ?, ?)")
            .bind(slug)
            .bind(name)
            .bind(db_name)
            .execute(pool)
            .await?;
            
        Self::find_by_slug(pool, slug).await.map(|t| t.unwrap())
    }

    pub async fn update_settings(pool: &SqlitePool, slug: &str, primary_color: &str, dark_mode: bool) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE tenants SET primary_color = ?, dark_mode = ? WHERE slug = ?")
            .bind(primary_color)
            .bind(dark_mode)
            .bind(slug)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn list_all(pool: &SqlitePool) -> Result<Vec<Tenant>, sqlx::Error> {
        sqlx::query_as::<_, Tenant>("SELECT * FROM tenants")
            .fetch_all(pool)
            .await
    }
}
