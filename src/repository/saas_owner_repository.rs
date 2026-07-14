use sqlx::{Sqlite, Executor};
use crate::model::saas_owner::SaasOwner;

pub struct SaasOwnerRepository;

impl SaasOwnerRepository {
    pub async fn find_by_username(executor: impl Executor<'_, Database = Sqlite>, username: &str) -> Result<Option<SaasOwner>, sqlx::Error> {
        sqlx::query_as::<_, SaasOwner>("SELECT * FROM saas_owners WHERE username = ?")
            .bind(username)
            .fetch_optional(executor)
            .await
    }

    pub async fn has_any(executor: impl Executor<'_, Database = Sqlite>) -> Result<bool, sqlx::Error> {
        let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM saas_owners")
            .fetch_one(executor)
            .await?;
        Ok(count > 0)
    }

    pub async fn save(executor: impl Executor<'_, Database = Sqlite>, username: &str, password_hash: &str, full_name: &str) -> Result<(), sqlx::Error> {
        let now = crate::util::id_util::current_timestamp();
        sqlx::query("INSERT INTO saas_owners (username, password_hash, full_name, created_at) VALUES (?, ?, ?, ?)")
            .bind(username)
            .bind(password_hash)
            .bind(full_name)
            .bind(now)
            .execute(executor)
            .await?;
        Ok(())
    }
}
