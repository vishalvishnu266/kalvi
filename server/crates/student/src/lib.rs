mod shared;
mod view_student;
mod edit_student_turbo;

use axum::Router;
use ::shared::middleware::AppState;

// Combine all student feature routes
pub fn routes() -> Router<AppState> {
    Router::new()
        .merge(view_student::routes())
        .merge(edit_student_turbo::routes())
}
