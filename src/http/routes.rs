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
use crate::tenancy::TenantRegistry;

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

/// Router state — one [`TenantRegistry`] (per-tenant DBs) plus one
/// [`SystemRegistry`] (control-plane DB). Cheap to clone.
#[derive(Clone)]
pub struct AppState {
    pub system: SystemRegistry,
    pub tenants: TenantRegistry,
}

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
    // Probes: `/api/health` is state-free; `/api/live` and `/api/ready` want
    // a shared `Readiness` flag. We pass `Readiness` as a request Extension
    // so probe handlers don't need their own router-level state — this lets
    // us keep the whole outer router uniformly `Router<AppState>` and avoid
    // axum 0.8's state-type merge restrictions.

    // ------------------------------------------------------------- API routes
    tracing::debug!("build_router: configuring admin + tenant api routes");
    let admin_api = api_routes::admin_api();
    let tenant_api = api_routes::tenant_api();

    // --------------------------------------------- web UI (public + shells)
    //
    // Global public routes: landing, assets, tenant-less login/logout.
    // These do NOT live under `/web/{tenant}` because they have no tenant
    // context in the URL.
    let web_global = Router::new()
        .route("/",               get(wl::index))
        .route("/assets/{*path}", get(wa::serve))
        .route("/web/login",      get(wau::get_login).post(wau::post_login))
        .route("/web/logout",     post(wau::post_logout))
        .route("/portal",         get(wp::home))
        .route("/portal/login",   get(wp::get_login).post(wp::post_login))
        .route("/portal/register",get(wp::get_register).post(wp::post_register))
        .route("/portal/link-tenant", post(wp::post_link_tenant))
        .route("/portal/logout",  post(wp::post_logout));

    // Tenant-scoped staff shell — session-gated.
    //
    // Just like the JSON API, the `{tenant}` segment is factored out of every
    // individual route via `.nest("/web/{tenant}", …)`. Handlers keep using
    // the `TenantScope` extractor to get the resolved tenant + services —
    // no per-route path repetition, no separate middleware for tenant lookup.
    //
    // Every route here is behind `require_session`, which validates the
    // opaque `erp_session` cookie against the tenant's own DB, followed by
    // per-module `require_perm!(...)` gates that return a friendly 403 to
    // callers who lack the right RBAC codes. Nav/tile filtering already
    // hides them from the sidebar and dashboard; this middleware is the
    // defense-in-depth layer that catches manual URL edits.
    //
    use crate::services::perm;
    use crate::require_perm;
    let web_tenant_shell = Router::new()
        .route("/",              get(wdb::index))
        .route("/students",      get(ws::list)
            .route_layer(require_perm!(perm::STUDENTS_VIEW)))
        .route("/students/{id}", get(ws::show)
            .route_layer(require_perm!(perm::STUDENTS_VIEW)))
        .route("/staff",         get(wsf::list)
            .route_layer(require_perm!(perm::STAFF_VIEW)))
        .route("/staff/{id}",    get(wsf::show)
            .route_layer(require_perm!(perm::STAFF_VIEW)))
        // ---- Guardians (full CRUD) ---------------------------------------
        // Reads: guardians.view. Writes: guardians.manage.
        .route("/guardians",              get(wgd::list)
            .post(wgd::create)
            .route_layer(require_perm!(perm::GUARDIANS_VIEW, perm::GUARDIANS_MANAGE)))
        .route("/guardians/new",          get(wgd::new_form)
            .route_layer(require_perm!(perm::GUARDIANS_MANAGE)))
        .route("/guardians/{id}",         get(wgd::show)
            .post(wgd::update)
            .route_layer(require_perm!(perm::GUARDIANS_VIEW, perm::GUARDIANS_MANAGE)))
        .route("/guardians/{id}/edit",    get(wgd::edit_form)
            .route_layer(require_perm!(perm::GUARDIANS_MANAGE)))
        .route("/guardians/{id}/delete",  axum::routing::post(wgd::delete)
            .route_layer(require_perm!(perm::GUARDIANS_MANAGE)))
        // module stub screens (one route per placeholder module)
        .route("/attendance",    get(wm::attendance)
            .route_layer(require_perm!(perm::ATTENDANCE_VIEW, perm::ATTENDANCE_VIEW_OWN, perm::ATTENDANCE_MARK)))
        .route("/timetable",     get(wm::timetable)
            .route_layer(require_perm!(perm::TIMETABLE_VIEW, perm::TIMETABLE_MANAGE)))
        .route("/fees",          get(wm::fees)
            .route_layer(require_perm!(perm::FEES_VIEW, perm::FEES_COLLECT)))
        .route("/examinations",  get(wm::examinations)
            .route_layer(require_perm!(perm::EXAMINATIONS_VIEW, perm::EXAMINATIONS_ENTER_MARKS)))
        .route("/academic",      get(wm::academic)
            .route_layer(require_perm!(perm::ACADEMIC_VIEW)))
        .route("/payroll",       get(wm::payroll)
            .route_layer(require_perm!(perm::PAYROLL_VIEW, perm::PAYROLL_RUN)))
        .route("/communication", get(wm::communication)
            .route_layer(require_perm!(perm::COMMUNICATION_VIEW, perm::COMMUNICATION_BROADCAST)))
        .route("/library",       get(wm::library)
            .route_layer(require_perm!(perm::LIBRARY_VIEW)))
        .route("/transport",     get(wm::transport)
            .route_layer(require_perm!(perm::TRANSPORT_VIEW)))
        .route("/hostel",        get(wm::hostel)
            .route_layer(require_perm!(perm::HOSTEL_VIEW)))
        .route("/inventory",     get(wm::inventory)
            .route_layer(require_perm!(perm::INVENTORY_VIEW)))
        .route("/health",        get(wm::health)
            .route_layer(require_perm!(perm::HEALTH_VIEW)))
        .route("/discipline",    get(wm::discipline)
            .route_layer(require_perm!(perm::DISCIPLINE_VIEW)))
        .route("/documents",     get(wm::documents)
            .route_layer(require_perm!(perm::DOCUMENTS_VIEW)))
        .route("/audit",         get(wm::audit)
            .route_layer(require_perm!(perm::AUDIT_VIEW)))
        .route("/settings",      get(wm::settings)
            .route_layer(require_perm!(perm::SETTINGS_VIEW, perm::SETTINGS_MANAGE)))
        .layer(axum::middleware::from_fn(wam::require_staff_shell))
        .layer(axum::middleware::from_fn_with_state(
            state.clone(), wam::require_session,
        ));

    // Tenant-scoped web PUBLIC — the login form/POST for a tenant. Kept
    // as a separate router so `require_session` doesn't gate it (that would
    // cause an infinite redirect loop).
    let web_tenant_public = Router::new()
        .route("/login", get(wau::get_tenant_login).post(wau::post_login));

    // Combine both halves under a single `/web/{tenant}` nest.
    let web_tenant = Router::new()
        .merge(web_tenant_public)
        .merge(web_tenant_shell);

    // Tenant-scoped parent/student portal shell.
    let portal_tenant_public = Router::new()
        .route("/login", get(wau::get_portal_tenant_login).post(wau::post_portal_login));
    let portal_tenant_shell = Router::new()
        .route("/", get(wp::index))
        .route("/students", get(wp::students)
            .route_layer(require_perm!(perm::STUDENTS_VIEW_OWN)))
        .route("/students/{id}", get(wp::student_show)
            .route_layer(require_perm!(perm::STUDENTS_VIEW_OWN)))
        .layer(axum::middleware::from_fn(wam::require_portal_shell))
        .layer(axum::middleware::from_fn_with_state(
            state.clone(), wam::require_session,
        ));
    let portal_tenant = Router::new()
        .merge(portal_tenant_public)
        .merge(portal_tenant_shell);

    // ------------------------------------------- admin (control-plane) UI
    //
    // Intentionally unauthenticated for now (operator sign-in comes later).
    // Merged with the JSON API under a single `/admin` nest so we don't have
    // overlapping `.nest("/admin", …)` + `.nest("/admin/api", …)` claims
    // (which axum refuses because both would own `/admin/api/*`).
    let admin_web = Router::new()
        .route("/",                              get(wad::index))
        .route("/tenants",                       get(wad::list_tenants).post(wad::create_tenant))
        .route("/tenants/new",                   get(wad::new_tenant_form))
        .route("/tenants/{tid}/enable",          post(wad::enable_tenant))
        .route("/tenants/{tid}/disable",         post(wad::disable_tenant))
        .route("/tenants/{tid}/delete",          post(wad::delete_tenant))
        .route("/tenants/{tid}/rename",          post(wad::rename_tenant))
        .nest("/api", admin_api);

    // ------------------------------------------------------------ assemble
    tracing::debug!("build_router: finalizing assembly and adding middleware");
    let router = Router::new()
        .route("/api/health", get(|| async { "ok" }))
        .route("/api/live",   get(probe_live))
        .route("/api/ready",  get(probe_ready))
        .nest("/admin",         admin_web)
        .nest("/api/{tenant}",  tenant_api)
        .nest("/web/{tenant}",  web_tenant)
        .nest("/portal/{tenant}", portal_tenant)
        .merge(web_global)
        .with_state(state)
        .layer(axum::Extension(readiness))
        .layer(axum::middleware::from_fn(wtr::trace_request));

    // `NormalizePathLayer` is applied *outside* axum's routing so path
    // rewrite happens BEFORE the router matches (axum#3233).
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
