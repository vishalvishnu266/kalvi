use axum::{
    routing::{get, post},
    Router,
    middleware as axum_middleware,
};
use crate::config::AppState;
use crate::controller::*;
use crate::middleware::{app_middleware, saas_middleware};

pub fn create_router(state: AppState) -> Router {
    // 1. SaaS Routes (Control Plane)
    let saas_routes = Router::new()
        .route("/onboard", get(saas_controller::show_onboard).post(saas_controller::process_onboard))
        .route("/login", get(saas_controller::show_login).post(saas_controller::process_login))
        .layer(axum_middleware::from_fn_with_state(state.clone(), saas_middleware));

    // 2. Tenant API Routes
    let tenant_api_routes = Router::new()
        .route("/health", get(api_controller::health));

    // 3. Tenant Web/Normal Routes
    let tenant_web_routes = Router::new()
        .route("/dashboard", get(dashboard_controller::show_dashboard))
        .route("/settings", get(settings_controller::show_settings).post(settings_controller::process_settings))
        .route("/logout", post(logout_controller::process_tenant_logout))
        .route("/login", get(login_controller::show_login).post(login_controller::process_login));

    // 4. Public and Root-level routes
    let public_routes = Router::new()
        .route("/", get(home_controller::show_home))
        .route("/login", get(login_controller::show_common_login).post(login_controller::process_common_login))
        .route("/logout", post(logout_controller::process_logout))
        .route("/registration", get(onboarding_controller::show_form).post(onboarding_controller::submit_form))
        .route("/pages/{*path}", get(|| async { axum::response::Html("<h1>Public Page Placeholder</h1>") }))
        .nest("/{slug}/api", tenant_api_routes)
        .nest("/{slug}", tenant_web_routes)
        .layer(axum_middleware::from_fn_with_state(state.clone(), app_middleware));

    // 5. Main Application Router
    Router::new()
        .nest("/saas", saas_routes)
        .merge(public_routes)
        .fallback(|| async { axum::response::Redirect::to("/") })
        .with_state(state)
}
