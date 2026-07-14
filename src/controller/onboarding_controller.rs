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

use std::collections::HashMap;

impl OnboardForm {
    pub fn validate(&self) -> Result<(), HashMap<String, String>> {
        let mut errors = HashMap::new();
        if self.name.trim().len() < 3 {
            errors.insert("name".to_string(), "Name must be at least 3 characters".to_string());
        }
        if self.slug.trim().len() < 3 {
            errors.insert("slug".to_string(), "Slug must be at least 3 characters".to_string());
        } else if !crate::util::html_util::is_valid_slug(&self.slug) {
            errors.insert("slug".to_string(), "Invalid slug format (lowercase, numbers, hyphens only)".to_string());
        }
        if self.admin_username.trim().len() < 3 {
            errors.insert("admin_username".to_string(), "Username must be at least 3 characters".to_string());
        }
        if self.admin_password.len() < 8 {
            errors.insert("admin_password".to_string(), "Password must be at least 8 characters".to_string());
        }

        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}

pub async fn show_form() -> Html<String> {
    use crate::util::html_util::IntoHtml;
    OnboardingView::render_form(HashMap::new(), None).into_html()
}

pub async fn submit_form(
    State(state): State<AppState>,
    Form(form): Form<OnboardForm>,
) -> Response {
    use crate::util::html_util::IntoHtml;
    
    // 1. Validate Form (Business Error)
    if let Err(errors) = form.validate() {
        return OnboardingView::render_form(errors, None).into_html().into_response();
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
        Err(AppError::Validation(fields, gen)) => OnboardingView::render_form(fields, gen).into_html().into_response(),
        Err(other_error) => other_error.into_response(),
    }
}
