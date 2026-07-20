//! `/api/tenant/hostel/*`

use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;

use crate::http::{ExtractServices, ServiceHttpError};
use crate::http::middleware::TenantScopeState;
use crate::repositories::hostel::{Hostel, HostelAllocation, HostelRoom};

pub fn routes() -> Router<TenantScopeState> {
    Router::new()
        .route("/hostels",                  get(list_hostels).post(create_hostel))
        .route("/hostels/{id}/rooms",        get(list_rooms).post(create_room))
        .route("/allocations",               post(allocate))
        .route("/allocations/transfer",      post(transfer))
        .route("/allocations/{sid}/vacate",  post(vacate))
        .route("/allocations/active/{sid}",  get(active_for))
}

async fn list_hostels(ExtractServices(a): ExtractServices)
    -> Result<Json<Vec<Hostel>>, ServiceHttpError>
{ Ok(Json(a.repos.hostels.list().await.map_err(re)?)) }

#[derive(Deserialize)] struct NewHostel { name: String, kind: Option<String> }

async fn create_hostel(ExtractServices(a): ExtractServices, Json(b): Json<NewHostel>)
    -> Result<Json<Hostel>, ServiceHttpError>
{ Ok(Json(a.repos.hostels.create(&b.name, b.kind.as_deref()).await.map_err(re)?)) }

async fn list_rooms(ExtractServices(a): ExtractServices, Path(id): Path<i64>)
    -> Result<Json<Vec<HostelRoom>>, ServiceHttpError>
{ Ok(Json(a.repos.hostel_rooms.rooms_in(id).await.map_err(re)?)) }

#[derive(Deserialize)] struct NewRoom { room_no: String, capacity: i64 }

async fn create_room(ExtractServices(a): ExtractServices, Path(id): Path<i64>, Json(b): Json<NewRoom>)
    -> Result<Json<HostelRoom>, ServiceHttpError>
{ Ok(Json(a.repos.hostel_rooms.create(id, &b.room_no, b.capacity).await.map_err(re)?)) }

#[derive(Deserialize)]
struct Allocate { student_id: i64, hostel_room_id: i64, from_date: chrono::NaiveDate }

async fn allocate(ExtractServices(a): ExtractServices, Json(b): Json<Allocate>)
    -> Result<Json<HostelAllocation>, ServiceHttpError>
{ Ok(Json(a.hostel.allocate(b.student_id, b.hostel_room_id, b.from_date).await?)) }

#[derive(Deserialize)]
struct Transfer { student_id: i64, to_room_id: i64, from_date: chrono::NaiveDate }

async fn transfer(ExtractServices(a): ExtractServices, Json(b): Json<Transfer>)
    -> Result<Json<HostelAllocation>, ServiceHttpError>
{ Ok(Json(a.hostel.transfer_room(b.student_id, b.to_room_id, b.from_date).await?)) }

#[derive(Deserialize)] struct Vacate { to_date: chrono::NaiveDate }

async fn vacate(ExtractServices(a): ExtractServices, Path(sid): Path<i64>, Json(b): Json<Vacate>)
    -> Result<axum::http::StatusCode, ServiceHttpError>
{ a.hostel.vacate(sid, b.to_date).await?; Ok(axum::http::StatusCode::NO_CONTENT) }

async fn active_for(ExtractServices(a): ExtractServices, Path(sid): Path<i64>)
    -> Result<Json<Option<HostelAllocation>>, ServiceHttpError>
{ Ok(Json(a.repos.hostel_allocations.active_for_student(sid).await.map_err(re)?)) }

fn re(e: crate::error::RepoError) -> ServiceHttpError { ServiceHttpError(e.into()) }
