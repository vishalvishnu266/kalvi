use axum::{extract::Path, http::StatusCode, Json};
use serde::Deserialize;

use crate::http::{ServiceHttpError, TenantScope};
use crate::repositories::hostel::{Hostel, HostelAllocation, HostelRoom};
use crate::services::perm;

pub async fn list_hostels(scope: TenantScope)
    -> Result<Json<Vec<Hostel>>, ServiceHttpError>
{
    scope.ctx.require_any(&[perm::HOSTEL_VIEW, perm::HOSTEL_MANAGE])?;
    Ok(Json(scope.services.repos.hostels.list().await?))
}

#[derive(Deserialize)] pub struct NewHostel { name: String, kind: Option<String> }

pub async fn create_hostel(scope: TenantScope, Json(b): Json<NewHostel>)
    -> Result<Json<Hostel>, ServiceHttpError>
{
    scope.ctx.require(perm::HOSTEL_MANAGE)?;
    Ok(Json(scope.services.repos.hostels.create(&b.name, b.kind.as_deref()).await?))
}

pub async fn list_rooms(scope: TenantScope, Path((_t, id)): Path<(String, i64)>)
    -> Result<Json<Vec<HostelRoom>>, ServiceHttpError>
{
    scope.ctx.require_any(&[perm::HOSTEL_VIEW, perm::HOSTEL_MANAGE])?;
    Ok(Json(scope.services.repos.hostel_rooms.rooms_in(id).await?))
}

#[derive(Deserialize)] pub struct NewRoom { room_no: String, capacity: i64 }

pub async fn create_room(scope: TenantScope, Path((_t, id)): Path<(String, i64)>, Json(b): Json<NewRoom>)
    -> Result<Json<HostelRoom>, ServiceHttpError>
{
    scope.ctx.require(perm::HOSTEL_MANAGE)?;
    Ok(Json(scope.services.repos.hostel_rooms.create(id, &b.room_no, b.capacity).await?))
}

#[derive(Deserialize)]
pub struct Allocate { student_id: i64, hostel_room_id: i64, from_date: chrono::NaiveDate }

pub async fn allocate(scope: TenantScope, Json(b): Json<Allocate>)
    -> Result<Json<HostelAllocation>, ServiceHttpError>
{ Ok(Json(scope.services.hostel.allocate(&scope.ctx, b.student_id, b.hostel_room_id, b.from_date).await?)) }

#[derive(Deserialize)]
pub struct Transfer { student_id: i64, to_room_id: i64, from_date: chrono::NaiveDate }

pub async fn transfer(scope: TenantScope, Json(b): Json<Transfer>)
    -> Result<Json<HostelAllocation>, ServiceHttpError>
{ Ok(Json(scope.services.hostel.transfer_room(&scope.ctx, b.student_id, b.to_room_id, b.from_date).await?)) }

#[derive(Deserialize)] pub struct Vacate { to_date: chrono::NaiveDate }

pub async fn vacate(scope: TenantScope, Path((_t, sid)): Path<(String, i64)>, Json(b): Json<Vacate>)
    -> Result<StatusCode, ServiceHttpError>
{ scope.services.hostel.vacate(&scope.ctx, sid, b.to_date).await?; Ok(StatusCode::NO_CONTENT) }

pub async fn active_for(scope: TenantScope, Path((_t, sid)): Path<(String, i64)>)
    -> Result<Json<Option<HostelAllocation>>, ServiceHttpError>
{
    scope.ctx.require_any(&[perm::HOSTEL_VIEW, perm::HOSTEL_MANAGE])?;
    Ok(Json(scope.services.repos.hostel_allocations.active_for_student(sid).await?))
}
