use sqlx::{Sqlite, Executor};
use crate::model::Tenant;

pub struct TenantRepository;

impl TenantRepository {
    pub async fn find_by_slug(executor: impl Executor<'_, Database = Sqlite>, slug: &str) -> Result<Option<Tenant>, sqlx::Error> {
        sqlx::query_as::<_, Tenant>("SELECT * FROM tenants WHERE slug = ?")
            .bind(slug)
            .fetch_optional(executor)
            .await
    }

    pub async fn save(executor: impl Executor<'_, Database = Sqlite>, slug: &str, name: &str, db_name: &str) -> Result<Tenant, sqlx::Error> {
        sqlx::query("INSERT INTO tenants (slug, name, database_name) VALUES (?, ?, ?)")
            .bind(slug)
            .bind(name)
            .bind(db_name)
            .execute(executor)
            .await?;
            
        Ok(Tenant {
            id: 0,
            slug: slug.to_string(),
            name: name.to_string(),
            database_name: db_name.to_string(),
            contact_email: None,
            contact_phone: None,
            address: None,
            created_at: None,
        })
    }

    pub async fn list_all(executor: impl Executor<'_, Database = Sqlite>) -> Result<Vec<Tenant>, sqlx::Error> {
        sqlx::query_as::<_, Tenant>("SELECT * FROM tenants")
            .fetch_all(executor)
            .await
    }
}
