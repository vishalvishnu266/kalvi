use axum::{routing::{get}, Router, middleware};
use crate::controllers::test_controller;
use crate::state::AppState;
use crate::middleware::tenant_db_middleware;

pub fn create_routes(state: AppState) -> Router {
    let public_router = Router::new()
        .route("/", get(|| async { "Hello, World!" }));

    let web_router = Router::new()
        .route("/{tenant_id}/test", get(test_controller::test_handler))
        .layer(middleware::from_fn_with_state(state.clone(), tenant_db_middleware));

    Router::new()
        .merge(public_router)
        .nest("/web", web_router)
        .with_state(state)
}
