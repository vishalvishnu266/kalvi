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
    admin as wad, assets as wa, auth as wau, dashboard as wdb, landing as wl,
    modules as wm, staff as wsf, students as ws,
};
use crate::middleware::auth as wam;
use crate::middleware::tracing as wtr;
pub use crate::middleware::tenant::TenantScope;

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
pub fn build_router(state: AppState, readiness: Readiness) -> Router {
    tracing::debug!("build_router: assembling application router");
    // Probes: `/api/health` is state-free; `/api/live` and `/api/ready` want
    // a shared `Readiness` flag. We pass `Readiness` as a request Extension
    // so probe handlers don't need their own router-level state — this lets
    // us keep the whole outer router uniformly `Router<AppState>` and avoid
    // axum 0.8's state-type merge restrictions.

    // ---------------------------------------------------------------- admin
    // Control-plane routes: operate on the system DB via `State<AppState>`.
    tracing::debug!("build_router: configuring admin routes");
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
    // Global public routes: landing, assets, tenant-less login/logout.
    // These do NOT live under `/web/{tenant}` because they have no tenant
    // context in the URL.
    let web_global = Router::new()
        .route("/",               get(wl::index))
        .route("/assets/{*path}", get(wa::serve))
        .route("/web/login",      get(wau::get_login).post(wau::post_login))
        .route("/web/logout",     post(wau::post_logout));

    // Tenant-scoped web shell — session-GATED half.
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
    // ## Future extension: parent / student portal
    //
    // If/when the parent-guardian or student self-service UX needs its own
    // shell (different sidebar, no top bar, mobile-first cards, etc.), add a
    // `let portal_shell = Router::new()...` block below with the same
    // `require_session` layer and nest it at `/web/{tenant}/portal`.
    // Handlers can be shared: the same `ws::list` handler works, only the
    // enclosing template differs. **The role never appears in the URL** —
    // RBAC still controls visibility and scoping.
    use crate::services::perm;
    use crate::require_perm;
    let web_tenant_shell = Router::new()
        .route("/",              get(wdb::index))
        .route("/students",      get(ws::list)
            .route_layer(require_perm!(perm::STUDENTS_VIEW, perm::STUDENTS_VIEW_OWN)))
        .route("/students/{id}", get(ws::show)
            .route_layer(require_perm!(perm::STUDENTS_VIEW, perm::STUDENTS_VIEW_OWN)))
        .route("/staff",         get(wsf::list)
            .route_layer(require_perm!(perm::STAFF_VIEW)))
        .route("/staff/{id}",    get(wsf::show)
            .route_layer(require_perm!(perm::STAFF_VIEW)))
        // module stub screens (one route per placeholder module)
        .route("/attendance",    get(wm::attendance)
            .route_layer(require_perm!(perm::ATTENDANCE_VIEW, perm::ATTENDANCE_VIEW_OWN, perm::ATTENDANCE_MARK)))
        .route("/timetable",     get(wm::timetable)
            .route_layer(require_perm!(perm::TIMETABLE_VIEW, perm::TIMETABLE_MANAGE)))
        .route("/fees",          get(wm::fees)
            .route_layer(require_perm!(perm::FEES_VIEW, perm::FEES_VIEW_OWN, perm::FEES_COLLECT, perm::FEES_PAY)))
        .route("/examinations",  get(wm::examinations)
            .route_layer(require_perm!(perm::EXAMINATIONS_VIEW, perm::EXAMINATIONS_VIEW_OWN, perm::EXAMINATIONS_ENTER_MARKS)))
        .route("/academic",      get(wm::academic)
            .route_layer(require_perm!(perm::ACADEMIC_VIEW)))
        .route("/payroll",       get(wm::payroll)
            .route_layer(require_perm!(perm::PAYROLL_VIEW, perm::PAYROLL_VIEW_OWN)))
        .route("/guardians",     get(wm::guardians)
            .route_layer(require_perm!(perm::GUARDIANS_VIEW)))
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
