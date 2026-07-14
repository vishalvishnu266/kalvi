use axum::{
    routing::{get, post},
    Router,
    extract::State,
    middleware as axum_middleware,
};
use crate::config::AppState;
use crate::controller::*;
use crate::middleware::{tenant_middleware, auth_middleware, saas_middleware, security_middleware, request_id_middleware, csrf_middleware, admin_only_middleware, rate_limit_middleware::rate_limit_middleware};
use axum::middleware as ax_middleware;
use tower_http::cors::{Any, CorsLayer};
use http::Method;

pub fn create_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        // In real production, replace Any with specific origins
        .allow_origin(Any) 
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_headers(Any);

    // 1. SaaS Routes (Control Plane)
    let saas_routes = Router::new()
        .route("/onboard", get(saas_controller::show_onboard).post(saas_controller::process_onboard))
        .route("/login", get(saas_controller::show_login).post(saas_controller::process_login))
        .layer(axum_middleware::from_fn_with_state(state.clone(), rate_limit_middleware))
        .layer(axum_middleware::from_fn_with_state(state.clone(), saas_middleware));

    // 2. Tenant API Routes (Protected)
    let tenant_api_routes = Router::new()
        .route("/health", get(api_controller::health))
        .layer(axum_middleware::from_fn(auth_middleware));

    // 3. Tenant Web Routes
    // Public Web Routes (No Auth needed)
    let tenant_web_public = Router::new()
        .route("/login", get(login_controller::show_login).post(login_controller::process_login));

    // Protected Web Routes
    let tenant_web_protected = Router::new()
        .route("/dashboard", get(dashboard_controller::show_dashboard))
        .route("/settings", get(settings_controller::show_settings).post(settings_controller::process_settings)
            .layer(axum_middleware::from_fn(admin_only_middleware)))
        .route("/students/add", get(student_controller::show_add_form).post(student_controller::process_add))
        .route("/logout", post(logout_controller::process_tenant_logout))
        .layer(axum_middleware::from_fn(auth_middleware));

    // Combined Tenant Web
    let tenant_web_routes = Router::new()
        .merge(tenant_web_public)
        .merge(tenant_web_protected)
        .layer(axum_middleware::from_fn_with_state(state.clone(), rate_limit_middleware));

    // 4. Public and Root-level routes
    let public_routes = Router::new()
        .route("/health", get(|State(state): State<AppState>| async move {
            use axum::response::IntoResponse;
            let db_ok = state.db.check_health().await;
            let timestamp = crate::util::id_util::current_timestamp();
            if db_ok {
                let body = format!(r#"{{"status":"ok","timestamp":{}}}"#, timestamp);
                (axum::http::StatusCode::OK, [("content-type", "application/json")], body).into_response()
            } else {
                let body = r#"{"status":"error","message":"database unavailable"}"#;
                (axum::http::StatusCode::SERVICE_UNAVAILABLE, [("content-type", "application/json")], body).into_response()
            }
        }))
        .route("/", get(home_controller::show_home))
        .route("/contact", get(home_controller::show_contact))
        .route("/login", get(login_controller::show_common_login).post(login_controller::process_common_login))
        .route("/logout", post(logout_controller::process_logout))
        .route("/registration", get(onboarding_controller::show_form).post(onboarding_controller::submit_form))
        .layer(axum_middleware::from_fn_with_state(state.clone(), rate_limit_middleware));

    // 5. Tenant Routes (Data Plane)
    // We group these so we can apply the tenant_middleware once to both
    let tenant_routes = Router::new()
        .nest("/api/{slug}", tenant_api_routes)
        .nest("/web/{slug}", tenant_web_routes)
        .layer(axum_middleware::from_fn_with_state(state.clone(), tenant_middleware));

    // 6. Main Application Router
    Router::new()
        .nest("/saas", saas_routes)
        .merge(public_routes)
        .merge(tenant_routes)
        .fallback(|| async { axum::response::Redirect::to("/") })
        // GLOBAL PRODUCTION MIDDLEWARES
        .layer(ax_middleware::from_fn(security_middleware))
        .layer(ax_middleware::from_fn(csrf_middleware))
        .layer(ax_middleware::from_fn(request_id_middleware))
        .layer(cors)
        .with_state(state)
}
