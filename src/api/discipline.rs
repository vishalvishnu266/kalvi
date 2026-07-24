use axum::{extract::{Path, Query}, Json};
use serde::Deserialize;

use crate::http::{ServiceHttpError, TenantScope};
use crate::repositories::discipline::{DisciplineIncident, NewIncident};
use crate::services::perm;

#[derive(Deserialize)]
pub struct ReportBody { #[serde(flatten)] incident: NewIncident, #[serde(default)] notify_guardians: bool }

pub async fn report(scope: TenantScope, Json(b): Json<ReportBody>)
    -> Result<Json<DisciplineIncident>, ServiceHttpError>
{
    let ctx = scope.ctx.clone();
    Ok(Json(scope.services.discipline.report(&ctx, b.incident, b.notify_guardians).await?))
}

pub async fn history(scope: TenantScope, Path((_t, sid)): Path<(String, i64)>)
    -> Result<Json<Vec<DisciplineIncident>>, ServiceHttpError>
{ Ok(Json(scope.services.discipline.history(&scope.ctx, sid).await?)) }

#[derive(Deserialize)] pub struct Range { from: chrono::NaiveDate, to: chrono::NaiveDate }

pub async fn between(scope: TenantScope, Query(q): Query<Range>)
    -> Result<Json<Vec<DisciplineIncident>>, ServiceHttpError>
{
    scope.ctx.require_any(&[perm::DISCIPLINE_VIEW, perm::DISCIPLINE_MANAGE])?;
    Ok(Json(scope.services.repos.discipline.between(q.from, q.to).await?))
}
