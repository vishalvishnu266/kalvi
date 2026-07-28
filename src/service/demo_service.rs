use sqlx::SqlitePool;

use crate::entity::demo_message::{DemoMessage, NewDemoMessage};
use crate::exception::repo_error::RepoResult;
use crate::exception::service_error::{ServiceError, ServiceResult};
use crate::repository::demo_message_repository;
use crate::security::permissions;
use crate::security::security_context::SecurityContext;

pub async fn list_messages(pool: &SqlitePool, limit: i64) -> RepoResult<Vec<DemoMessage>> {
    demo_message_repository::list(pool, limit).await
}

pub async fn get_message(pool: &SqlitePool, id: i64) -> RepoResult<DemoMessage> {
    demo_message_repository::get(pool, id).await
}

pub async fn create_message(
    pool: &SqlitePool,
    ctx: &SecurityContext,
    body: NewDemoMessage,
) -> ServiceResult<DemoMessage> {
    ctx.require(permissions::DEMO_VIEW)?;

    let text = body.text.trim();
    if text.is_empty() {
        return Err(ServiceError::validation("text must not be empty"));
    }

    Ok(demo_message_repository::insert(pool, text, ctx.user_id()).await?)
}
