use axum::{
    extract::{State, Form},
    response::{Html, IntoResponse, Response, Redirect},
    middleware::Extension,
};
use serde::Deserialize;
use std::collections::HashMap;
use crate::config::AppState;
use crate::middleware::TenantContext;
use crate::service::StudentService;
use crate::view::StudentView;
use crate::util::AppError;

#[derive(Deserialize)]
pub struct AddStudentForm {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub enrollment_number: String,
}

pub async fn show_add_form(Extension(ctx): Extension<TenantContext>) -> Html<String> {
    Html(StudentView::render_add_form(&ctx.tenant, HashMap::new(), None))
}

pub async fn process_add(
    Extension(ctx): Extension<TenantContext>,
    Form(form): Form<AddStudentForm>,
) -> Response {
    let email = if form.email.trim().is_empty() { None } else { Some(form.email.as_str()) };
    let enroll = if form.enrollment_number.trim().is_empty() { None } else { Some(form.enrollment_number.as_str()) };

    match StudentService::add_student(
        &ctx.pool,
        &form.first_name,
        &form.last_name,
        email,
        None, // phone
        enroll,
    ).await {
        Ok(_) => Redirect::to(&format!("/web/{}/dashboard", ctx.tenant.slug)).into_response(),
        Err(AppError::BusinessException(msg, fields)) => Html(StudentView::render_add_form(&ctx.tenant, fields, Some(msg))).into_response(),
        Err(e) => e.into_response(),
    }
}
