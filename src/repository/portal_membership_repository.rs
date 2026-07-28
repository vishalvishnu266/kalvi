use sqlx::SqlitePool;

use crate::entity::portal_membership::{NewPortalMembership, PortalMembership};
use crate::exception::repo_error::{RepoError, RepoResult};

pub async fn add(pool: &SqlitePool, m: &NewPortalMembership) -> RepoResult<()> {
    if !matches!(m.role.as_str(), "guardian" | "student") {
        return Err(RepoError::validation("role must be guardian|student"));
    }
    sqlx::query(
        r#"INSERT OR IGNORE INTO portal_membership
           (portal_user_id, tenant_id, tenant_user_id, role)
           VALUES (?, ?, ?, ?)"#,
    )
    .bind(m.portal_user_id)
    .bind(&m.tenant_id)
    .bind(m.tenant_user_id)
    .bind(&m.role)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn list_for_user(
    pool: &SqlitePool,
    portal_user_id: i64,
) -> RepoResult<Vec<PortalMembership>> {
    Ok(sqlx::query_as::<_, PortalMembership>(
        r#"SELECT pm.* FROM portal_membership pm
           INNER JOIN tenant t ON t.tenant_id = pm.tenant_id
           WHERE pm.portal_user_id = ? AND t.status = 'active'
           ORDER BY pm.tenant_id, pm.id"#,
    )
    .bind(portal_user_id)
    .fetch_all(pool)
    .await?)
}
