use axum::{
    extract::{State, Form},
    response::{Html, IntoResponse, Response},
};
use serde::Deserialize;
use validator::Validate;
use crate::config::AppState;
use crate::view::OnboardingView;
use crate::service::TenantService;
use crate::util::AppError;

#[derive(Deserialize, Validate)]
pub struct OnboardForm {
    #[validate(length(min = 3, message = "Name must be at least 3 characters"))]
    pub name: String,
    #[validate(length(min = 3, message = "Slug must be at least 3 characters"))]
    #[validate(regex(path = *crate::util::html_util::SLUG_REGEX, message = "Invalid slug format"))]
    pub slug: String,
    #[validate(length(min = 3, message = "Username must be at least 3 characters"))]
    pub admin_username: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub admin_password: String,
}

pub async fn show_form() -> Html<String> {
    use crate::util::html_util::IntoHtml;
    OnboardingView::render_form(None).into_html()
}

pub async fn submit_form(
    State(state): State<AppState>,
    Form(form): Form<OnboardForm>,
) -> Result<Response, AppError> {
    use crate::util::html_util::IntoHtml;
    form.validate().map_err(|e| AppError::Internal(e.to_string()))?;
    
    let tenant = TenantService::create_tenant(
        &state,
        &form.name,
        &form.slug,
        &form.admin_username,
        &form.admin_password,
    ).await?;
    
    Ok(OnboardingView::render_success(&tenant.slug, &tenant.name).into_html().into_response())
}
