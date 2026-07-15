use axum::{routing::{get, post}, Router, middleware};
use axum::response::Response;
use crate::public_middleware;
use crate::csrf_middleware::csrf_middleware;
use crate::state::AppState;
use crate::tenant_db_middleware::tenant_db_middleware;
use crate::controllers::{dashboard_controller, student_controller};

pub fn create_routes(state: AppState) -> Router {
    // Public routes - minimal for now
    let public_router = Router::new()
        .route("/", get(|| async { "Welcome to Kalvi ERP. Please use your tenant portal." }))
        .layer(middleware::from_fn(public_middleware::public_middleware))
        .layer(middleware::map_response(add_security_headers));

    // Tenant specific routes under /web/{tenant_id}/
    let web_router = Router::new()
        .route("/{tenant_id}/dashboard", get(dashboard_controller::tenant_dashboard_handler))
        .route("/{tenant_id}/students", get(student_controller::list_students_handler))
        .route("/{tenant_id}/students/new", get(student_controller::new_student_handler))
        .route("/{tenant_id}/students/create", post(student_controller::create_student_handler))
        .route("/{tenant_id}/students/{student_id}/edit", get(student_controller::edit_student_handler))
        .route("/{tenant_id}/students/{student_id}/update", post(student_controller::update_student_handler))
        .route("/{tenant_id}/students/{student_id}/delete", post(student_controller::delete_student_handler))
        .layer(middleware::from_fn(csrf_middleware))
        .layer(middleware::from_fn_with_state(state.clone(), tenant_db_middleware));

    Router::new()
        .merge(public_router)
        .nest("/web", web_router)
        .with_state(state)
}

// A pure post-processing function
async fn add_security_headers(res: Response) -> Response {
    let mut res = res;
    res.headers_mut().insert("X-Content-Type-Options", "nosniff".parse().unwrap());
    res.headers_mut().insert("X-Frame-Options", "DENY".parse().unwrap());
    res
}
