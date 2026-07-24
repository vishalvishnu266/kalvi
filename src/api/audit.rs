use axum::{extract::{Path, Query}, Json};
use serde::Deserialize;

use crate::http::{ServiceHttpError, TenantScope};
use crate::repositories::audit::AuditEntry;
use crate::services::perm;

#[derive(Deserialize)] pub struct Limit { #[serde(default = "d100")] limit: i64 }
fn d100() -> i64 { 100 }

pub async fn for_entity(
    scope: TenantScope,
    Path((_t, entity, id)): Path<(String, String, i64)>,
    Query(q): Query<Limit>,
) -> Result<Json<Vec<AuditEntry>>, ServiceHttpError>
{
    scope.ctx.require(perm::AUDIT_VIEW)?;
    Ok(Json(scope.services.repos.audit_log.for_entity(&entity, id, q.limit).await?))
}

pub async fn for_user(
    scope: TenantScope,
    Path((_t, uid)): Path<(String, i64)>,
    Query(q): Query<Limit>,
) -> Result<Json<Vec<AuditEntry>>, ServiceHttpError>
{
    scope.ctx.require(perm::AUDIT_VIEW)?;
    Ok(Json(scope.services.repos.audit_log.for_user(uid, q.limit).await?))
}
