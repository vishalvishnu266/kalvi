use sqlx::SqlitePool;

use crate::entity::demo_message::DemoMessage;
use crate::exception::repo_error::{RepoError, RepoResult};

pub async fn list(pool: &SqlitePool, limit: i64) -> RepoResult<Vec<DemoMessage>> {
    Ok(sqlx::query_as::<_, DemoMessage>(
        "SELECT * FROM demo_message ORDER BY created_at DESC LIMIT ?",
    )
    .bind(limit)
    .fetch_all(pool)
    .await?)
}

pub async fn get(pool: &SqlitePool, id: i64) -> RepoResult<DemoMessage> {
    sqlx::query_as::<_, DemoMessage>("SELECT * FROM demo_message WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or(RepoError::NotFound)
}

pub async fn insert(
    pool: &SqlitePool,
    text: &str,
    created_by: Option<i64>,
) -> RepoResult<DemoMessage> {
    let id = sqlx::query_scalar::<_, i64>(
        r#"INSERT INTO demo_message (text, created_by)
           VALUES (?, ?) RETURNING id"#,
    )
    .bind(text)
    .bind(created_by)
    .fetch_one(pool)
    .await?;
    get(pool, id).await
}
