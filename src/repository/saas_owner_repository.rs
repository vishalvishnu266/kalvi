use sqlx::{Sqlite, Executor};
use crate::model::SaasOwner;

pub struct SaasOwnerRepository;

impl SaasOwnerRepository {
    pub async fn find_by_username<'a, E>(executor: E, username: &str) -> Result<Option<SaasOwner>, sqlx::Error> 
    where E: Executor<'a, Database = Sqlite>
    {
        sqlx::query_as::<_, SaasOwner>("SELECT * FROM saas_owners WHERE username = ?")
            .bind(username)
            .fetch_optional(executor)
            .await
    }

    pub async fn save<'a, E>(executor: E, username: &str, password_hash: &str, full_name: &str) -> Result<(), sqlx::Error> 
    where E: Executor<'a, Database = Sqlite>
    {
        sqlx::query("INSERT INTO saas_owners (username, password_hash, full_name) VALUES (?, ?, ?)")
            .bind(username)
            .bind(password_hash)
            .bind(full_name)
            .execute(executor)
            .await?;
        Ok(())
    }

    pub async fn count<'a, E>(executor: E) -> Result<i64, sqlx::Error> 
    where E: Executor<'a, Database = Sqlite>
    {
        let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM saas_owners")
            .fetch_one(executor)
            .await?;
        Ok(count)
    }
}
