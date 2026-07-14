use sqlx::{Sqlite, Executor};
use crate::model::tenant::Tenant;

pub struct TenantRepository;

impl TenantRepository {
    pub async fn find_by_slug(executor: impl Executor<'_, Database = Sqlite>, slug: &str) -> Result<Option<Tenant>, sqlx::Error> {
        sqlx::query_as::<_, Tenant>("SELECT * FROM tenants WHERE slug = ?")
            .bind(slug)
            .fetch_optional(executor)
            .await
    }

    pub async fn save(executor: impl Executor<'_, Database = Sqlite>, slug: &str, name: &str, db_name: &str) -> Result<Tenant, sqlx::Error> {
        let now = crate::util::id_util::current_timestamp();
        sqlx::query("INSERT INTO tenants (slug, name, database_name, created_at) VALUES (?, ?, ?, ?)")
            .bind(slug)
            .bind(name)
            .bind(db_name)
            .bind(now)
            .execute(executor)
            .await?;
            
        Ok(Tenant {
            id: 0, // In a real app, you might fetch the last inserted ID
            slug: slug.to_string(),
            name: name.to_string(),
            database_name: db_name.to_string(),
            created_at: now,
        })
    }
}
