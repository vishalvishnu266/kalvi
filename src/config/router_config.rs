//! Router assembly (Spring Boot's `@Configuration` equivalent for
//! MVC — this file wires every controller onto its URL path).

use axum::{
    routing::{get, post},
    Router,
};

use crate::application::AppState;
use crate::controller::{
    admin_rest_controller as admin_rest, admin_web_controller as admin_web,
    asset_controller as assets, auth_rest_controller as auth_rest,
    auth_web_controller as auth_web, dashboard_controller as dashboard,
    demo_rest_controller as demo_rest, demo_web_controller as demo_web,
    health_controller as health, landing_controller as landing,
    portal_tenant_controller as portal_tenant, portal_web_controller as portal_web,
};
use crate::infrastructure::readiness_probe::ReadinessProbe;
use crate::security::{
    auth_filter, portal_shell_filter, staff_shell_filter, tracing_filter,
};

pub fn build_router(state: AppState, readiness: ReadinessProbe) -> Router {
    tracing::debug!("router_config::build_router");

    let router = Router::new()
        // ── Health ──────────────────────────────────────────────
        .route("/api/health", get(health::health))
        .route("/api/live", get(health::live))
        .route("/api/ready", get(health::ready))
        // ── Admin ───────────────────────────────────────────────
        .nest("/admin", admin_routes())
        // ── Tenant JSON API ────────────────────────────────────
        .nest("/api/{tenant}", tenant_api_routes())
        // ── Tenant Web UI ──────────────────────────────────────
        .nest("/web/{tenant}", tenant_web_routes(state.clone()))
        // ── Tenant Portal UI ───────────────────────────────────
        .nest("/portal/{tenant}", tenant_portal_routes(state.clone()))
        // ── Global Web + Portal ────────────────────────────────
        .nest("/web", web_global_routes())
        .nest("/portal", portal_global_routes())
        // ── Landing + assets ───────────────────────────────────
        .route("/", get(landing::index))
        .route("/assets/{*path}", get(assets::serve))
        // ── State + layers ─────────────────────────────────────
        .with_state(state)
        .layer(axum::Extension(readiness))
        .layer(axum::middleware::from_fn(tracing_filter::trace_request));

    tracing::debug!("router_config::build_router complete");
    router
}

fn admin_routes() -> Router<AppState> {
    let admin_api = Router::new()
        .route("/tenants", get(admin_rest::list).post(admin_rest::create))
        .route(
            "/tenants/{tenant_id}",
            get(admin_rest::get_one)
                .put(admin_rest::update)
                .delete(admin_rest::soft_delete),
        )
        .route("/tenants/{tenant_id}/enable", post(admin_rest::enable))
        .route("/tenants/{tenant_id}/disable", post(admin_rest::disable));

    Router::new()
        .route("/", get(admin_web::index))
        .route(
            "/tenants",
            get(admin_web::list_tenants).post(admin_web::create_tenant),
        )
        .route("/tenants/new", get(admin_web::new_tenant_form))
        .route("/tenants/{tid}/enable", post(admin_web::enable_tenant))
        .route("/tenants/{tid}/disable", post(admin_web::disable_tenant))
        .route("/tenants/{tid}/delete", post(admin_web::delete_tenant))
        .route("/tenants/{tid}/rename", post(admin_web::rename_tenant))
        .nest("/api", admin_api)
}

fn tenant_api_routes() -> Router<AppState> {
    Router::new()
        .route("/auth/register", post(auth_rest::register))
        .route("/auth/login", post(auth_rest::login))
        .route("/auth/change-password", post(auth_rest::change_password))
        .route("/auth/whoami", get(auth_rest::whoami))
        .route("/demo/ping", get(demo_rest::ping))
        .route("/demo/messages", get(demo_rest::list))
        .route("/demo/echo", post(demo_rest::echo))
}

fn tenant_web_routes(state: AppState) -> Router<AppState> {
    let shell = Router::new()
        .route("/", get(dashboard::index))
        .route("/demo", get(demo_web::index))
        .layer(axum::middleware::from_fn(
            staff_shell_filter::require_staff_shell,
        ))
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            auth_filter::require_session,
        ));

    let public = Router::new().route(
        "/login",
        get(auth_web::get_tenant_login).post(auth_web::post_login),
    );

    Router::new().merge(public).merge(shell)
}

fn tenant_portal_routes(state: AppState) -> Router<AppState> {
    let public = Router::new().route(
        "/login",
        get(auth_web::get_portal_tenant_login).post(auth_web::post_portal_login),
    );

    let shell = Router::new()
        .route("/", get(portal_tenant::index))
        .layer(axum::middleware::from_fn(
            portal_shell_filter::require_portal_shell,
        ))
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            auth_filter::require_session,
        ));

    Router::new().merge(public).merge(shell)
}

fn web_global_routes() -> Router<AppState> {
    Router::new()
        .route("/login", get(auth_web::get_login).post(auth_web::post_login))
        .route("/logout", post(auth_web::post_logout))
}

fn portal_global_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(portal_web::home))
        .route(
            "/login",
            get(portal_web::get_login).post(portal_web::post_login),
        )
        .route(
            "/register",
            get(portal_web::get_register).post(portal_web::post_register),
        )
        .route("/link-tenant", post(portal_web::post_link_tenant))
        .route("/logout", post(portal_web::post_logout))
}
