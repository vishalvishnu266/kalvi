use axum::{routing::{get}, Router, middleware};
use crate::controllers::test_controller;
use crate::state::AppState;
use crate::middleware::tenant_db_middleware;

pub fn create_routes(state: AppState) -> Router {
    let public_router = Router::new()
        .route("/", get(|| async { "Hello, World!" }));

    Router::new()
        .merge(public_router)
        .route("/web/{tenant_id}/test", get(test_controller::test_handler))
        .layer(middleware::from_fn_with_state(state.clone(), tenant_db_middleware))
        .with_state(state)
}
