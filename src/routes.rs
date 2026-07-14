use axum::{Router, middleware::{from_fn, Extension}, routing::{get, post}};
use crate::middleware::request_id_middleware;
use crate::controller::{saas_controller, onboarding_controller, login_controller, student_controller, dashboard_controller};
use crate::middleware::{request_id_middleware, tenant_middleware, auth_middleware};
use crate::config::AppState;

pub fn create_router(state: AppState) -> Router {
    // 1. SaaS Routes
    let saas_routes = Router::new()
        .route("/onboard", get(saas_controller::show_onboard).post(saas_controller::process_onboard))
        .route("/login", get(saas_controller::show_login).post(saas_controller::process_login));

    // 2. Tenant Public Routes
    let public_routes = Router::new()
        .route("/registration", get(onboarding_controller::show_registration).post(onboarding_controller::process_registration))
        .route("/login", get(login_controller::show_common_login).post(login_controller::process_common_login));

    // 3. Institutional (Isolated) Routes
    let tenant_routes = Router::new()
        .route("/login", get(login_controller::show_login).post(login_controller::process_login))
        .route("/dashboard", get(dashboard_controller::show_dashboard))
        .route("/students/add", get(student_controller::show_add_form).post(student_controller::process_add))
        .layer(from_fn(auth_middleware));

    Router::new()
        .nest("/saas", saas_routes)
        .merge(public_routes)
        .nest("/web/{tenant}", tenant_routes.layer(from_fn(tenant_middleware)))
        .layer(from_fn(request_id_middleware))
        .with_state(state)
}
