use axum::{
    extract::{State, Form},
    response::{Html, IntoResponse, Response, Redirect},
};
use serde::Deserialize;
use std::collections::HashMap;
use crate::config::AppState;
use crate::service::SaasService;
use crate::view::SaasView;
use crate::util::AppError;

#[derive(Deserialize)]
pub struct SaasOnboardForm {
    pub username: String,
    pub full_name: String,
    pub password: String,
}

pub async fn show_onboard(State(state): State<AppState>) -> Response {
    match SaasService::has_owners(&state.db.master_pool).await {
        Ok(true) => Redirect::to("/saas/login").into_response(),
        Ok(false) => Html(SaasView::render_onboard(HashMap::new(), None)).into_response(),
        Err(e) => e.into_response(),
    }
}

pub async fn process_onboard(
    State(state): State<AppState>,
    Form(form): Form<SaasOnboardForm>,
) -> Response {
    let mut errors = HashMap::new();
    if form.username.trim().len() < 3 { errors.insert("username".to_string(), "Min 3 chars".to_string()); }
    if form.password.len() < 8 { errors.insert("password".to_string(), "Min 8 chars".to_string()); }
    
    if !errors.is_empty() {
        return Html(SaasView::render_onboard(errors, None)).into_response();
    }

    match SaasService::onboard_owner(&state.db.master_pool, &form.username, &form.password, &form.full_name).await {
        Ok(_) => Redirect::to("/saas/login").into_response(),
        Err(AppError::BusinessException(msg, fields)) => Html(SaasView::render_onboard(fields, Some(msg))).into_response(),
        Err(e) => e.into_response(),
    }
}

#[derive(Deserialize)]
pub struct SaasLoginForm {
    pub username: String,
    pub password: String,
}

pub async fn show_login() -> Response {
    Html(SaasView::render_login(HashMap::new(), None)).into_response()
}

pub async fn process_login(
    State(state): State<AppState>,
    Form(form): Form<SaasLoginForm>,
) -> Response {
    match SaasService::authenticate(&state.db.master_pool, &form.username, &form.password).await {
        Ok(Some(_owner)) => {
            // Success: For now redirect to a placeholder (Step 9 will add real session storage)
            Redirect::to("/saas/onboard").into_response()
        }
        Ok(None) => Html(SaasView::render_login(HashMap::new(), Some("Invalid credentials".to_string()))).into_response(),
        Err(e) => e.into_response(),
    }
}
