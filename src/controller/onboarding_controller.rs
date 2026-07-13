use axum::{
    extract::{State, Form},
    response::{Html, IntoResponse},
};
use serde::Deserialize;
use crate::config::AppState;
use crate::view::OnboardingView;
use crate::service::TenantService;

#[derive(Deserialize)]
pub struct OnboardForm {
    pub name: String,
    pub slug: String,
    pub admin_username: String,
    pub admin_password: String,
}

pub async fn show_form() -> Html<String> {
    Html(OnboardingView::render_form(None))
}

pub async fn submit_form(
    State(state): State<AppState>,
    Form(form): Form<OnboardForm>,
) -> impl IntoResponse {
    match TenantService::create_tenant(
        &state,
        &form.name,
        &form.slug,
        &form.admin_username,
        &form.admin_password,
    ).await {
        Ok(tenant) => Html(OnboardingView::render_success(&tenant.slug, &tenant.name)).into_response(),
        Err(e) => Html(OnboardingView::render_form(Some(e))).into_response(),
    }
}
