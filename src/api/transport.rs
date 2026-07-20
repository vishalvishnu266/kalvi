//! `/api/tenant/transport/*`

use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;

use crate::http::{ExtractServices, ServiceHttpError};
use crate::http::middleware::TenantScopeState;
use crate::repositories::transport::{NewStop, Route, RouteStop, StudentTransport, Vehicle};

pub fn routes() -> Router<TenantScopeState> {
    Router::new()
        .route("/vehicles",             get(list_vehicles).post(create_vehicle))
        .route("/routes",               get(list_routes).post(create_route))
        .route("/routes/{id}/stops",     get(list_stops).post(add_stop))
        .route("/assignments",           post(assign))
        .route("/assignments/{id}/end",  post(end_assignment))
        .route("/routes/{id}/students",  get(students_on_route))
}

async fn list_vehicles(ExtractServices(a): ExtractServices)
    -> Result<Json<Vec<Vehicle>>, ServiceHttpError>
{ Ok(Json(a.repos.vehicles.list().await.map_err(re)?)) }

async fn create_vehicle(ExtractServices(a): ExtractServices, Json(b): Json<Vehicle>)
    -> Result<Json<Vehicle>, ServiceHttpError>
{ Ok(Json(a.repos.vehicles.create(&b).await.map_err(re)?)) }

async fn list_routes(ExtractServices(a): ExtractServices)
    -> Result<Json<Vec<Route>>, ServiceHttpError>
{ Ok(Json(a.repos.routes.list().await.map_err(re)?)) }

#[derive(Deserialize)] struct NewRoute { name: String, vehicle_id: Option<i64> }

async fn create_route(ExtractServices(a): ExtractServices, Json(b): Json<NewRoute>)
    -> Result<Json<Route>, ServiceHttpError>
{ Ok(Json(a.repos.routes.create(&b.name, b.vehicle_id).await.map_err(re)?)) }

async fn list_stops(ExtractServices(a): ExtractServices, Path(id): Path<i64>)
    -> Result<Json<Vec<RouteStop>>, ServiceHttpError>
{ Ok(Json(a.repos.routes.stops(id).await.map_err(re)?)) }

async fn add_stop(ExtractServices(a): ExtractServices, Path(id): Path<i64>, Json(b): Json<NewStop>)
    -> Result<Json<RouteStop>, ServiceHttpError>
{ Ok(Json(a.repos.routes.add_stop(id, &b).await.map_err(re)?)) }

#[derive(Deserialize)]
struct Assign { student_id: i64, route_stop_id: i64, valid_from: chrono::NaiveDate }

async fn assign(ExtractServices(a): ExtractServices, Json(b): Json<Assign>)
    -> Result<Json<StudentTransport>, ServiceHttpError>
{ Ok(Json(a.transport.assign_to_stop(b.student_id, b.route_stop_id, b.valid_from).await?)) }

#[derive(Deserialize)] struct EndOn { on: chrono::NaiveDate }

async fn end_assignment(ExtractServices(a): ExtractServices, Path(id): Path<i64>, Json(b): Json<EndOn>)
    -> Result<axum::http::StatusCode, ServiceHttpError>
{ a.transport.end_assignment(id, b.on).await?; Ok(axum::http::StatusCode::NO_CONTENT) }

async fn students_on_route(ExtractServices(a): ExtractServices, Path(id): Path<i64>)
    -> Result<Json<Vec<StudentTransport>>, ServiceHttpError>
{ Ok(Json(a.repos.student_transport.students_on_route(id).await.map_err(re)?)) }

fn re(e: crate::error::RepoError) -> ServiceHttpError { ServiceHttpError(e.into()) }
