use sqlx::SqlitePool;

use crate::entity::user::{NewUser, User};
use crate::exception::repo_error::{RepoError, RepoResult};

pub async fn create(pool: &SqlitePool, u: &NewUser) -> RepoResult<User> {
    let id = sqlx::query_scalar::<_, i64>(
        r#"INSERT INTO user_account (username, email, password_hash, is_active)
           VALUES (?, ?, ?, ?) RETURNING id"#,
    )
    .bind(&u.username)
    .bind(&u.email)
    .bind(&u.password_hash)
    .bind(u.is_active as i64)
    .fetch_one(pool)
    .await?;
    get(pool, id).await
}

pub async fn get(pool: &SqlitePool, id: i64) -> RepoResult<User> {
    sqlx::query_as::<_, User>("SELECT * FROM user_account WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or(RepoError::NotFound)
}

pub async fn find_by_username(pool: &SqlitePool, username: &str) -> RepoResult<Option<User>> {
    Ok(
        sqlx::query_as::<_, User>("SELECT * FROM user_account WHERE username = ?")
            .bind(username)
            .fetch_optional(pool)
            .await?,
    )
}

pub async fn find_by_email(pool: &SqlitePool, email: &str) -> RepoResult<Option<User>> {
    Ok(
        sqlx::query_as::<_, User>("SELECT * FROM user_account WHERE email = ?")
            .bind(email)
            .fetch_optional(pool)
            .await?,
    )
}

pub async fn touch_last_login(pool: &SqlitePool, id: i64) -> RepoResult<()> {
    sqlx::query("UPDATE user_account SET last_login_at = datetime('now') WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn update_password_hash(pool: &SqlitePool, id: i64, h: &str) -> RepoResult<()> {
    sqlx::query("UPDATE user_account SET password_hash = ? WHERE id = ?")
        .bind(h)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
