use sqlx::SqlitePool;

use crate::entity::permission::Permission;
use crate::exception::repo_error::RepoResult;

pub async fn find_by_user(pool: &SqlitePool, user_id: i64) -> RepoResult<Vec<Permission>> {
    Ok(sqlx::query_as::<_, Permission>(
        r#"SELECT DISTINCT p.* FROM permission p
           INNER JOIN role_permission rp ON rp.permission_id = p.id
           INNER JOIN user_role ur       ON ur.role_id = rp.role_id
           WHERE ur.user_id = ? ORDER BY p.code"#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?)
}
