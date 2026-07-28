use sqlx::SqlitePool;

use crate::entity::tenant::{NewTenant, Tenant, UpdateTenant};
use crate::exception::repo_error::{RepoError, RepoResult};
use crate::tenant::tenant_id::validate_tenant_id;

pub async fn create(pool: &SqlitePool, t: &NewTenant) -> RepoResult<Tenant> {
    let _ = validate_tenant_id(&t.tenant_id).map_err(|e| RepoError::validation(e.to_string()))?;

    let id = sqlx::query_scalar::<_, i64>(
        r#"INSERT INTO tenant (tenant_id, name, plan, notes)
           VALUES (?, ?, ?, ?) RETURNING id"#,
    )
    .bind(&t.tenant_id)
    .bind(&t.name)
    .bind(&t.plan)
    .bind(&t.notes)
    .fetch_one(pool)
    .await?;
    get(pool, id).await
}

pub async fn get(pool: &SqlitePool, id: i64) -> RepoResult<Tenant> {
    sqlx::query_as::<_, Tenant>("SELECT * FROM tenant WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or(RepoError::NotFound)
}

pub async fn find_by_tenant_id(pool: &SqlitePool, tid: &str) -> RepoResult<Option<Tenant>> {
    Ok(
        sqlx::query_as::<_, Tenant>("SELECT * FROM tenant WHERE tenant_id = ?")
            .bind(tid)
            .fetch_optional(pool)
            .await?,
    )
}

pub async fn list_all(pool: &SqlitePool) -> RepoResult<Vec<Tenant>> {
    Ok(sqlx::query_as::<_, Tenant>(
        "SELECT * FROM tenant WHERE status <> 'deleted' ORDER BY tenant_id",
    )
    .fetch_all(pool)
    .await?)
}

pub async fn update(pool: &SqlitePool, id: i64, u: &UpdateTenant) -> RepoResult<Tenant> {
    if let Some(s) = &u.status {
        if !matches!(s.as_str(), "active" | "disabled" | "deleted") {
            return Err(RepoError::validation(
                "status must be active|disabled|deleted",
            ));
        }
    }
    sqlx::query(
        r#"UPDATE tenant SET
             name       = COALESCE(?, name),
             plan       = COALESCE(?, plan),
             notes      = COALESCE(?, notes),
             status     = COALESCE(?, status),
             updated_at = datetime('now')
           WHERE id = ?"#,
    )
    .bind(&u.name)
    .bind(&u.plan)
    .bind(&u.notes)
    .bind(&u.status)
    .bind(id)
    .execute(pool)
    .await?;
    get(pool, id).await
}
