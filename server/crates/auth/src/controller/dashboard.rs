use axum::{
    extract::Extension,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use shared::{AppState, TenantContext};

use crate::extractors::RequireAuth;
use crate::view::dashboard::dashboard_page;

pub async fn show_dashboard(
    RequireAuth(user): RequireAuth,
    Extension(ctx): Extension<TenantContext>,
) -> Response {
    Html(dashboard_page(&ctx.slug, &user.username).into_string()).into_response()
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/t/{tenant_slug}/dashboard", get(show_dashboard))
}
