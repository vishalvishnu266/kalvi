//! Demo service — free functions over a tenant `SqlitePool`.
//!
//! This is the template every new module should follow:
//!   * All SQL is inlined here (no repository layer).
//!   * Read helpers take `&SqlitePool` and return `RepoResult<T>`.
//!   * Workflow helpers additionally take `&RequestCtx` for auth/audit
//!     and return `ServiceResult<T>` after calling `ctx.require(perm)`.

use sqlx::SqlitePool;

use crate::error::{RepoError, RepoResult};
use crate::models::demo::{DemoMessage, NewDemoMessage};
use crate::services::{perm, RequestCtx, ServiceResult};

pub async fn list_messages(pool: &SqlitePool, limit: i64) -> RepoResult<Vec<DemoMessage>> {
    Ok(sqlx::query_as::<_, DemoMessage>(
        "SELECT * FROM demo_message ORDER BY created_at DESC LIMIT ?",
    )
    .bind(limit)
    .fetch_all(pool)
    .await?)
}

pub async fn get_message(pool: &SqlitePool, id: i64) -> RepoResult<DemoMessage> {
    sqlx::query_as::<_, DemoMessage>("SELECT * FROM demo_message WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or(RepoError::NotFound)
}

/// High-level workflow: enforces permission, then inserts a row.
pub async fn create_message(
    pool: &SqlitePool,
    ctx: &RequestCtx,
    body: NewDemoMessage,
) -> ServiceResult<DemoMessage> {
    ctx.require(perm::DEMO_VIEW)?;

    let text = body.text.trim();
    if text.is_empty() {
        return Err(crate::services::ServiceError::validation(
            "text must not be empty",
        ));
    }

    let id = sqlx::query_scalar::<_, i64>(
        r#"INSERT INTO demo_message (text, created_by)
           VALUES (?, ?) RETURNING id"#,
    )
    .bind(text)
    .bind(ctx.user_id())
    .fetch_one(pool)
    .await?;

    Ok(get_message(pool, id).await?)
}
