use axum::{
    extract::Extension,
    response::Html,
};
use crate::middleware::TenantContext;
use crate::model::User;
use crate::view::DashboardView;
use crate::util::AppError;

use crate::util::html_util::IntoHtml;

pub async fn show_dashboard(
    Extension(ctx): Extension<TenantContext>,
    Extension(user): Extension<User>,
) -> Result<Html<String>, AppError> {
    Ok(DashboardView::render_dashboard(&ctx.tenant, &user).into_html())
}
