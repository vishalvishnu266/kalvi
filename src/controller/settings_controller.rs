use axum::{
    extract::{Extension, Form, State},
    response::{Html, IntoResponse, Redirect, Response},
};
use serde::Deserialize;
use crate::config::AppState;
use crate::middleware::TenantContext;
use crate::view::SettingsView;
use crate::service::TenantService;
use crate::util::AppError;

#[derive(Deserialize)]
pub struct SettingsForm {
    pub primary_color: String,
    pub dark_mode: Option<String>,
}

pub async fn show_settings(Extension(ctx): Extension<TenantContext>) -> Html<String> {
    Html(SettingsView::render_settings(&ctx.tenant, None))
}

pub async fn process_settings(
    State(state): State<AppState>,
    Extension(ctx): Extension<TenantContext>,
    Extension(user): Extension<crate::model::User>,
    Form(form): Form<SettingsForm>,
) -> Result<Response, AppError> {
    // Only Admin can update the global BRAND identity
    if user.role == "admin" {
        let dark_mode = form.dark_mode.is_some();
        TenantService::update_tenant_settings(&state, &ctx.tenant.slug, &form.primary_color, dark_mode).await?;
    }
    
    Ok(Redirect::to(&format!("/web/{}/dashboard", ctx.tenant.slug)).into_response())
}
