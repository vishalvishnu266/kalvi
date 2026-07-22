//! REST API layer.
//!
//! Route layout:
//!
//! * `/admin/api/tenants/*`     — control-plane, uses the **system DB** only.
//! * `/api/{tenant}/*`          — per-tenant business API. The tenant id
//!   travels as a path parameter (e.g. `/api/acme/people/students`), which
//!   is bookmarkable, log-friendly, and unambiguous. A per-tenant
//!   [`AppServices`] is built lazily against that tenant's DB.
//! * `/api/health`, `/api/live`, `/api/ready` — process health probes.
//!
//! The web UI is mounted separately by [`crate::web::build_web_router`]
//! under `/`, `/web/login`, `/web/{tenant}/…`.

use axum::{Router, routing::get};
use tower::{Layer, ServiceBuilder};
use tower_http::{
    normalize_path::{NormalizePath, NormalizePathLayer},
    request_id::{PropagateRequestIdLayer, SetRequestIdLayer},
    trace::{DefaultOnResponse, TraceLayer},
};

use crate::http::middleware::{TenantScopeState, TenantSource, tenant_scope};
use crate::system::SystemRegistry;
use crate::tenancy::TenantRegistry;
use crate::tracing_layer::{make_span_with_ids, UuidRequestId, X_REQUEST_ID};

pub mod admin;
pub mod academic;
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

/// Build the full application router.
///
/// Mounts:
/// * `/admin/api/*`      — tenant control-plane (system DB only)
/// * `/api/{tenant}/*`   — per-tenant business API (path-based tenant routing)
/// * `/api/health`, `/api/live`, `/api/ready` — health probes
/// * `/` and `/web/*`    — server-rendered web UI (login, dashboard, …)
///
/// Returns a `NormalizePath<Router>` service (not a bare `Router`) so that
/// trailing slashes in incoming request paths are stripped *before* axum's
/// router does path matching. This works around a well-known axum 0.8
/// nesting quirk where `nest("/x", inner)` with `inner.route("/", …)`
/// matches `/x` but 404s on `/x/` (see tokio-rs/axum#3233).
///
/// Callers pass the result to `axum::serve` via
/// `ServiceExt::<Request>::into_make_service` (see `main.rs`).
///
/// The `_source` parameter is retained for backwards compatibility. The
/// built-in tenant subtree always uses [`TenantSource::path_param("tenant")`]
/// internally, i.e. tenant DB routing is entirely path-based.
pub fn build_router(
    state: AppState,
    _source: TenantSource,
    readiness: crate::health_probes::Readiness,
) -> NormalizePath<Router> {
    // Tenant-scoped subtree. Tenant id comes from the `{tenant}` path
    // parameter captured by the `.nest("/api/{tenant}", ...)` below.
    let tenant_state = TenantScopeState::new(state.tenants.clone())
        .with_source(TenantSource::path_param("tenant"));
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

    // Observability stack, applied to every request, in outermost-first order:
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

    let router = Router::new()
        .route("/api/health", get(|| async { "ok" }))
        .merge(crate::health_probes::router(readiness))
        // Control-plane (system DB only) lives under /admin/api/*.
        .nest("/admin/api",   admin::routes(state.clone()))
        // Per-tenant business API lives under /api/{tenant}/*.
        .nest("/api/{tenant}",  tenant_routes)
        // Server-rendered web UI (Hotwire + Askama). Mounts:
        //   GET  /               → SaaS landing page (or redirect if signed in)
        //   GET  /web/login      → global login
        //   GET  /web/{tenant}/login → tenant-scoped login
        //   POST /web/login      → submit login
        //   POST /web/logout     → sign out
        //   /web/{tenant}/…      → authenticated app shell
        .merge(crate::web::build_web_router(state.tenants.clone()))
        .layer(observability);

    // Wrap the whole router in `NormalizePathLayer` — applied *outside*
    // axum's routing so the path rewrite (`/x/` → `/x`) happens BEFORE the
    // router matches. This works around axum 0.8's nesting quirk where a
    // nested router with `route("/", …)` matches `/x` but 404s on `/x/`
    // (see tokio-rs/axum#3233). Applying it via `Layer::layer` on the
    // router — instead of `Router::layer` — is the documented way to make
    // it actually affect routing.
    NormalizePathLayer::trim_trailing_slash().layer(router)
}
