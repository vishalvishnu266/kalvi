use axum::{
    extract::Extension,
    response::{IntoResponse, Response},
};
use shared::{AppState, TenantContext};

use crate::extractors::RequireAuth;
use crate::view::dashboard::dashboard_page;

pub async fn show_dashboard(
    RequireAuth(user): RequireAuth,
    Extension(ctx): Extension<TenantContext>,
) -> Response {
    dashboard_page(&ctx.slug, &user.username).into_response()
}

