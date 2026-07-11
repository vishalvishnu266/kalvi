use axum::{
    extract::Extension,
    response::{IntoResponse, Response},
};
use crate::middleware::TenantMiddleware::TenantContext;
use crate::views::DashboardView;
use crate::models::UserModel::User;

pub async fn show_dashboard(
    Extension(user): Extension<User>,
    Extension(ctx): Extension<TenantContext>,
) -> Response {
    DashboardView::render_dashboard(&ctx, &user.username).into_response()
}
