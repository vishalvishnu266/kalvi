use axum::{
    routing::{get, post},
    Router,
};
use crate::http::AppState;
use crate::web::{
    auth as wau, dashboard as wdb, guardians as wgd,
    modules as wm, staff as wsf, students as ws,
};
mod middleware;

use crate::services::perm;
use crate::require_perm;

pub fn routes(state: AppState) -> Router<AppState> {
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
        .route("/guardians/{id}/delete",  post(wgd::delete)
            .route_layer(require_perm!(perm::GUARDIANS_MANAGE)))

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
        .layer(axum::middleware::from_fn(middleware::require_staff_shell))
        .layer(axum::middleware::from_fn_with_state(
            state.clone(), middleware::require_session,
        ));

    let web_tenant_public = Router::new()
        .route("/login", get(wau::get_tenant_login).post(wau::post_login));

    Router::new()
        .merge(web_tenant_public)
        .merge(web_tenant_shell)
}

pub fn global_routes() -> Router<AppState> {
    Router::new()
        .route("/login",  get(wau::get_login).post(wau::post_login))
        .route("/logout", post(wau::post_logout))
}
