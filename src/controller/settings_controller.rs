use axum::{
    extract::Extension,
    response::{Html, IntoResponse, Redirect, Response},
};
use crate::middleware::TenantContext;
use crate::view::SettingsView;
use crate::util::AppError;

pub async fn show_settings(Extension(ctx): Extension<TenantContext>) -> Html<String> {
    use crate::util::html_util::IntoHtml;
    SettingsView::render_settings(&ctx.tenant).into_html()
}

pub async fn process_settings(
    Extension(ctx): Extension<TenantContext>,
) -> Result<Response, AppError> {
    // We no longer process settings on server
    Ok(Redirect::to(&format!("/web/{}/dashboard", ctx.tenant.slug)).into_response())
}
