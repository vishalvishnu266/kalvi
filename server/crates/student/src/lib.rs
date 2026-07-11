//! Placeholder student crate.
//! Routes will be added here as student-management features are built out.

use axum::Router;
use shared::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
}
