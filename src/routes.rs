use axum::{
    middleware,
    routing::{get},
    Router,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::controllers::api::{dashboard_api_controller, student_api_controller};
use crate::models::student::{Student, StudentForm, StudentFilters};
use crate::controllers::api::dashboard_api_controller::DashboardStatsResponse;
use crate::state::AppState;
use crate::tenant_db_middleware::tenant_db_middleware;

#[derive(OpenApi)]
#[openapi(
    paths(
        dashboard_api_controller::get_dashboard_stats,
        student_api_controller::list_students,
        student_api_controller::get_student,
        student_api_controller::create_student,
        student_api_controller::update_student,
        student_api_controller::delete_student,
    ),
    components(
        schemas(Student, StudentForm, StudentFilters, DashboardStatsResponse)
    ),
    tags(
        (name = "Student ERP", description = "Student Management API")
    )
)]
struct ApiDoc;

pub fn create_routes(state: AppState) -> Router {
    // API routes: /api/{tenant_id}/...
    let api_router = Router::new()
        .route("/{tenant_id}/dashboard-stats", get(dashboard_api_controller::get_dashboard_stats))
        .route("/{tenant_id}/students", get(student_api_controller::list_students).post(student_api_controller::create_student))
        .route("/{tenant_id}/students/{student_id}", get(student_api_controller::get_student).put(student_api_controller::update_student).delete(student_api_controller::delete_student))
        .layer(middleware::from_fn_with_state(state.clone(), tenant_db_middleware));

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest("/api", api_router)
        .with_state(state)
}
