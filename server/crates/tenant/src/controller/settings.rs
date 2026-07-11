use axum::{
    extract::{Extension, Form},
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use shared::{AppState, TenantContext};
use sqlx::SqlitePool;

use crate::repository;
use crate::view::settings::settings_page;

pub async fn show_settings(Extension(ctx): Extension<TenantContext>) -> Response {
    settings_page(&ctx, None).into_response()
}

#[derive(Debug, Deserialize)]
pub struct ThemeForm {
    pub primary_color: String,
    pub dark_mode: Option<String>,
}

pub async fn update_settings(
    Extension(master_pool): Extension<SqlitePool>,
    Extension(mut ctx): Extension<TenantContext>,
    Form(form): Form<ThemeForm>,
) -> Response {
    let dark_mode = form.dark_mode.is_some();
    
    match repository::update_theme(&master_pool, &ctx.slug, &form.primary_color, dark_mode).await {
        Ok(_) => {
            ctx.primary_color = form.primary_color;
            ctx.dark_mode = dark_mode;
            settings_page(&ctx, Some("Settings updated successfully!".into())).into_response()
        }
        Err(err) => {
            eprintln!("update_theme error: {err:?}");
            settings_page(&ctx, Some("Failed to update settings".into())).into_response()
        }
    }
}
