//! Top-level application router assembly.
//!
//! Design rules for this module:
//!
//! * **Path-based tenancy.** The tenant id is always a URL path segment
//!   (`/api/{tenant}/…`, `/web/{tenant}/…`). No headers, no cookies, no
//!   subdomains are consulted to route to a tenant DB.
//! * **No middleware in the tenant pipeline.** Handlers get their tenant +
//!   [`AppServices`] via a single small extractor, [`TenantScope`], that
//!   validates the `{tenant}` path segment and looks the entry up in the
//!   [`TenantRegistry`]. That's it — no `tower` middleware, no request
//!   extensions plumbing.
//! * **Top-level wiring lives here; API URL wiring lives in**
//!   [`crate::http::api_routes`]. Handlers remain in their per-domain files
//!   (`src/api/*.rs`, `src/web/*.rs`).
//! * **Only one `tower` layer remains** — `NormalizePathLayer` — so `/x/`
//!   and `/x` both match (works around axum#3233).

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};

use crate::health_probes::Readiness;
use crate::services::ServiceError;
use crate::system::SystemRegistry;

use crate::http::api_routes;
use crate::web::{
    admin as wad, assets as wa, auth as wau, dashboard as wdb, guardians as wgd,
    landing as wl, modules as wm, portal as wp, staff as wsf, students as ws,
};
use crate::middleware::auth as wam;
use crate::middleware::tracing as wtr;
pub use crate::middleware::tenant::TenantScope;

// ============================================================================
// AppState + API error response mapping
// ============================================================================

// AppState moved to src/http/mod.rs

/// Backward-compatible alias used by API handler signatures.
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


// ============================================================================
// build_router — the single view of every URL in the app.
// ============================================================================

/// Assemble the whole application router.
///
/// Layout (top-down, all in one place):
///
/// ```text
///   /                              → web landing page
///   /assets/{*path}                → embedded static assets
///   /api/health                    → { "ok" }
///   /api/live | /api/ready         → k8s probes
///
///   /admin/api/tenants             → control-plane list / create
///   /admin/api/tenants/{id}        → get / update / delete
///   /admin/api/tenants/{id}/enable | /disable
///   /portal/login | /portal/register → global portal auth
///   /portal                        → cross-tenant portal hub
///
///   /api/{tenant}/…                → per-tenant JSON API (19 modules)
///
///   /web/login | /web/logout       → global sign-in
///   /web/{tenant}/login            → tenant-locked sign-in
///   /web/{tenant}/                 → dashboard launcher (session-gated)
///   /web/{tenant}/students[/{id}]  → students screen (session-gated)
///   /web/{tenant}/{module}         → placeholder stub screens (session-gated)
///
///   /portal/{tenant}/login         → tenant-locked portal sign-in
///   /portal/{tenant}/              → parent/student portal home
///   /portal/{tenant}/students      → parent/student student list
/// ```
///
/// Returns `NormalizePath<Router>` so trailing slashes are trimmed *before*
/// axum routes.
pub fn build_router(state: AppState, readiness: Readiness) -> Router {
    tracing::debug!("build_router: assembling application router");
    
    let tenant_api = api_routes::tenant_api();

    // Global public routes: landing, assets.
    let global = Router::new()
        .route("/",               get(wl::index))
        .route("/assets/{*path}", get(wa::serve));

    let web_global = crate::http::web::global_routes();
    let portal_global = crate::http::portal::global_routes();
    let admin = crate::http::admin::routes();
    let web_tenant = crate::http::web::routes(state.clone());
    let portal_tenant = crate::http::portal::routes(state.clone());

    // ------------------------------------------------------------ assemble
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

// ---------------- probe handlers (kept local to routes.rs) ----------------

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
