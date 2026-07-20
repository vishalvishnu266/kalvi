//! `/api/tenant/timetable/*`

use axum::{
    extract::Path,
    routing::{get, post, delete},
    Json, Router,
};

use crate::http::{ExtractServices, ServiceHttpError};
use crate::http::middleware::TenantScopeState;
use crate::repositories::timetable::{NewPeriod, NewSlot, Period, TimetableSlot};

pub fn routes() -> Router<TenantScopeState> {
    Router::new()
        .route("/periods",              get(list_periods).post(create_period))
        .route("/slots",                post(set_slot))
        .route("/slots/{id}",            delete(remove_slot))
        .route("/class/{id}",            get(class_grid))
        .route("/teacher/{id}",          get(teacher_grid))
        .route("/room/{id}",             get(room_grid))
}

async fn list_periods(ExtractServices(a): ExtractServices)
    -> Result<Json<Vec<Period>>, ServiceHttpError>
{ Ok(Json(a.repos.periods.list().await.map_err(|e| ServiceHttpError(e.into()))?)) }

async fn create_period(ExtractServices(a): ExtractServices, Json(b): Json<NewPeriod>)
    -> Result<Json<Period>, ServiceHttpError>
{ Ok(Json(a.repos.periods.create(&b).await.map_err(|e| ServiceHttpError(e.into()))?)) }

async fn set_slot(ExtractServices(a): ExtractServices, Json(b): Json<NewSlot>)
    -> Result<Json<TimetableSlot>, ServiceHttpError>
{ Ok(Json(a.timetable.set_slot(b).await?)) }

async fn remove_slot(ExtractServices(a): ExtractServices, Path(id): Path<i64>)
    -> Result<axum::http::StatusCode, ServiceHttpError>
{ a.timetable.remove(id).await?; Ok(axum::http::StatusCode::NO_CONTENT) }

async fn class_grid(ExtractServices(a): ExtractServices, Path(id): Path<i64>)
    -> Result<Json<Vec<TimetableSlot>>, ServiceHttpError>
{ Ok(Json(a.timetable.class_grid(id).await?)) }

async fn teacher_grid(ExtractServices(a): ExtractServices, Path(id): Path<i64>)
    -> Result<Json<Vec<TimetableSlot>>, ServiceHttpError>
{ Ok(Json(a.timetable.teacher_grid(id).await?)) }

async fn room_grid(ExtractServices(a): ExtractServices, Path(id): Path<i64>)
    -> Result<Json<Vec<TimetableSlot>>, ServiceHttpError>
{ Ok(Json(a.repos.timetable.for_room(id).await.map_err(|e| ServiceHttpError(e.into()))?)) }
