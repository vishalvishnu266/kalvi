use sqlx::SqlitePool;

use crate::entity::portal_user::{NewPortalUser, PortalUser};
use crate::exception::repo_error::{RepoError, RepoResult};

pub async fn create(pool: &SqlitePool, u: &NewPortalUser) -> RepoResult<PortalUser> {
    let id = sqlx::query_scalar::<_, i64>(
        r#"INSERT INTO portal_user (username, email, password_hash)
           VALUES (?, ?, ?) RETURNING id"#,
    )
    .bind(&u.username)
    .bind(&u.email)
    .bind(&u.password_hash)
    .fetch_one(pool)
    .await?;
    get(pool, id).await
}

pub async fn get(pool: &SqlitePool, id: i64) -> RepoResult<PortalUser> {
    sqlx::query_as::<_, PortalUser>("SELECT * FROM portal_user WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or(RepoError::NotFound)
}

pub async fn find_by_identifier(
    pool: &SqlitePool,
    identifier: &str,
) -> RepoResult<Option<PortalUser>> {
    Ok(sqlx::query_as::<_, PortalUser>(
        "SELECT * FROM portal_user WHERE username = ? OR email = ? LIMIT 1",
    )
    .bind(identifier)
    .bind(identifier)
    .fetch_optional(pool)
    .await?)
}
