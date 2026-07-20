//! REST API layer.
//!
//! All routes are mounted under `/api/`:
//!
//! * `/api/admin/tenants/*` — control-plane, uses the **system DB** only.
//! * `/api/tenant/*`        — everything else. Requires the `x-tenant-id`
//!   header (or your preferred [`TenantSource`]). Constructs an
//!   [`AppServices`] against the tenant's DB.

use axum::{Router, routing::get};
use tower::ServiceBuilder;
use tower_http::{
    request_id::{PropagateRequestIdLayer, SetRequestIdLayer},
    trace::{DefaultOnResponse, TraceLayer},
};

use crate::http::middleware::{TenantScopeState, TenantSource, tenant_scope};
use crate::system::SystemRegistry;
use crate::tenancy::TenantRegistry;
use crate::tracing_layer::{make_span_with_ids, UuidRequestId, X_REQUEST_ID};

pub mod admin;
pub mod academic;
pub mod openapi;
pub mod attendance;
pub mod audit;
pub mod auth;
pub mod communication;
pub mod discipline;
pub mod documents;
pub mod enrollment;
pub mod examinations;
pub mod fees;
pub mod guardians;
pub mod health;
pub mod hostel;
pub mod inventory;
pub mod library;
pub mod payroll;
pub mod people;
pub mod timetable;
pub mod transport;

/// Everything the API layer needs, cloneable into request state.
#[derive(Clone)]
pub struct AppState {
    pub system: SystemRegistry,
    pub tenants: TenantRegistry,
}

/// Build the full application router (all routes live under `/api/`).
/// The `readiness` handle is exposed via `/api/live` and `/api/ready`; flip
/// it to `false` when shutdown starts.
pub fn build_router(
    state: AppState,
    source: TenantSource,
    readiness: crate::health_probes::Readiness,
) -> Router {
    // Tenant-scoped subtree.
    let tenant_state = TenantScopeState::new(state.tenants.clone()).with_source(source);
    let tenant_routes = Router::new()
        .nest("/auth",           auth::routes())
        .nest("/academic",       academic::routes())
        .nest("/people",         people::routes())
        .nest("/guardians",      guardians::routes())
        .nest("/enrollment",     enrollment::routes())
        .nest("/attendance",     attendance::routes())
        .nest("/timetable",      timetable::routes())
        .nest("/examinations",   examinations::routes())
        .nest("/fees",           fees::routes())
        .nest("/payroll",        payroll::routes())
        .nest("/library",        library::routes())
        .nest("/transport",      transport::routes())
        .nest("/hostel",         hostel::routes())
        .nest("/inventory",      inventory::routes())
        .nest("/communication",  communication::routes())
        .nest("/health",         health::routes())
        .nest("/discipline",     discipline::routes())
        .nest("/documents",      documents::routes())
        .nest("/audit",          audit::routes())
        .layer(axum::middleware::from_fn_with_state(tenant_state.clone(), tenant_scope))
        .with_state(tenant_state);

    // Tower stack applied to every request, in this order:
    //   1. Assign `x-request-id` if the client didn't send one.
    //   2. Open a tracing span carrying method / path / request_id / tenant_id.
    //   3. Propagate `x-request-id` back on the response.
    let observability = ServiceBuilder::new()
        .layer(SetRequestIdLayer::new(X_REQUEST_ID.clone(), UuidRequestId))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(make_span_with_ids)
                .on_response(DefaultOnResponse::new().level(tracing::Level::INFO)),
        )
        .layer(PropagateRequestIdLayer::new(X_REQUEST_ID.clone()));

    Router::new()
        .route("/api/health", get(|| async { "ok" }))
        .merge(crate::health_probes::router(readiness))
        .nest("/api/admin",   admin::routes(state.clone()))
        .nest("/api/tenant",  tenant_routes)
        .merge(openapi::swagger_router())
        .layer(observability)
}
