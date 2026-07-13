use sqlx::SqlitePool;
use crate::model::SaasOwner;

pub struct SaasOwnerRepository;

impl SaasOwnerRepository {
    pub async fn find_by_username(pool: &SqlitePool, username: &str) -> Result<Option<SaasOwner>, sqlx::Error> {
        sqlx::query_as::<_, SaasOwner>("SELECT * FROM saas_owners WHERE username = ?")
            .bind(username)
            .fetch_optional(pool)
            .await
    }

    pub async fn save(pool: &SqlitePool, username: &str, password_hash: &str, full_name: &str) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO saas_owners (username, password_hash, full_name) VALUES (?, ?, ?)")
            .bind(username)
            .bind(password_hash)
            .bind(full_name)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn count(pool: &SqlitePool) -> Result<i64, sqlx::Error> {
        let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM saas_owners")
            .fetch_one(pool)
            .await?;
        Ok(count)
    }
}
