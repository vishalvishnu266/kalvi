use axum::{
    extract::Extension,
    response::Html,
};
use crate::middleware::TenantContext;
use crate::model::User;
use crate::view::DashboardView;

pub async fn show_dashboard(
    Extension(ctx): Extension<TenantContext>,
    Extension(user): Extension<User>,
) -> Html<String> {
    Html(DashboardView::render_dashboard(&ctx.tenant, &user))
}
