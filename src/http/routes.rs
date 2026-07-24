use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use crate::AppState;
use crate::health_probes::Readiness;
use crate::services::ServiceError;

use crate::http::api_routes;
use crate::web::{assets as wa, landing as wl};
use crate::middleware::tracing as wtr;
pub use crate::middleware::tenant::TenantScope;

pub type ServiceHttpError = ServiceError;

impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        let (status, code) = match &self {
            ServiceError::NotFound        => (StatusCode::NOT_FOUND, "not_found"),
            ServiceError::Validation(_)   => (StatusCode::BAD_REQUEST, "validation_error"),
            ServiceError::Conflict(_)     => (StatusCode::CONFLICT, "conflict"),
            ServiceError::Unauthorized    => (StatusCode::UNAUTHORIZED, "unauthorized"),
            ServiceError::Forbidden(_)    => (StatusCode::FORBIDDEN, "forbidden"),
            ServiceError::Hash(_)         => (StatusCode::INTERNAL_SERVER_ERROR, "hash_error"),
            ServiceError::Repo(_)         => (StatusCode::INTERNAL_SERVER_ERROR, "repo_error"),
            ServiceError::Sqlx(_)         => (StatusCode::INTERNAL_SERVER_ERROR, "db_error"),
        };
        let body = Json(serde_json::json!({
            "error":   code,
            "message": self.to_string(),
        }));
        (status, body).into_response()
    }
}

pub fn build_router(state: AppState, readiness: Readiness) -> Router {
    tracing::debug!("build_router: assembling application router");

    let tenant_api = api_routes::tenant_api();

let global = Router::new()
        .route("/",               get(wl::index))
        .route("/assets/{*path}", get(wa::serve));

    let web_global = crate::http::web::global_routes();
    let portal_global = crate::http::portal::global_routes();
    let admin = crate::http::admin::routes();
    let web_tenant = crate::http::web::routes(state.clone());
    let portal_tenant = crate::http::portal::routes(state.clone());

tracing::debug!("build_router: finalizing assembly and adding middleware");
    let router = Router::new()
        .route("/api/health", get(|| async { "ok" }))
        .route("/api/live",   get(probe_live))
        .route("/api/ready",  get(probe_ready))
        .nest("/admin",         admin)
        .nest("/api/{tenant}",  tenant_api)
        .nest("/web/{tenant}",  web_tenant)
        .nest("/portal/{tenant}", portal_tenant)
        .nest("/web",           web_global)
        .nest("/portal",        portal_global)
        .merge(global)
        .with_state(state)
        .layer(axum::Extension(readiness))
        .layer(axum::middleware::from_fn(wtr::trace_request));

    tracing::debug!("build_router: router assembly complete");
    router
}

async fn probe_live() -> impl IntoResponse {
    (StatusCode::OK, Json(serde_json::json!({ "status": "alive" })))
}

async fn probe_ready(axum::Extension(r): axum::Extension<Readiness>) -> impl IntoResponse {
    if r.is_ready() {
        (StatusCode::OK, Json(serde_json::json!({ "status": "ready" })))
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, Json(serde_json::json!({ "status": "draining" })))
    }
}
