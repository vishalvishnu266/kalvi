use axum::{
    extract::{State, Form},
    response::{Html, IntoResponse, Response},
};
use serde::Deserialize;
use crate::config::AppState;
use crate::view::OnboardingView;
use crate::service::TenantService;
use crate::util::AppError;

#[derive(Deserialize)]
pub struct OnboardForm {
    pub name: String,
    pub slug: String,
    pub admin_username: String,
    pub admin_password: String,
}

impl OnboardForm {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().len() < 3 {
            return Err("Institution Name must be at least 3 characters".to_string());
        }
        if self.slug.trim().len() < 3 {
            return Err("Slug must be at least 3 characters".to_string());
        }
        if !crate::util::html_util::is_valid_slug(&self.slug) {
            return Err("Invalid slug format (use lowercase, numbers, and hyphens)".to_string());
        }
        if self.admin_username.trim().len() < 3 {
            return Err("Admin Username must be at least 3 characters".to_string());
        }
        if self.admin_password.len() < 8 {
            return Err("Admin Password must be at least 8 characters".to_string());
        }
        Ok(())
    }
}

pub async fn show_form() -> Html<String> {
    use crate::util::html_util::IntoHtml;
    OnboardingView::render_form(None).into_html()
}

pub async fn submit_form(
    State(state): State<AppState>,
    Form(form): Form<OnboardForm>,
) -> Response {
    use crate::util::html_util::IntoHtml;
    
    // 1. Validate Form (Business Error)
    if let Err(msg) = form.validate() {
        return OnboardingView::render_form(Some(msg)).into_html().into_response();
    }
    
    // 2. Business Logic Execution
    match TenantService::create_tenant(
        &state,
        &form.name,
        &form.slug,
        &form.admin_username,
        &form.admin_password,
    ).await {
        Ok(tenant) => OnboardingView::render_success(&tenant.slug, &tenant.name).into_html().into_response(),
        Err(AppError::Validation(msg)) => OnboardingView::render_form(Some(msg)).into_html().into_response(),
        Err(other_error) => other_error.into_response(),
    }
}
