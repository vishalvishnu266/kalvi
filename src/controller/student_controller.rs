use axum::{
    extract::{State, Form},
    response::{Html, IntoResponse},
    Extension,
};
use std::sync::Arc;
use serde::Deserialize;
use crate::config::database_config::DatabaseConfig;
use crate::middleware::tenant_middleware::TenantContext;
use crate::service::student_service::StudentService;
use crate::view::student_view::render_form;
use crate::view::layout_view::layout;
use crate::util::errors::AppError;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct StudentForm {
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
}

pub struct StudentController;

impl StudentController {
    pub async fn get_form(
        Extension(ctx): Extension<TenantContext>,
    ) -> Html<String> {
        let content = render_form("", "", "", &HashMap::new());
        Html(layout(&format!("Enroll Student | {}", ctx.tenant.name), content))
    }

    pub async fn post_student(
        Extension(ctx): Extension<TenantContext>,
        Form(form): Form<StudentForm>,
    ) -> impl IntoResponse {
        match StudentService::add_student(
            &ctx.pool,
            form.first_name.clone(),
            form.last_name.clone(),
            form.email.clone(),
            None,
            None,
        ).await {
            Ok(_) => {
                // Success: Redirect or show success message
                Html(r#"<turbo-frame id="student-form"><div class="p-4 bg-green-100 text-green-800 rounded">Student enrolled successfully!</div></turbo-frame>"#.to_string()).into_response()
            }
            Err(AppError::BusinessException { errors, .. }) => {
                // Return only the form fragment for Turbo
                Html(render_form(&form.first_name, &form.last_name, &form.email.unwrap_or_default(), &errors)).into_response()
            }
            Err(e) => e.into_response(),
        }
    }
}
