use sqlx::SqlitePool;

use crate::entity::tenant::{NewTenant, Tenant, UpdateTenant};
use crate::exception::repo_error::RepoResult;
use crate::repository::tenant_repository;

pub async fn create(pool: &SqlitePool, t: &NewTenant) -> RepoResult<Tenant> {
    tenant_repository::create(pool, t).await
}

pub async fn get(pool: &SqlitePool, id: i64) -> RepoResult<Tenant> {
    tenant_repository::get(pool, id).await
}

pub async fn find_by_tenant_id(pool: &SqlitePool, tid: &str) -> RepoResult<Option<Tenant>> {
    tenant_repository::find_by_tenant_id(pool, tid).await
}

pub async fn list_all(pool: &SqlitePool) -> RepoResult<Vec<Tenant>> {
    tenant_repository::list_all(pool).await
}

pub async fn update(pool: &SqlitePool, id: i64, u: &UpdateTenant) -> RepoResult<Tenant> {
    tenant_repository::update(pool, id, u).await
}

pub async fn set_status(pool: &SqlitePool, id: i64, status: &str) -> RepoResult<()> {
    tenant_repository::update(
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

pub async fn soft_delete(pool: &SqlitePool, id: i64) -> RepoResult<()> {
    set_status(pool, id, "deleted").await
}
