use axum::{extract::Path, Json};
use serde::Deserialize;

use crate::http::{ServiceHttpError, TenantScope};
use crate::repositories::health::{ClinicVisit, HealthRecord, Vaccination};
use crate::services::perm;

pub async fn get_record(scope: TenantScope, Path((_t, sid)): Path<(String, i64)>)
    -> Result<Json<Option<HealthRecord>>, ServiceHttpError>
{
    scope.ctx.require_any(&[perm::HEALTH_VIEW, perm::HEALTH_MANAGE])?;
    Ok(Json(scope.services.repos.health_records.for_student(sid).await?))
}

#[derive(Deserialize)]
pub struct Vitals {
    height_cm: Option<f64>, weight_kg: Option<f64>,
    allergies: Option<String>, conditions: Option<String>,
}

pub async fn upsert(scope: TenantScope, Path((_t, sid)): Path<(String, i64)>, Json(b): Json<Vitals>)
    -> Result<Json<HealthRecord>, ServiceHttpError>
{
    Ok(Json(scope.services.health.upsert_vitals(&scope.ctx, sid, b.height_cm, b.weight_kg,
        b.allergies.as_deref(), b.conditions.as_deref()).await?))
}

pub async fn bmi(scope: TenantScope, Path((_t, sid)): Path<(String, i64)>)
    -> Result<Json<serde_json::Value>, ServiceHttpError>
{
    Ok(Json(match scope.services.health.bmi_for(&scope.ctx, sid).await? {
        Some(b) => serde_json::json!({ "bmi": b.bmi, "category": b.category }),
        None    => serde_json::json!({ "bmi": null }),
    }))
}

pub async fn list_vacc(scope: TenantScope, Path((_t, sid)): Path<(String, i64)>)
    -> Result<Json<Vec<Vaccination>>, ServiceHttpError>
{
    scope.ctx.require_any(&[perm::HEALTH_VIEW, perm::HEALTH_MANAGE])?;
    Ok(Json(scope.services.repos.vaccinations.for_student(sid).await?))
}

#[derive(Deserialize)]
pub struct AddVacc { vaccine_name: String, dose: Option<String>, given_on: chrono::NaiveDate }

pub async fn add_vacc(scope: TenantScope, Path((_t, sid)): Path<(String, i64)>, Json(b): Json<AddVacc>)
    -> Result<Json<serde_json::Value>, ServiceHttpError>
{
    let id = scope.services.health.record_vaccination(&scope.ctx, sid, &b.vaccine_name, b.dose.as_deref(), b.given_on).await?;
    Ok(Json(serde_json::json!({ "id": id })))
}

pub async fn list_visits(scope: TenantScope, Path((_t, sid)): Path<(String, i64)>)
    -> Result<Json<Vec<ClinicVisit>>, ServiceHttpError>
{
    scope.ctx.require_any(&[perm::HEALTH_VIEW, perm::HEALTH_MANAGE])?;
    Ok(Json(scope.services.repos.clinic_visits.for_student(sid).await?))
}

#[derive(Deserialize)]
pub struct AddVisit { complaint: Option<String>, treatment: Option<String>, attended_by_staff_id: Option<i64> }

pub async fn add_visit(scope: TenantScope, Path((_t, sid)): Path<(String, i64)>, Json(b): Json<AddVisit>)
    -> Result<Json<serde_json::Value>, ServiceHttpError>
{
    let id = scope.services.health.clinic_visit(&scope.ctx, sid, b.complaint.as_deref(), b.treatment.as_deref(), b.attended_by_staff_id).await?;
    Ok(Json(serde_json::json!({ "id": id })))
}
