use axum::{
    extract::{Extension, Form, State},
    response::{Html, IntoResponse, Redirect, Response},
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

pub async fn show_common_login() -> Html<String> {
    Html(CommonLoginView::render_common_login(None))
}

pub async fn process_common_login(
    State(state): State<AppState>,
    Form(form): Form<CommonLoginForm>,
) -> Result<Response, AppError> {
    let slug = form.slug.trim().to_lowercase();
    let tenant = match TenantService::find_by_slug(&state, &slug).await? {
        Some(t) => t,
        None => return Ok(Html(CommonLoginView::render_common_login(Some("Institution not found".to_string()))).into_response()),
    };

    let tenant_pool = state.db.get_tenant_pool(&tenant.database_name).await?;

    match UserService::authenticate(&tenant_pool, &form.username, &form.password).await? {
        Some(user) => {
            let session_id = UserService::create_session(&tenant_pool, user.id).await?;
            let mut response = Redirect::to(&format!("/web/{}/dashboard", tenant.slug)).into_response();
            SessionUtil::set_session_cookie(&mut response, &session_id);
            SessionUtil::set_tenant_cookie(&mut response, &tenant.slug);
            Ok(response)
        }
        None => Ok(Html(CommonLoginView::render_common_login(Some("Invalid credentials".to_string()))).into_response()),
    }
}

pub async fn show_login(Extension(ctx): Extension<TenantContext>) -> Html<String> {
    Html(LoginView::render_login(&ctx.tenant, None))
}

pub async fn process_login(
    Extension(ctx): Extension<TenantContext>,
    Form(form): Form<LoginForm>,
) -> Result<Response, AppError> {
    match UserService::authenticate(&ctx.pool, &form.username, &form.password).await? {
        Some(user) => {
            let session_id = UserService::create_session(&ctx.pool, user.id).await?;
            let mut response = Redirect::to(&format!("/web/{}/dashboard", ctx.tenant.slug)).into_response();
            SessionUtil::set_session_cookie(&mut response, &session_id);
            SessionUtil::set_tenant_cookie(&mut response, &ctx.tenant.slug);
            Ok(response)
        }
        None => Ok(Html(LoginView::render_login(&ctx.tenant, Some("Invalid username or password".to_string()))).into_response()),
    }
}
