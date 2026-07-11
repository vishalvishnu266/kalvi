use axum::{
    extract::{Extension, Form},
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use crate::middleware::TenantMiddleware::TenantContext;
use crate::models::UserModel::User;
use sqlx::SqlitePool;

use crate::repositories::TenantRepository;
use crate::views::SettingsView;

pub async fn show_settings(
    Extension(_user): Extension<User>,
    Extension(ctx): Extension<TenantContext>
) -> Response {
    SettingsView::render_settings(&ctx, None).into_response()
}

#[derive(Debug, Deserialize)]
pub struct ThemeForm {
    pub primary_color: String,
    pub dark_mode: Option<String>,
}

pub async fn update_settings(
    Extension(_user): Extension<User>,
    Extension(master_pool): Extension<SqlitePool>,
    Extension(mut ctx): Extension<TenantContext>,
    Form(form): Form<ThemeForm>,
) -> Response {
    let dark_mode = form.dark_mode.is_some();
    
    match TenantRepository::update_theme(&master_pool, &ctx.slug, &form.primary_color, dark_mode).await {
        Ok(_) => {
            ctx.primary_color = form.primary_color;
            ctx.dark_mode = dark_mode;
            SettingsView::render_settings(&ctx, Some("Settings updated successfully!".into())).into_response()
        }
        Err(err) => {
            eprintln!("update_theme error: {err:?}");
            SettingsView::render_settings(&ctx, Some("Failed to update settings".into())).into_response()
        }
    }
}
