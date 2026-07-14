use axum::{
    extract::{State, Form},
    response::{Html, IntoResponse, Response, Redirect},
    Extension,
};
use serde::Deserialize;
use std::collections::HashMap;
use crate::config::AppState;
use crate::middleware::TenantContext;
use crate::service::{TenantService, UserService};
use crate::view::{LoginView, CommonLoginView};
use crate::util::{errors::AppError, SessionUtil};

#[derive(Deserialize)]
pub struct CommonLoginForm {
    pub tenant: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

pub async fn show_common_login() -> Html<String> {
    Html(CommonLoginView::render_common_login(HashMap::new(), None))
}

pub async fn process_common_login(
    State(state): State<AppState>,
    Form(form): Form<CommonLoginForm>,
) -> Response {
    let slug = form.tenant.trim().to_lowercase();
    
    let tenant = match TenantService::find_by_slug(&state, &slug).await {
        Ok(Some(t)) => t,
        Ok(None) => return Html(CommonLoginView::render_common_login(HashMap::new(), Some("Institution not found".to_string()))).into_response(),
        Err(e) => return e.into_response(),
    };

    let tenant_pool = match state.db.get_tenant_pool(&tenant.database_name).await {
        Ok(p) => p,
        Err(e) => return AppError::from(e).into_response(),
    };

    match UserService::authenticate(&tenant_pool, &form.username, &form.password).await {
        Ok(Some(user)) => {
            match UserService::create_session(&tenant_pool, user.id).await {
                Ok(session_id) => {
                    let mut response = Redirect::to(&format!("/web/{}/dashboard", tenant.slug)).into_response();
                    SessionUtil::set_session_cookie(&mut response, &session_id);
                    response
                }
                Err(e) => e.into_response(),
            }
        }
        Ok(None) => Html(CommonLoginView::render_common_login(HashMap::new(), Some("Invalid credentials".to_string()))).into_response(),
        Err(e) => e.into_response(),
    }
}

pub async fn show_login(Extension(ctx): Extension<TenantContext>) -> Html<String> {
    Html(LoginView::render_login(&ctx.tenant, HashMap::new(), None))
}

pub async fn process_login(
    Extension(ctx): Extension<TenantContext>,
    Form(form): Form<LoginForm>,
) -> Response {
    match UserService::authenticate(&ctx.pool, &form.username, &form.password).await {
        Ok(Some(user)) => {
            match UserService::create_session(&ctx.pool, user.id).await {
                Ok(session_id) => {
                    let mut response = Redirect::to(&format!("/web/{}/dashboard", ctx.tenant.slug)).into_response();
                    SessionUtil::set_session_cookie(&mut response, &session_id);
                    response
                }
                Err(e) => e.into_response(),
            }
        }
        Ok(None) => Html(LoginView::render_login(&ctx.tenant, HashMap::new(), Some("Invalid credentials".to_string()))).into_response(),
        Err(AppError::Validation(fields, gen)) => Html(LoginView::render_login(&ctx.tenant, fields, gen)).into_response(),
        Err(e) => e.into_response(),
    }
}
