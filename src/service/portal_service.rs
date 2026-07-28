use sqlx::SqlitePool;

use crate::entity::portal_membership::{NewPortalMembership, PortalMembership};
use crate::entity::portal_user::{NewPortalUser, PortalUser};
use crate::exception::repo_error::RepoResult;
use crate::repository::{portal_membership_repository, portal_user_repository};

pub async fn create_portal_user(pool: &SqlitePool, u: &NewPortalUser) -> RepoResult<PortalUser> {
    portal_user_repository::create(pool, u).await
}

pub async fn get_portal_user(pool: &SqlitePool, id: i64) -> RepoResult<PortalUser> {
    portal_user_repository::get(pool, id).await
}

pub async fn find_portal_user_by_identifier(
    pool: &SqlitePool,
    identifier: &str,
) -> RepoResult<Option<PortalUser>> {
    portal_user_repository::find_by_identifier(pool, identifier).await
}

pub async fn add_portal_membership(
    pool: &SqlitePool,
    m: &NewPortalMembership,
) -> RepoResult<()> {
    portal_membership_repository::add(pool, m).await
}

pub async fn list_portal_memberships(
    pool: &SqlitePool,
    portal_user_id: i64,
) -> RepoResult<Vec<PortalMembership>> {
    portal_membership_repository::list_for_user(pool, portal_user_id).await
}
