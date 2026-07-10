use axum::{routing::get, Router};
use controller::{student_handler, student_json_handler};
use middleware::AppState;

pub fn student_router() -> Router<AppState> {
    Router::new()
        .route("/student/{id}", get(student_handler))
        .route("/api/student/{id}", get(student_json_handler))
}
