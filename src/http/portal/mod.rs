use crate::http::AppState;
use axum::{
    routing::{get, post},
    Router,
};

pub fn routes(state: AppState) -> Router<AppState> {
    let portal_tenant_public = Router::new();

    let portal_tenant_shell = Router::new();

    Router::new()
        .merge(portal_tenant_public)
        .merge(portal_tenant_shell)
}

pub fn global_routes() -> Router<AppState> {
    Router::new()
}
