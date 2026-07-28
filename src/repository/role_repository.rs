use sqlx::SqlitePool;

use crate::entity::role::Role;
use crate::exception::repo_error::RepoResult;

pub async fn find_by_name(pool: &SqlitePool, name: &str) -> RepoResult<Option<Role>> {
    Ok(sqlx::query_as::<_, Role>("SELECT * FROM role WHERE name = ?")
        .bind(name)
        .fetch_optional(pool)
        .await?)
}

pub async fn assign_to_user(pool: &SqlitePool, user_id: i64, role_id: i64) -> RepoResult<()> {
    sqlx::query("INSERT OR IGNORE INTO user_role (user_id, role_id) VALUES (?, ?)")
        .bind(user_id)
        .bind(role_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn find_by_user(pool: &SqlitePool, user_id: i64) -> RepoResult<Vec<Role>> {
    Ok(sqlx::query_as::<_, Role>(
        r#"SELECT r.* FROM role r
           INNER JOIN user_role ur ON ur.role_id = r.id
           WHERE ur.user_id = ? ORDER BY r.name"#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?)
}
