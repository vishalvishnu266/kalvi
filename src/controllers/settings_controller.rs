//! Tenant settings landing page + institution_type / display_name update.
//!
//! Routes:
//!   GET  /web/{tenant}/settings           -> settings landing (links to sub-pages)
//!   POST /web/{tenant}/settings/general   -> save institution_type/display_name/locale/timezone
use askama::Template;
use axum::{
    extract::{Form, Path},
    response::{Html, IntoResponse, Redirect, Response},
    Extension,
};
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::auth_middleware::CurrentUser;
use crate::errors::AppError;
use crate::models::academic_year::{AcademicYear, TenantSetting};
use crate::utils::page::PageChrome;

#[derive(Template)]
#[template(path = "settings/index.html")]
struct IndexTpl {
    chrome: PageChrome,
    tenant_id: String,

    // General settings values (with sensible defaults)
    institution_type: String,
    display_name: String,
    locale: String,
    timezone: String,

    // AY quick view
    current_year_name: String,
    total_years: i64,
}

pub async fn index_handler(
    Path(tenant_id): Path<String>,
    Extension(pool): Extension<SqlitePool>,
    Extension(cu): Extension<CurrentUser>,
) -> Result<Html<String>, AppError> {
    let chrome = PageChrome::load(&pool, &tenant_id, "settings", &cu).await?;

    let institution_type = TenantSetting::get_or(&pool, "institution_type", "school").await?;
    let display_name = TenantSetting::get_or(&pool, "display_name", "Institution").await?;
    let locale = TenantSetting::get_or(&pool, "locale", "en-IN").await?;
    let timezone = TenantSetting::get_or(&pool, "timezone", "Asia/Kolkata").await?;

    let current_year_name = AcademicYear::current(&pool)
        .await?
        .map(|y| y.name)
        .unwrap_or_else(|| "— none set —".to_string());
    let total_years: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM academic_years").fetch_one(&pool).await?;

    let tpl = IndexTpl {
        chrome,
        tenant_id,
        institution_type,
        display_name,
        locale,
        timezone,
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
    // Whitelist institution_type — never trust the raw form value.
    let it = match form.institution_type.as_str() {
        "school" | "university" => form.institution_type.as_str(),
        _ => "school",
    };
    TenantSetting::set(&pool, "institution_type", it).await?;
    TenantSetting::set(&pool, "display_name", form.display_name.trim()).await?;
    if !form.locale.trim().is_empty() {
        TenantSetting::set(&pool, "locale", form.locale.trim()).await?;
    }
    if !form.timezone.trim().is_empty() {
        TenantSetting::set(&pool, "timezone", form.timezone.trim()).await?;
    }
    Ok(Redirect::to(&format!("/web/{}/settings", tenant_id)).into_response())
}
