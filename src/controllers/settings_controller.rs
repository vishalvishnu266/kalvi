use askama::Template;
use axum::{
    extract::{Form, Path},
    response::{Html, IntoResponse, Redirect, Response},
    Extension,
};
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::csrf_middleware::CSRF_TOKEN_VALUE;
use crate::errors::AppError;
use crate::services::academic_year_service::AcademicYearService;
use crate::services::settings_service::SettingsService;
use crate::services::student_service::StudentService;

#[derive(Template)]
#[template(path = "settings/index.html")]
struct IndexTpl<'a> {
    tenant_id: String,
    active: &'static str,
    csrf_token: &'a str,
    student_count: i64,

    institution_type: String,
    display_name: String,
    locale: String,
    timezone: String,

    current_year_name: String,
    total_years: i64,
}

pub async fn index_handler(
    Path(tenant_id): Path<String>,
    Extension(pool): Extension<SqlitePool>,
) -> Result<Html<String>, AppError> {
    let settings = SettingsService::get_general_settings(&pool).await?;
    
    let current_year_name = AcademicYearService::current(&pool)
        .await?
        .map(|y| y.name)
        .unwrap_or_else(|| "— none set —".to_string());
        
    let total_years = AcademicYearService::list_all(&pool).await?.len() as i64;
    let student_count = StudentService::count(&pool).await.unwrap_or(0);

    let tpl = IndexTpl {
        tenant_id,
        active: "settings",
        csrf_token: CSRF_TOKEN_VALUE,
        student_count,
        institution_type: settings.institution_type,
        display_name: settings.display_name,
        locale: settings.locale,
        timezone: settings.timezone,
        current_year_name,
        total_years,
    };
    Ok(Html(tpl.render()?))
}

#[derive(Debug, Deserialize)]
pub struct GeneralSettingsForm {
    pub csrf_token: Option<String>,
    pub institution_type: String,
    pub display_name: String,
    #[serde(default)]
    pub locale: String,
    #[serde(default)]
    pub timezone: String,
}

pub async fn update_general_handler(
    Path(tenant_id): Path<String>,
    Extension(pool): Extension<SqlitePool>,
    Form(form): Form<GeneralSettingsForm>,
) -> Result<Response, AppError> {
    SettingsService::update_general_settings(
        &pool,
        &form.institution_type,
        &form.display_name,
        &form.locale,
        &form.timezone,
    ).await?;
    
    Ok(Redirect::to(&format!("/web/{}/settings", tenant_id)).into_response())
}
