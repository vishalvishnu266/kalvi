//! `/api/tenant/discipline/*`

use axum::{
    extract::{Path, Query},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;

use crate::http::{ExtractServices, ServiceHttpError};
use crate::http::middleware::TenantScopeState;
use crate::repositories::discipline::{DisciplineIncident, NewIncident};

pub fn routes() -> Router<TenantScopeState> {
    Router::new()
        .route("/",               post(report))
        .route("/student/{sid}",   get(history))
        .route("/between",        get(between))
}

#[derive(Deserialize)] struct ReportBody { #[serde(flatten)] incident: NewIncident, #[serde(default)] notify_guardians: bool }

async fn report(ExtractServices(a): ExtractServices, Json(b): Json<ReportBody>)
    -> Result<Json<DisciplineIncident>, ServiceHttpError>
{ Ok(Json(a.discipline.report(b.incident, b.notify_guardians).await?)) }

async fn history(ExtractServices(a): ExtractServices, Path(sid): Path<i64>)
    -> Result<Json<Vec<DisciplineIncident>>, ServiceHttpError>
{ Ok(Json(a.discipline.history(sid).await?)) }

#[derive(Deserialize)] struct Range { from: chrono::NaiveDate, to: chrono::NaiveDate }

async fn between(ExtractServices(a): ExtractServices, Query(q): Query<Range>)
    -> Result<Json<Vec<DisciplineIncident>>, ServiceHttpError>
{ Ok(Json(a.repos.discipline.between(q.from, q.to).await.map_err(|e| ServiceHttpError(e.into()))?)) }
