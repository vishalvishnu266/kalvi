use crate::http::AppState;
use axum::{
    routing::{get, post},
    Router,
};

pub fn routes(state: AppState) -> Router<AppState> {
    // Business module screens have been stripped. Add one route per
    // module inside this router as you build them; the auth + shell
    // middleware layers below apply automatically.
    let web_tenant_shell = Router::new();

    let web_tenant_public =
        Router::new();

    Router::new()
        .merge(web_tenant_public)
        .merge(web_tenant_shell)
}

pub fn global_routes() -> Router<AppState> {
    Router::new()
}
