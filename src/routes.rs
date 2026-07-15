use askama::Template;
use axum::{
    middleware,
    response::{Html, IntoResponse, Redirect, Response},
    routing::{get, post},
    Router,
};

use crate::controllers::{
    academic_year_controller, dashboard_controller, settings_controller, student_controller,
};
use crate::csrf_middleware::csrf_middleware;
use crate::public_middleware;
use crate::state::AppState;
use crate::tenant_db_middleware::tenant_db_middleware;

#[derive(Template)]
#[template(path = "landing.html")]
struct LandingTpl;

pub fn create_routes(state: AppState) -> Router {
    // Public routes
    let public_router = Router::new()
        .route("/", get(landing_handler))
        .layer(middleware::from_fn(public_middleware::public_middleware))
        .layer(middleware::map_response(add_security_headers));

    // Tenant-scoped routes: /web/{tenant_id}/...
    let web_router = Router::new()
        .route("/{tenant_id}", get(|axum::extract::Path(t): axum::extract::Path<String>| async move {
            Redirect::to(&format!("/web/{}/dashboard", t))
        }))
        .route("/{tenant_id}/dashboard", get(dashboard_controller::tenant_dashboard_handler))
        .route("/{tenant_id}/students", get(student_controller::list_students_handler))
        .route("/{tenant_id}/students/new", get(student_controller::new_student_handler))
        .route("/{tenant_id}/students/create", post(student_controller::create_student_handler))
        .route("/{tenant_id}/students/{student_id}/edit", get(student_controller::edit_student_handler))
        .route("/{tenant_id}/students/{student_id}/update", post(student_controller::update_student_handler))
        .route("/{tenant_id}/students/{student_id}/delete", post(student_controller::delete_student_handler))

        // ---- Settings ----
        .route("/{tenant_id}/settings",         get(settings_controller::index_handler))
        .route("/{tenant_id}/settings/general", post(settings_controller::update_general_handler))

        // ---- Academic Years (settings sub-page) ----
        .route("/{tenant_id}/settings/academic-years",             get(academic_year_controller::list_handler))
        .route("/{tenant_id}/settings/academic-years/new",         get(academic_year_controller::new_handler))
        .route("/{tenant_id}/settings/academic-years/create",      post(academic_year_controller::create_handler))
        .route("/{tenant_id}/settings/academic-years/{id}/edit",   get(academic_year_controller::edit_handler))
        .route("/{tenant_id}/settings/academic-years/{id}/update", post(academic_year_controller::update_handler))
        .route("/{tenant_id}/settings/academic-years/{id}/delete", post(academic_year_controller::delete_handler))
        .route("/{tenant_id}/settings/academic-years/{id}/set-current",
               post(academic_year_controller::set_current_handler))

        .layer(middleware::from_fn(csrf_middleware))
        .layer(middleware::from_fn_with_state(state.clone(), tenant_db_middleware));

    Router::new()
        .merge(public_router)
        .nest("/web", web_router)
        .with_state(state)
}

async fn landing_handler() -> Response {
    match LandingTpl.render() {
        Ok(body) => Html(body).into_response(),
        Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "template error").into_response(),
    }
}

async fn add_security_headers(res: Response) -> Response {
    let mut res = res;
    res.headers_mut().insert("X-Content-Type-Options", "nosniff".parse().unwrap());
    res.headers_mut().insert("X-Frame-Options", "SAMEORIGIN".parse().unwrap());
    res
}
