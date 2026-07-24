use axum::{extract::Path, http::StatusCode, Json};
use serde::Deserialize;

use crate::http::{ServiceHttpError, TenantScope};
use crate::repositories::transport::{NewStop, Route, RouteStop, StudentTransport, Vehicle};
use crate::services::perm;

pub async fn list_vehicles(scope: TenantScope)
    -> Result<Json<Vec<Vehicle>>, ServiceHttpError>
{
    scope.ctx.require_any(&[perm::TRANSPORT_VIEW, perm::TRANSPORT_MANAGE])?;
    Ok(Json(scope.services.repos.vehicles.list().await?))
}

pub async fn create_vehicle(scope: TenantScope, Json(b): Json<Vehicle>)
    -> Result<Json<Vehicle>, ServiceHttpError>
{
    scope.ctx.require(perm::TRANSPORT_MANAGE)?;
    Ok(Json(scope.services.repos.vehicles.create(&b).await?))
}

pub async fn list_routes(scope: TenantScope)
    -> Result<Json<Vec<Route>>, ServiceHttpError>
{
    scope.ctx.require_any(&[perm::TRANSPORT_VIEW, perm::TRANSPORT_MANAGE])?;
    Ok(Json(scope.services.repos.routes.list().await?))
}

#[derive(Deserialize)] pub struct NewRoute { name: String, vehicle_id: Option<i64> }

pub async fn create_route(scope: TenantScope, Json(b): Json<NewRoute>)
    -> Result<Json<Route>, ServiceHttpError>
{
    scope.ctx.require(perm::TRANSPORT_MANAGE)?;
    Ok(Json(scope.services.repos.routes.create(&b.name, b.vehicle_id).await?))
}

pub async fn list_stops(scope: TenantScope, Path((_t, id)): Path<(String, i64)>)
    -> Result<Json<Vec<RouteStop>>, ServiceHttpError>
{
    scope.ctx.require_any(&[perm::TRANSPORT_VIEW, perm::TRANSPORT_MANAGE])?;
    Ok(Json(scope.services.repos.routes.stops(id).await?))
}

pub async fn add_stop(scope: TenantScope, Path((_t, id)): Path<(String, i64)>, Json(b): Json<NewStop>)
    -> Result<Json<RouteStop>, ServiceHttpError>
{
    scope.ctx.require(perm::TRANSPORT_MANAGE)?;
    Ok(Json(scope.services.repos.routes.add_stop(id, &b).await?))
}

#[derive(Deserialize)]
pub struct Assign { student_id: i64, route_stop_id: i64, valid_from: chrono::NaiveDate }

pub async fn assign(scope: TenantScope, Json(b): Json<Assign>)
    -> Result<Json<StudentTransport>, ServiceHttpError>
{ Ok(Json(scope.services.transport.assign_to_stop(&scope.ctx, b.student_id, b.route_stop_id, b.valid_from).await?)) }

#[derive(Deserialize)] pub struct EndOn { on: chrono::NaiveDate }

pub async fn end_assignment(scope: TenantScope, Path((_t, id)): Path<(String, i64)>, Json(b): Json<EndOn>)
    -> Result<StatusCode, ServiceHttpError>
{ scope.services.transport.end_assignment(&scope.ctx, id, b.on).await?; Ok(StatusCode::NO_CONTENT) }

pub async fn students_on_route(scope: TenantScope, Path((_t, id)): Path<(String, i64)>)
    -> Result<Json<Vec<StudentTransport>>, ServiceHttpError>
{
    scope.ctx.require_any(&[perm::TRANSPORT_VIEW, perm::TRANSPORT_MANAGE])?;
    Ok(Json(scope.services.repos.student_transport.students_on_route(id).await?))
}
