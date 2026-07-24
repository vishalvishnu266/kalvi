use axum::{extract::Path, http::StatusCode, Json};

use crate::http::{ServiceHttpError, TenantScope};
use crate::repositories::timetable::{NewPeriod, NewSlot, Period, TimetableSlot};

pub async fn list_periods(scope: TenantScope)
    -> Result<Json<Vec<Period>>, ServiceHttpError>
{ Ok(Json(scope.services.repos.periods.list().await?)) }

pub async fn create_period(scope: TenantScope, Json(b): Json<NewPeriod>)
    -> Result<Json<Period>, ServiceHttpError>
{ Ok(Json(scope.services.repos.periods.create(&b).await?)) }

pub async fn set_slot(scope: TenantScope, Json(b): Json<NewSlot>)
    -> Result<Json<TimetableSlot>, ServiceHttpError>
{ Ok(Json(scope.services.timetable.set_slot(b).await?)) }

pub async fn remove_slot(scope: TenantScope, Path((_t, id)): Path<(String, i64)>)
    -> Result<StatusCode, ServiceHttpError>
{ scope.services.timetable.remove(id).await?; Ok(StatusCode::NO_CONTENT) }

pub async fn class_grid(scope: TenantScope, Path((_t, id)): Path<(String, i64)>)
    -> Result<Json<Vec<TimetableSlot>>, ServiceHttpError>
{ Ok(Json(scope.services.timetable.class_grid(id).await?)) }

pub async fn teacher_grid(scope: TenantScope, Path((_t, id)): Path<(String, i64)>)
    -> Result<Json<Vec<TimetableSlot>>, ServiceHttpError>
{ Ok(Json(scope.services.timetable.teacher_grid(id).await?)) }

pub async fn room_grid(scope: TenantScope, Path((_t, id)): Path<(String, i64)>)
    -> Result<Json<Vec<TimetableSlot>>, ServiceHttpError>
{ Ok(Json(scope.services.repos.timetable.for_room(id).await?)) }
