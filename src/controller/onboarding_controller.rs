use axum::{
    extract::{State, Form},
    response::{Html, IntoResponse, Response},
};
use serde::Deserialize;
use std::collections::HashMap;
use crate::config::AppState;
use crate::service::TenantService;
use crate::view::OnboardingView;
use crate::util::AppError;

#[derive(Deserialize)]
pub struct RegistrationForm {
    pub name: String,
    pub tenant: String,
    pub admin_username: String,
    pub admin_password: String,
}

pub async fn show_registration() -> Html<String> {
    Html(OnboardingView::render_form(HashMap::new(), None))
}

pub async fn process_registration(
    State(state): State<AppState>,
    Form(form): Form<RegistrationForm>,
) -> Response {
    // 1. Basic validation
    let mut errors = HashMap::new();
    if form.name.trim().len() < 3 { errors.insert("name".to_string(), "Min 3 chars".to_string()); }
    if form.tenant.trim().len() < 3 { errors.insert("tenant".to_string(), "Min 3 chars".to_string()); }
    
    if !errors.is_empty() {
        return Html(OnboardingView::render_form(errors, None)).into_response();
    }

    // 2. Business Execution
    match TenantService::register_tenant(
        &state,
        &form.name,
        &form.tenant,
        &form.admin_username,
        &form.admin_password
    ).await {
        Ok(tenant) => Html(OnboardingView::render_success(&tenant.name, &tenant.slug)).into_response(),
        Err(AppError::BusinessException(msg, fields)) => Html(OnboardingView::render_form(fields, Some(msg))).into_response(),
        Err(e) => e.into_response(),
    }
}
