use crate::http::AppState;
use crate::web::{auth as wau, dashboard as wdb, demo as wdm};
use axum::{
    routing::{get, post},
    Router,
};
mod middleware;

pub fn routes(state: AppState) -> Router<AppState> {
    // Business module screens have been stripped. Add one route per
    // module inside this router as you build them; the auth + shell
    // middleware layers below apply automatically.
    let web_tenant_shell = Router::new()
        .route("/", get(wdb::index))
        .route("/demo", get(wdm::index))
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
