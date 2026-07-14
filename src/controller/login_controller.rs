use axum::{
    extract::{Extension, Form, State},
    response::{Html, IntoResponse, Response},
};
use crate::util::AppError;
use serde::Deserialize;
use crate::config::AppState;
use crate::middleware::TenantContext;
use crate::view::{LoginView, CommonLoginView};
use crate::util::SessionUtil;
use crate::service::{UserService, TenantService};

#[derive(Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct CommonLoginForm {
    pub slug: String,
    pub username: String,
    pub password: String,
}

use crate::util::html_util::IntoHtml;

pub async fn show_common_login() -> Html<String> {
    CommonLoginView::render_common_login(None).into_html()
}

pub async fn process_common_login(
    State(state): State<AppState>,
    Form(form): Form<CommonLoginForm>,
) -> Response {
    let slug = form.slug.trim().to_lowercase();
    
    let tenant = match TenantService::find_by_slug(&state, &slug).await {
        Ok(Some(t)) => t,
        Ok(None) => return CommonLoginView::render_common_login(Some("Institution not found".to_string())).into_html().into_response(),
        Err(e) => return e.into_response(),
    };

    let tenant_pool = match state.db.get_tenant_pool(&tenant.database_name).await {
        Ok(p) => p,
        Err(e) => return AppError::from(e).into_response(),
    };

    match UserService::authenticate(&tenant_pool, &form.username, &form.password).await {
        Ok(Some(user)) => {
            match UserService::create_session(&tenant_pool, user.id).await {
                Ok(session_id) => SessionUtil::finalize_login(&session_id, &tenant.slug),
                Err(e) => AppError::from(e).into_response(),
            }
        }
        Ok(None) => CommonLoginView::render_common_login(Some("Invalid username or password".to_string())).into_html().into_response(),
        Err(e) => AppError::from(e).into_response(),
    }
}

pub async fn show_login(Extension(ctx): Extension<TenantContext>) -> Html<String> {
    LoginView::render_login(&ctx.tenant, None).into_html()
}

pub async fn process_login(
    Extension(ctx): Extension<TenantContext>,
    Form(form): Form<LoginForm>,
) -> Response {
    match UserService::authenticate(&ctx.pool, &form.username, &form.password).await {
        Ok(Some(user)) => {
            match UserService::create_session(&ctx.pool, user.id).await {
                Ok(session_id) => SessionUtil::finalize_login(&session_id, &ctx.tenant.slug),
                Err(e) => AppError::from(e).into_response(),
            }
        }
        Ok(None) => LoginView::render_login(&ctx.tenant, Some("Invalid username or password".to_string())).into_html().into_response(),
        Err(e) => AppError::from(e).into_response(),
    }
}
