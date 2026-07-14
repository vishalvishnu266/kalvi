use axum::{
    extract::{State, Form},
    response::{Html, IntoResponse, Redirect, Response},
};
use serde::Deserialize;
use crate::config::AppState;
use crate::view::SaasView;
use crate::util::{SessionUtil, AppError};
use crate::service::SaasService;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct SaasOnboardForm {
    pub username: String,
    pub full_name: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct SaasLoginForm {
    pub username: String,
    pub password: String,
}

impl SaasOnboardForm {
    pub fn validate(&self) -> Result<(), HashMap<String, String>> {
        let mut errors = HashMap::new();
        if self.username.trim().len() < 3 {
            errors.insert("username".to_string(), "Username must be at least 3 characters".to_string());
        }
        if self.full_name.trim().is_empty() {
            errors.insert("full_name".to_string(), "Full name is required".to_string());
        }
        if self.password.len() < 8 {
            errors.insert("password".to_string(), "Password must be at least 8 characters".to_string());
        }
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}

pub async fn show_onboard(State(state): State<AppState>) -> impl IntoResponse {
    if SaasService::has_owners(&state.db.master_pool).await {
        Redirect::to("/saas/login").into_response()
    } else {
        Html(SaasView::render_onboard(HashMap::new(), None)).into_response()
    }
}

pub async fn process_onboard(
    State(state): State<AppState>,
    Form(form): Form<SaasOnboardForm>,
) -> Response {
    if let Err(errors) = form.validate() {
        return Html(SaasView::render_onboard(errors, None)).into_response();
    }
    
    match SaasService::onboard_owner(&state.db.master_pool, &form.username, &form.password, &form.full_name).await {
        Ok(_) => Redirect::to("/saas/login").into_response(),
        Err(AppError::Validation(fields, gen)) => Html(SaasView::render_onboard(fields, gen)).into_response(),
        Err(e) => e.into_response(),
    }
}

pub async fn show_login() -> Html<String> {
    Html(SaasView::render_login(HashMap::new(), None))
}

pub async fn process_login(
    State(state): State<AppState>,
    Form(form): Form<SaasLoginForm>,
) -> Response {
    match SaasService::authenticate(&state.db.master_pool, &form.username, &form.password).await {
        Ok(Some(_owner)) => {
            let session_id = SaasService::generate_session_id();
            let mut response = Redirect::to("/onboard").into_response();
            SessionUtil::set_session_cookie(&mut response, &session_id);
            response
        }
        Ok(None) => Html(SaasView::render_login(HashMap::new(), Some("Invalid username or password".to_string()))).into_response(),
        Err(AppError::Validation(fields, gen)) => Html(SaasView::render_login(fields, gen)).into_response(),
        Err(e) => e.into_response(),
    }
}
