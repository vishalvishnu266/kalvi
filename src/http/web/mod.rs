use crate::http::AppState;
use crate::web::{
    auth as wau, dashboard as wdb, guardians as wgd, modules as wm, staff as wsf, students as ws,
};
use axum::{
    routing::{get, post},
    Router,
};
mod middleware;

pub fn routes(state: AppState) -> Router<AppState> {
    let web_tenant_shell = Router::new()
        .route("/", get(wdb::index))
        .route("/students", get(ws::list))
        .route("/students/{id}", get(ws::show))
        .route("/staff", get(wsf::list))
        .route("/staff/{id}", get(wsf::show))
        .route("/guardians", get(wgd::list).post(wgd::create))
        .route("/guardians/new", get(wgd::new_form))
        .route("/guardians/{id}", get(wgd::show).post(wgd::update))
        .route("/guardians/{id}/edit", get(wgd::edit_form))
        .route("/guardians/{id}/delete", post(wgd::delete))
        .route("/attendance", get(wm::attendance))
        .route("/timetable", get(wm::timetable))
        .route("/fees", get(wm::fees))
        .route("/examinations", get(wm::examinations))
        .route("/academic", get(wm::academic))
        .route("/payroll", get(wm::payroll))
        .route("/communication", get(wm::communication))
        .route("/library", get(wm::library))
        .route("/transport", get(wm::transport))
        .route("/hostel", get(wm::hostel))
        .route("/inventory", get(wm::inventory))
        .route("/health", get(wm::health))
        .route("/discipline", get(wm::discipline))
        .route("/documents", get(wm::documents))
        .route("/audit", get(wm::audit))
        .route("/settings", get(wm::settings))
        .layer(axum::middleware::from_fn(middleware::require_staff_shell))
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            middleware::require_session,
        ));

    let web_tenant_public =
        Router::new().route("/login", get(wau::get_tenant_login).post(wau::post_login));

    Router::new()
        .merge(web_tenant_public)
        .merge(web_tenant_shell)
}

pub fn global_routes() -> Router<AppState> {
    Router::new()
        .route("/login", get(wau::get_login).post(wau::post_login))
        .route("/logout", post(wau::post_logout))
}
