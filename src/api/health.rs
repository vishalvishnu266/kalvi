//! `/api/tenant/health/*`

use axum::{
    extract::Path,
    routing::get,
    Json, Router,
};
use serde::Deserialize;

use crate::http::{ExtractServices, ServiceHttpError};
use crate::http::middleware::TenantScopeState;
use crate::repositories::health::{ClinicVisit, HealthRecord, Vaccination};

pub fn routes() -> Router<TenantScopeState> {
    Router::new()
        .route("/records/{sid}",       get(get_record).post(upsert))
        .route("/records/{sid}/bmi",   get(bmi))
        .route("/vaccinations/{sid}",  get(list_vacc).post(add_vacc))
        .route("/visits/{sid}",        get(list_visits).post(add_visit))
}

async fn get_record(ExtractServices(a): ExtractServices, Path(sid): Path<i64>)
    -> Result<Json<Option<HealthRecord>>, ServiceHttpError>
{ Ok(Json(a.repos.health_records.for_student(sid).await.map_err(re)?)) }

#[derive(Deserialize)]
struct Vitals {
    height_cm: Option<f64>, weight_kg: Option<f64>,
    allergies: Option<String>, conditions: Option<String>,
}

async fn upsert(ExtractServices(a): ExtractServices, Path(sid): Path<i64>, Json(b): Json<Vitals>)
    -> Result<Json<HealthRecord>, ServiceHttpError>
{
    Ok(Json(a.health.upsert_vitals(sid, b.height_cm, b.weight_kg,
        b.allergies.as_deref(), b.conditions.as_deref()).await?))
}

async fn bmi(ExtractServices(a): ExtractServices, Path(sid): Path<i64>)
    -> Result<Json<serde_json::Value>, ServiceHttpError>
{
    Ok(Json(match a.health.bmi_for(sid).await? {
        Some(b) => serde_json::json!({ "bmi": b.bmi, "category": b.category }),
        None    => serde_json::json!({ "bmi": null }),
    }))
}

async fn list_vacc(ExtractServices(a): ExtractServices, Path(sid): Path<i64>)
    -> Result<Json<Vec<Vaccination>>, ServiceHttpError>
{ Ok(Json(a.repos.vaccinations.for_student(sid).await.map_err(re)?)) }

#[derive(Deserialize)]
struct AddVacc { vaccine_name: String, dose: Option<String>, given_on: chrono::NaiveDate }

async fn add_vacc(ExtractServices(a): ExtractServices, Path(sid): Path<i64>, Json(b): Json<AddVacc>)
    -> Result<Json<serde_json::Value>, ServiceHttpError>
{
    let id = a.health.record_vaccination(sid, &b.vaccine_name, b.dose.as_deref(), b.given_on).await?;
    Ok(Json(serde_json::json!({ "id": id })))
}

async fn list_visits(ExtractServices(a): ExtractServices, Path(sid): Path<i64>)
    -> Result<Json<Vec<ClinicVisit>>, ServiceHttpError>
{ Ok(Json(a.repos.clinic_visits.for_student(sid).await.map_err(re)?)) }

#[derive(Deserialize)]
struct AddVisit { complaint: Option<String>, treatment: Option<String>, attended_by_staff_id: Option<i64> }

async fn add_visit(ExtractServices(a): ExtractServices, Path(sid): Path<i64>, Json(b): Json<AddVisit>)
    -> Result<Json<serde_json::Value>, ServiceHttpError>
{
    let id = a.health.clinic_visit(sid, b.complaint.as_deref(), b.treatment.as_deref(), b.attended_by_staff_id).await?;
    Ok(Json(serde_json::json!({ "id": id })))
}

fn re(e: crate::error::RepoError) -> ServiceHttpError { ServiceHttpError(e.into()) }
