use crate::http::{api_routes, AppState};
use crate::web::admin as wad;
use axum::{
    routing::{get, post},
    Router,
};

pub fn routes() -> Router<AppState> {
    let admin_api = api_routes::admin_api();

    Router::new()
        .route("/", get(wad::index))
        .route("/tenants", get(wad::list_tenants).post(wad::create_tenant))
        .route("/tenants/new", get(wad::new_tenant_form))
        .route("/tenants/{tid}/enable", post(wad::enable_tenant))
        .route("/tenants/{tid}/disable", post(wad::disable_tenant))
        .route("/tenants/{tid}/delete", post(wad::delete_tenant))
        .route("/tenants/{tid}/rename", post(wad::rename_tenant))
        .nest("/api", admin_api)
}
