use sqlx::{Sqlite, Executor, SqlitePool};
use crate::model::Tenant;

pub struct TenantRepository;

impl TenantRepository {
    pub async fn find_by_slug<'a, E>(executor: E, slug: &str) -> Result<Option<Tenant>, sqlx::Error> 
    where E: Executor<'a, Database = Sqlite>
    {
        sqlx::query_as::<_, Tenant>("SELECT * FROM tenants WHERE slug = ?")
            .bind(slug)
            .fetch_optional(executor)
            .await
    }

    pub async fn save(executor: &mut sqlx::Transaction<'_, sqlx::Sqlite>, slug: &str, name: &str, db_name: &str) -> Result<Tenant, sqlx::Error> 
    {
        sqlx::query("INSERT INTO tenants (slug, name, database_name) VALUES (?, ?, ?)")
            .bind(slug)
            .bind(name)
            .bind(db_name)
            .execute(&mut **executor)
            .await?;
            
        Ok(Self::find_by_slug(&mut **executor, slug).await?.unwrap())
    }

    pub async fn update_settings<'a, E>(executor: E, slug: &str, primary_color: &str, dark_mode: bool) -> Result<(), sqlx::Error> 
    where E: Executor<'a, Database = Sqlite>
    {
        sqlx::query("UPDATE tenants SET primary_color = ?, dark_mode = ? WHERE slug = ?")
            .bind(primary_color)
            .bind(dark_mode)
            .bind(slug)
            .execute(executor)
            .await?;
        Ok(())
    }

    pub async fn list_all<'a, E>(executor: E) -> Result<Vec<Tenant>, sqlx::Error> 
    where E: Executor<'a, Database = Sqlite>
    {
        sqlx::query_as::<_, Tenant>("SELECT * FROM tenants")
            .fetch_all(executor)
            .await
    }
}
