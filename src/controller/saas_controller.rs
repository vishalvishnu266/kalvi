use axum::{
    extract::{State, Form},
    response::{Html, IntoResponse, Redirect},
};
use serde::Deserialize;
use crate::config::AppState;
use crate::view::SaasView;
use crate::util::SessionUtil;
use crate::service::SaasService;

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

pub async fn show_onboard(State(state): State<AppState>) -> impl IntoResponse {
    if SaasService::has_owners(&state.db.master_pool).await {
        Redirect::to("/saas/login").into_response()
    } else {
        Html(SaasView::render_onboard(None)).into_response()
    }
}

pub async fn process_onboard(
    State(state): State<AppState>,
    Form(form): Form<SaasOnboardForm>,
) -> impl IntoResponse {
    match SaasService::onboard_owner(&state.db.master_pool, &form.username, &form.password, &form.full_name).await {
        Ok(_) => Redirect::to("/saas/login").into_response(),
        Err(e) => Html(SaasView::render_onboard(Some(e))).into_response(),
    }
}

pub async fn show_login() -> Html<String> {
    Html(SaasView::render_login(None))
}

pub async fn process_login(
    State(state): State<AppState>,
    Form(form): Form<SaasLoginForm>,
) -> impl IntoResponse {
    match SaasService::authenticate(&state.db.master_pool, &form.username, &form.password).await {
        Ok(Some(_owner)) => {
            let session_id = SaasService::generate_session_id();
            let mut response = Redirect::to("/onboard").into_response();
            SessionUtil::set_session_cookie(&mut response, &session_id);
            response
        }
        Ok(None) => Html(SaasView::render_login(Some("Invalid credentials".to_string()))).into_response(),
        Err(e) => Html(SaasView::render_login(Some(e))).into_response(),
    }
}
