use axum::{
    response::Html,
    Extension,
};
use crate::middleware::TenantContext;
use crate::model::User;
use crate::repository::StudentRepository;
use crate::view::DashboardView;

pub async fn show_dashboard(
    Extension(ctx): Extension<TenantContext>,
    Extension(user): Extension<User>,
) -> Html<String> {
    let student_count = StudentRepository::count(&ctx.pool).await.unwrap_or(0_i64);
    Html(DashboardView::render_dashboard(&ctx.tenant, &user, student_count))
}
