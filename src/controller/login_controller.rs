use axum::{
    extract::{Extension, Form, State},
    response::{Html, IntoResponse, Redirect},
};
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
) -> impl IntoResponse {
    let slug = form.slug.trim().to_lowercase();
    let tenant = match TenantService::find_by_slug(&state, &slug).await {
        Ok(Some(t)) => t,
        _ => return Html(CommonLoginView::render_common_login(Some("Institution not found".to_string()))).into_response(),
    };

    let tenant_pool = match state.db.get_tenant_pool(&tenant.database_name).await {
        Ok(p) => p,
        Err(_) => return Html(CommonLoginView::render_common_login(Some("Database error".to_string()))).into_response(),
    };

    match UserService::authenticate(&tenant_pool, &form.username, &form.password).await {
        Ok(Some(user)) => {
            match UserService::create_session(&tenant_pool, user.id).await {
                Ok(session_id) => {
                    let mut response = Redirect::to(&format!("/web/{}/dashboard", tenant.slug)).into_response();
                    SessionUtil::set_session_cookie(&mut response, &session_id);
                    SessionUtil::set_tenant_cookie(&mut response, &tenant.slug);
                    response
                }
                Err(e) => Html(CommonLoginView::render_common_login(Some(e))).into_response(),
            }
        }
        Ok(None) => Html(CommonLoginView::render_common_login(Some("Invalid credentials".to_string()))).into_response(),
        Err(e) => Html(CommonLoginView::render_common_login(Some(e))).into_response(),
    }
}

pub async fn show_login(Extension(ctx): Extension<TenantContext>) -> Html<String> {
    Html(LoginView::render_login(&ctx.tenant, None))
}

pub async fn process_login(
    Extension(ctx): Extension<TenantContext>,
    Form(form): Form<LoginForm>,
) -> impl IntoResponse {
    match UserService::authenticate(&ctx.pool, &form.username, &form.password).await {
        Ok(Some(user)) => {
            match UserService::create_session(&ctx.pool, user.id).await {
                Ok(session_id) => {
                    let mut response = Redirect::to(&format!("/web/{}/dashboard", ctx.tenant.slug)).into_response();
                    SessionUtil::set_session_cookie(&mut response, &session_id);
                    SessionUtil::set_tenant_cookie(&mut response, &ctx.tenant.slug);
                    response
                }
                Err(e) => Html(LoginView::render_login(&ctx.tenant, Some(e))).into_response(),
            }
        }
        Ok(None) => Html(LoginView::render_login(&ctx.tenant, Some("Invalid username or password".to_string()))).into_response(),
        Err(e) => Html(LoginView::render_login(&ctx.tenant, Some(e))).into_response(),
    }
}
