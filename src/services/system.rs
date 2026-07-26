//! Free-function services against the master (system) database.
//!
//! `AppState` stores the system `SqlitePool` directly (no wrapper struct).
//! These functions replace the former `SystemRegistry` methods and are
//! called as `services::system::create_tenant(&state.system, &body)`.

use sqlx::SqlitePool;

use crate::error::{RepoError, RepoResult};
use crate::system::{
    NewPortalMembership, NewPortalUser, NewTenant, PortalMembership, PortalUser, Tenant,
    UpdateTenant,
};
use crate::tenancy::validate_tenant_id;

// ── Tenant registry ─────────────────────────────────────────────────────

pub async fn create_tenant(pool: &SqlitePool, t: &NewTenant) -> RepoResult<Tenant> {
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
    get_tenant(pool, id).await
}

pub async fn get_tenant(pool: &SqlitePool, id: i64) -> RepoResult<Tenant> {
    sqlx::query_as::<_, Tenant>("SELECT * FROM tenant WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or(RepoError::NotFound)
}

pub async fn find_tenant_by_tenant_id(pool: &SqlitePool, tid: &str) -> RepoResult<Option<Tenant>> {
    Ok(
        sqlx::query_as::<_, Tenant>("SELECT * FROM tenant WHERE tenant_id = ?")
            .bind(tid)
            .fetch_optional(pool)
            .await?,
    )
}

pub async fn list_tenants(pool: &SqlitePool) -> RepoResult<Vec<Tenant>> {
    Ok(sqlx::query_as::<_, Tenant>(
        "SELECT * FROM tenant WHERE status <> 'deleted' ORDER BY tenant_id",
    )
    .fetch_all(pool)
    .await?)
}

pub async fn update_tenant(pool: &SqlitePool, id: i64, u: &UpdateTenant) -> RepoResult<Tenant> {
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
    get_tenant(pool, id).await
}

pub async fn set_tenant_status(pool: &SqlitePool, id: i64, status: &str) -> RepoResult<()> {
    update_tenant(
        pool,
        id,
        &UpdateTenant {
            status: Some(status.into()),
            ..Default::default()
        },
    )
    .await
    .map(|_| ())
}

pub async fn soft_delete_tenant(pool: &SqlitePool, id: i64) -> RepoResult<()> {
    set_tenant_status(pool, id, "deleted").await
}

// ── Portal user registry ────────────────────────────────────────────────

pub async fn create_portal_user(pool: &SqlitePool, u: &NewPortalUser) -> RepoResult<PortalUser> {
    let id = sqlx::query_scalar::<_, i64>(
        r#"INSERT INTO portal_user (username, email, password_hash)
           VALUES (?, ?, ?) RETURNING id"#,
    )
    .bind(&u.username)
    .bind(&u.email)
    .bind(&u.password_hash)
    .fetch_one(pool)
    .await?;
    get_portal_user(pool, id).await
}

pub async fn get_portal_user(pool: &SqlitePool, id: i64) -> RepoResult<PortalUser> {
    sqlx::query_as::<_, PortalUser>("SELECT * FROM portal_user WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or(RepoError::NotFound)
}

pub async fn find_portal_user_by_identifier(
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

pub async fn add_portal_membership(pool: &SqlitePool, m: &NewPortalMembership) -> RepoResult<()> {
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

pub async fn list_portal_memberships(
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
