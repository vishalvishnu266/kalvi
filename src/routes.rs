use axum::{routing::{get}, Router, middleware};
use axum::response::Response;
use crate::controllers::test_controller;
use crate::public_middleware;
use crate::state::AppState;
use crate::tenant_db_middleware::tenant_db_middleware;
pub fn create_routes(state: AppState) -> Router {
    let public_router = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/storybook", get(crate::controllers::component_controller::storybook_handler))
        .layer(middleware::from_fn(public_middleware::public_middleware))
        .layer(middleware::map_response(add_security_headers));

    let web_router = Router::new()
        .route("/{tenant_id}/test", get(test_controller::test_handler))
        .layer(middleware::from_fn_with_state(state.clone(), tenant_db_middleware));

    Router::new()
        .merge(public_router)
        .nest("/web", web_router)
        .with_state(state)
}


// A pure post-processing function
async fn add_security_headers(res: Response) -> Response {
    let mut res = res;
    res.headers_mut().insert("test", "test".parse().unwrap());
    res
}
