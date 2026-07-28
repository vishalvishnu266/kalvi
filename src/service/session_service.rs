use sqlx::SqlitePool;

use crate::entity::session::{NewSession, Session};
use crate::exception::repo_error::RepoResult;
use crate::repository::session_repository;

pub async fn create(pool: &SqlitePool, s: &NewSession) -> RepoResult<Session> {
    session_repository::create(pool, s).await
}

pub async fn find_active_by_token(pool: &SqlitePool, token: &str) -> RepoResult<Option<Session>> {
    session_repository::find_active_by_token(pool, token).await
}

pub async fn touch(pool: &SqlitePool, id: i64) -> RepoResult<()> {
    session_repository::touch(pool, id).await
}

pub async fn revoke_by_token(pool: &SqlitePool, token: &str) -> RepoResult<()> {
    session_repository::revoke_by_token(pool, token).await
}

pub async fn revoke_all_for_user(pool: &SqlitePool, user_id: i64) -> RepoResult<()> {
    session_repository::revoke_all_for_user(pool, user_id).await
}

pub async fn cleanup(pool: &SqlitePool, keep_days: i64) -> RepoResult<u64> {
    session_repository::cleanup(pool, keep_days).await
}
