use crate::http::AppState;
use crate::web::{auth as wau, portal as wp};
use axum::{
    routing::{get, post},
    Router,
};
mod middleware;

pub fn routes(state: AppState) -> Router<AppState> {
    let portal_tenant_public = Router::new().route(
        "/login",
        get(wau::get_portal_tenant_login).post(wau::post_portal_login),
    );

    let portal_tenant_shell = Router::new()
        .route("/", get(wp::index))
        .route("/students", get(wp::students))
        .route("/students/{id}", get(wp::student_show))
        .layer(axum::middleware::from_fn(middleware::require_portal_shell))
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            middleware::require_session,
        ));

    Router::new()
        .merge(portal_tenant_public)
        .merge(portal_tenant_shell)
}

pub fn global_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(wp::home))
        .route("/login", get(wp::get_login).post(wp::post_login))
        .route("/register", get(wp::get_register).post(wp::post_register))
        .route("/link-tenant", post(wp::post_link_tenant))
        .route("/logout", post(wp::post_logout))
}
