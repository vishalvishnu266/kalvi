use axum::{
    extract::{Extension, Form, State},
    response::{Html, IntoResponse},
};
use serde::Deserialize;
use crate::config::AppState::AppState;
use crate::middleware::TenantMiddleware::TenantContext;
use crate::repository::TenantRepository::TenantRepository;
use crate::view::SettingsView;

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
    Form(form): Form<SettingsForm>,
) -> impl IntoResponse {
    let dark_mode = form.dark_mode.is_some();
    
    match TenantRepository::update_settings(&state.db.master_pool, &ctx.tenant.slug, &form.primary_color, dark_mode).await {
        Ok(_) => {
            // Update context in-memory if needed or let refresh handle it
            let mut updated_tenant = ctx.tenant.clone();
            updated_tenant.primary_color = form.primary_color;
            updated_tenant.dark_mode = dark_mode;
            Html(SettingsView::render_settings(&updated_tenant, Some("Settings updated successfully!".to_string())))
        }
        Err(_) => Html(SettingsView::render_settings(&ctx.tenant, Some("Failed to update settings".to_string()))),
    }
}
