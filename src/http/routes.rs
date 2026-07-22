//! **Every URL in the app is registered in this single file.**
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
//! * **Routing lives here; handlers live where they always did.** Every
//!   handler is a `pub async fn` in its per-domain file (`src/api/*.rs`,
//!   `src/web/*.rs`). This module only wires URL → handler.
//! * **Only one `tower` layer remains** — `NormalizePathLayer` — so `/x/`
//!   and `/x` both match (works around axum#3233).

use std::sync::Arc;

use axum::{
    extract::{FromRequestParts, Path},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    routing::{delete, get, post},
    Json, Router,
};
use serde::Deserialize;
use tower::Layer;
use tower_http::normalize_path::{NormalizePath, NormalizePathLayer};

use crate::health_probes::Readiness;
use crate::services::{Actor, AppServices, RequestCtx, ServiceError};
use crate::system::SystemRegistry;
use crate::tenancy::{TenantError, TenantId, TenantRegistry};

// Handler imports (one alias per per-domain module for tidy routing calls).
use crate::api::{
    academic as ac, admin as adm, attendance as at, audit as au, auth as ath,
    communication as cm, discipline as di, documents as dc, enrollment as en,
    examinations as ex, fees as fe, guardians as gd, health as hl, hostel as ho,
    inventory as iv, library as lb, payroll as pr, people as pp, timetable as tt,
    transport as tr,
};
use crate::web::{
    assets as wa, auth as wau, dashboard as wdb, landing as wl, modules as wm,
    students as ws,
};

// ============================================================================
// AppState + ServiceHttpError
// ============================================================================

/// Router state — one [`TenantRegistry`] (per-tenant DBs) plus one
/// [`SystemRegistry`] (control-plane DB). Cheap to clone.
#[derive(Clone)]
pub struct AppState {
    pub system: SystemRegistry,
    pub tenants: TenantRegistry,
}

/// Wraps [`ServiceError`] so it can be returned from handlers via `?` and
/// convert into an HTTP JSON error response.
pub struct ServiceHttpError(pub ServiceError);

impl From<ServiceError> for ServiceHttpError {
    fn from(e: ServiceError) -> Self { Self(e) }
}
impl From<crate::error::RepoError> for ServiceHttpError {
    fn from(e: crate::error::RepoError) -> Self { Self(ServiceError::from(e)) }
}

impl IntoResponse for ServiceHttpError {
    fn into_response(self) -> Response {
        let (status, code) = match &self.0 {
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
            "message": self.0.to_string(),
        }));
        (status, body).into_response()
    }
}

// ============================================================================
// TenantScope — the ONE extractor that replaces the old tenant middleware.
// ============================================================================

/// Everything a tenant-scoped handler needs, obtained in one extractor call:
/// the validated tenant id, the per-tenant [`AppServices`] bundle, and a
/// fresh [`RequestCtx`].
///
/// The extractor:
/// 1. Pulls the `{tenant}` path segment via `axum::extract::Path`.
/// 2. Validates it with [`TenantId::new`] (charset + length).
/// 3. Asks the [`TenantRegistry`] for the cached `AppServices` (or lazily
///    builds it on first use).
/// 4. Mints a fresh UUID request id and returns a [`RequestCtx`] with
///    `Actor::Anonymous`. Real auth can layer on top later.
///
/// There is **no middleware** on the tenant path. Any handler that wants
/// tenant-scoped state just declares `scope: TenantScope` in its signature.
pub struct TenantScope {
    pub tenant: TenantId,
    pub services: AppServices,
    pub ctx: RequestCtx,
}

#[derive(Deserialize)]
struct TenantPath { tenant: String }

impl FromRequestParts<AppState> for TenantScope {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let Path(TenantPath { tenant }) =
            Path::<TenantPath>::from_request_parts(parts, state)
                .await
                .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

        let tid = TenantId::new(tenant)
            .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

        let services = state
            .tenants
            .services_for(&tid)
            .await
            .map_err(tenant_error_to_http)?;

        let ctx = RequestCtx {
            tenant: tid.clone(),
            actor: Actor::Anonymous,
            request_id: uuid::Uuid::new_v4().to_string(),
            trace_id: None,
            permissions: Arc::new(Vec::new()),
            remote_ip: None,
        };

        Ok(Self { tenant: tid, services, ctx })
    }
}

fn tenant_error_to_http(e: TenantError) -> (StatusCode, String) {
    match e {
        TenantError::InvalidId(_) => (StatusCode::BAD_REQUEST, e.to_string()),
        TenantError::NotFound(_)  => (StatusCode::NOT_FOUND, e.to_string()),
        TenantError::Disabled(_)  => (StatusCode::FORBIDDEN, e.to_string()),
        TenantError::Repo(_)      => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
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
///
///   /api/{tenant}/…                → per-tenant JSON API (19 modules)
///
///   /web/login | /web/logout       → global sign-in
///   /web/{tenant}/login            → tenant-locked sign-in
///   /web/{tenant}/                 → dashboard launcher (session-gated)
///   /web/{tenant}/students[/{id}]  → students screen (session-gated)
///   /web/{tenant}/{module}         → placeholder stub screens (session-gated)
/// ```
///
/// Returns `NormalizePath<Router>` so trailing slashes are trimmed *before*
/// axum routes.
pub fn build_router(state: AppState, readiness: Readiness) -> NormalizePath<Router> {
    // Probes: `/api/health` is state-free; `/api/live` and `/api/ready` want
    // a shared `Readiness` flag. We pass `Readiness` as a request Extension
    // so probe handlers don't need their own router-level state — this lets
    // us keep the whole outer router uniformly `Router<AppState>` and avoid
    // axum 0.8's state-type merge restrictions.

    // ---------------------------------------------------------------- admin
    // Control-plane routes: operate on the system DB via `State<AppState>`.
    let admin_api = Router::new()
        .route("/tenants",                      get(adm::list).post(adm::create))
        .route("/tenants/{tenant_id}",          get(adm::get_one).put(adm::update).delete(adm::soft_delete))
        .route("/tenants/{tenant_id}/enable",   post(adm::enable))
        .route("/tenants/{tenant_id}/disable",  post(adm::disable));

    // -------------------------------------------------- per-tenant JSON API
    // Every handler declares `TenantScope` in its signature — the extractor
    // pulls the `{tenant}` path segment and resolves AppServices with zero
    // middleware. Routes are grouped by module for readability.
    let tenant_api = Router::new()
        // auth
        .route("/auth/register",         post(ath::register))
        .route("/auth/login",            post(ath::login))
        .route("/auth/change-password",  post(ath::change_password))
        .route("/auth/whoami",           get(ath::whoami))

        // academic
        .route("/academic/years",                          get(ac::list_years).post(ac::create_year))
        .route("/academic/years/current",                  get(ac::current_year))
        .route("/academic/years/{id}/activate",            post(ac::activate_year))
        .route("/academic/years/{id}/terms",               get(ac::list_terms).post(ac::create_term))
        .route("/academic/grades",                         get(ac::list_grades))
        .route("/academic/sections",                       get(ac::list_sections))
        .route("/academic/rooms",                          get(ac::list_rooms).post(ac::create_room))
        .route("/academic/subjects",                       get(ac::list_subjects).post(ac::create_subject))
        .route("/academic/class-sections",                 post(ac::create_class_section))
        .route("/academic/class-sections/{year_id}",       get(ac::list_class_sections))
        .route("/academic/class-sections/{id}/subjects",   get(ac::list_class_subjects).post(ac::assign_class_subject))

        // people (students + staff)
        .route("/people/students",                   get(pp::list_students))
        .route("/people/students/search",            get(pp::search_students))
        .route("/people/students/{id}",              get(pp::get_student).put(pp::update_student).delete(pp::delete_student))
        .route("/people/students/admit",             post(pp::admit))
        .route("/people/students/{id}/withdraw",     post(pp::withdraw))
        .route("/people/students/{id}/graduate",     post(pp::graduate))
        .route("/people/staff",                      get(pp::list_staff).post(pp::hire))
        .route("/people/staff/{id}",                 get(pp::get_staff).put(pp::update_staff))
        .route("/people/staff/{id}/terminate",       post(pp::terminate))

        // guardians
        .route("/guardians",                    get(gd::list).post(gd::create))
        .route("/guardians/{id}",               get(gd::get_one).delete(gd::remove))
        .route("/guardians/link",               post(gd::link))
        .route("/guardians/link/{sid}/{gid}",   delete(gd::unlink))
        .route("/guardians/of-student/{sid}",   get(gd::of_student))

        // enrollment
        .route("/enrollment",                              post(en::enroll))
        .route("/enrollment/transfer",                     post(en::transfer))
        .route("/enrollment/close-current",                post(en::close_current))
        .route("/enrollment/roster/{class_section_id}",    get(en::roster))
        .route("/enrollment/history/{student_id}",         get(en::history))
        .route("/enrollment/promote",                      post(en::promote_class))

        // attendance
        .route("/attendance/students/mark",         post(at::mark_one))
        .route("/attendance/students/mark-class",   post(at::mark_class))
        .route("/attendance/students/for/{sid}",    get(at::for_student))
        .route("/attendance/students/percentage",   get(at::percentage))
        .route("/attendance/students/class/{id}",   get(at::for_class_on))
        .route("/attendance/staff/mark",            post(at::mark_staff))
        .route("/attendance/staff/for/{sid}",       get(at::for_staff))

        // timetable
        .route("/timetable/periods",       get(tt::list_periods).post(tt::create_period))
        .route("/timetable/slots",         post(tt::set_slot))
        .route("/timetable/slots/{id}",    delete(tt::remove_slot))
        .route("/timetable/class/{id}",    get(tt::class_grid))
        .route("/timetable/teacher/{id}",  get(tt::teacher_grid))
        .route("/timetable/room/{id}",     get(tt::room_grid))

        // examinations
        .route("/examinations/grading-scales",              post(ex::create_scale))
        .route("/examinations/grading-scales/{id}/bands",   post(ex::add_band).get(ex::list_bands))
        .route("/examinations/exams",                       post(ex::create_exam))
        .route("/examinations/exams/term/{tid}",            get(ex::list_by_term))
        .route("/examinations/exams/{id}/schedules",        post(ex::schedule).get(ex::list_schedules))
        .route("/examinations/results",                     post(ex::enter_result))
        .route("/examinations/results/student/{sid}",       get(ex::for_student))
        .route("/examinations/report-cards/{sid}/{eid}",    get(ex::report_card))

        // fees
        .route("/fees/categories",              get(fe::list_categories).post(fe::create_category))
        .route("/fees/structures",              post(fe::create_structure))
        .route("/fees/structures/year/{yid}",   get(fe::list_structures))
        .route("/fees/structures/{id}/items",   post(fe::add_item).get(fe::list_items))
        .route("/fees/invoices/generate",       post(fe::generate_invoice))
        .route("/fees/invoices/student/{sid}",  get(fe::for_student))
        .route("/fees/invoices/{id}",           get(fe::get_invoice))
        .route("/fees/invoices/{id}/lines",     get(fe::get_lines))
        .route("/fees/invoices/{id}/cancel",    post(fe::cancel))
        .route("/fees/outstanding/{sid}",       get(fe::outstanding))
        .route("/fees/overdue",                 get(fe::overdue))
        .route("/fees/aging",                   get(fe::aging))
        .route("/fees/payments",                post(fe::record_payment))
        .route("/fees/payments/invoice/{id}",   get(fe::payments_for_invoice))
        .route("/fees/discounts",               post(fe::grant_discount))
        .route("/fees/discounts/student/{sid}", get(fe::discounts_for_student))
        .route("/fees/ledger/trial-balance",    get(fe::trial_balance))

        // payroll
        .route("/payroll/components",                    get(pr::list_components))
        .route("/payroll/structures/set",                post(pr::set_salary))
        .route("/payroll/payslips/generate",             post(pr::generate))
        .route("/payroll/payslips/month",                post(pr::generate_month))
        .route("/payroll/payslips/{id}/approve",         post(pr::approve))
        .route("/payroll/payslips/{id}/pay",             post(pr::pay))
        .route("/payroll/payslips/staff/{sid}/{year}",   get(pr::list_for_staff))

        // library
        .route("/library/books",               get(lb::list_books).post(lb::create_book))
        .route("/library/books/search",        get(lb::search))
        .route("/library/books/{id}",          get(lb::get_book))
        .route("/library/books/{id}/adjust",   post(lb::adjust))
        .route("/library/issues/student",      post(lb::issue_student))
        .route("/library/issues/staff",        post(lb::issue_staff))
        .route("/library/issues/{id}/return",  post(lb::return_book))
        .route("/library/issues/overdue",      get(lb::overdue))

        // transport
        .route("/transport/vehicles",              get(tr::list_vehicles).post(tr::create_vehicle))
        .route("/transport/routes",                get(tr::list_routes).post(tr::create_route))
        .route("/transport/routes/{id}/stops",     get(tr::list_stops).post(tr::add_stop))
        .route("/transport/routes/{id}/students",  get(tr::students_on_route))
        .route("/transport/assignments",           post(tr::assign))
        .route("/transport/assignments/{id}/end",  post(tr::end_assignment))

        // hostel
        .route("/hostel/hostels",                     get(ho::list_hostels).post(ho::create_hostel))
        .route("/hostel/hostels/{id}/rooms",          get(ho::list_rooms).post(ho::create_room))
        .route("/hostel/allocations",                 post(ho::allocate))
        .route("/hostel/allocations/transfer",        post(ho::transfer))
        .route("/hostel/allocations/{sid}/vacate",    post(ho::vacate))
        .route("/hostel/allocations/active/{sid}",    get(ho::active_for))

        // inventory
        .route("/inventory/vendors",                     get(iv::list_vendors).post(iv::create_vendor))
        .route("/inventory/items",                       get(iv::list_items).post(iv::create_item))
        .route("/inventory/items/low-stock",             get(iv::low_stock))
        .route("/inventory/items/{id}",                  get(iv::get_item))
        .route("/inventory/items/{id}/history",          get(iv::history))
        .route("/inventory/movements",                   post(iv::move_stock))
        .route("/inventory/purchase-orders",             post(iv::create_po))
        .route("/inventory/purchase-orders/{id}",        get(iv::get_po))
        .route("/inventory/purchase-orders/{id}/status", post(iv::set_po_status))

        // communication
        .route("/communication/announcements",                     get(cm::active).post(cm::broadcast))
        .route("/communication/announcements/class/{cid}",         get(cm::for_class))
        .route("/communication/announcements/{id}",                delete(cm::delete_ann))
        .route("/communication/messages",                          post(cm::send))
        .route("/communication/messages/inbox/{uid}",              get(cm::inbox))
        .route("/communication/messages/unread/{uid}",             get(cm::unread))
        .route("/communication/messages/{id}/read",                post(cm::mark_read_msg))
        .route("/communication/notifications",                     post(cm::notify))
        .route("/communication/notifications/user/{uid}",          get(cm::for_user))
        .route("/communication/notifications/{id}/read",           post(cm::mark_read))
        .route("/communication/notifications/user/{uid}/read-all", post(cm::read_all))

        // health (student clinic records)
        .route("/health/records/{sid}",      get(hl::get_record).post(hl::upsert))
        .route("/health/records/{sid}/bmi",  get(hl::bmi))
        .route("/health/vaccinations/{sid}", get(hl::list_vacc).post(hl::add_vacc))
        .route("/health/visits/{sid}",       get(hl::list_visits).post(hl::add_visit))

        // discipline
        .route("/discipline",                post(di::report))
        .route("/discipline/student/{sid}",  get(di::history))
        .route("/discipline/between",        get(di::between))

        // documents
        .route("/documents",                                post(dc::attach))
        .route("/documents/{id}",                           delete(dc::remove))
        .route("/documents/owner/{owner_type}/{owner_id}",  get(dc::for_owner))

        // audit
        .route("/audit/entity/{entity}/{id}", get(au::for_entity))
        .route("/audit/user/{uid}",           get(au::for_user));

    // --------------------------------------------- web UI (public + shell)
    //
    // Public routes: landing, assets, login/logout — no session cookie needed.
    let web_public = Router::new()
        .route("/",                    get(wl::index))
        .route("/assets/{*path}",      get(wa::serve))
        .route("/web/login",           get(wau::get_login).post(wau::post_login))
        .route("/web/logout",          post(wau::post_logout))
        .route("/web/{tenant}/login",  get(wau::get_tenant_login).post(wau::post_login));

    // Session-gated shell: dashboard + real screens + module stubs.
    let web_shell = Router::new()
        .route("/web/{tenant}/",              get(wdb::index))
        .route("/web/{tenant}/students",      get(ws::list))
        .route("/web/{tenant}/students/{id}", get(ws::show))
        // module stub screens (one route per placeholder module)
        .route("/web/{tenant}/attendance",    get(wm::attendance))
        .route("/web/{tenant}/timetable",     get(wm::timetable))
        .route("/web/{tenant}/fees",          get(wm::fees))
        .route("/web/{tenant}/examinations",  get(wm::examinations))
        .route("/web/{tenant}/academic",      get(wm::academic))
        .route("/web/{tenant}/staff",         get(wm::staff))
        .route("/web/{tenant}/payroll",       get(wm::payroll))
        .route("/web/{tenant}/guardians",     get(wm::guardians))
        .route("/web/{tenant}/communication", get(wm::communication))
        .route("/web/{tenant}/library",       get(wm::library))
        .route("/web/{tenant}/transport",     get(wm::transport))
        .route("/web/{tenant}/hostel",        get(wm::hostel))
        .route("/web/{tenant}/inventory",     get(wm::inventory))
        .route("/web/{tenant}/health",        get(wm::health))
        .route("/web/{tenant}/discipline",    get(wm::discipline))
        .route("/web/{tenant}/documents",     get(wm::documents))
        .route("/web/{tenant}/audit",         get(wm::audit))
        .route("/web/{tenant}/settings",      get(wm::settings))
        .layer(axum::middleware::from_fn(wau::require_session));

    // ------------------------------------------------------------ assemble
    let router = Router::new()
        .route("/api/health", get(|| async { "ok" }))
        .route("/api/live",   get(probe_live))
        .route("/api/ready",  get(probe_ready))
        .nest("/admin/api",    admin_api)
        .nest("/api/{tenant}", tenant_api)
        .merge(web_public)
        .merge(web_shell)
        .with_state(state)
        .layer(axum::Extension(readiness));

    // `NormalizePathLayer` is applied *outside* axum's routing so path
    // rewrite happens BEFORE the router matches (axum#3233).
    NormalizePathLayer::trim_trailing_slash().layer(router)
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
