use axum::{
    extract::{State, Form},
    response::{Html, IntoResponse, Redirect},
};
use serde::Deserialize;
use bcrypt::{hash, verify, DEFAULT_COST};
use crate::config::AppState::AppState;
use crate::repository::SaasOwnerRepository::SaasOwnerRepository;
use crate::view::SaasView;
use crate::util::SessionUtil;
use uuid::Uuid;

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
    match SaasOwnerRepository::count(&state.db.master_pool).await {
        Ok(count) if count > 0 => Redirect::to("/saas/login").into_response(),
        _ => Html(SaasView::render_onboard(None)).into_response(),
    }
}

pub async fn process_onboard(
    State(state): State<AppState>,
    Form(form): Form<SaasOnboardForm>,
) -> impl IntoResponse {
    let hashed_pw = hash(&form.password, DEFAULT_COST).unwrap();
    match SaasOwnerRepository::save(&state.db.master_pool, &form.username, &hashed_pw, &form.full_name).await {
        Ok(_) => Redirect::to("/saas/login").into_response(),
        Err(_) => Html(SaasView::render_onboard(Some("Failed to create SaaS owner".to_string()))).into_response(),
    }
}

pub async fn show_login() -> Html<String> {
    Html(SaasView::render_login(None))
}

pub async fn process_login(
    State(state): State<AppState>,
    Form(form): Form<SaasLoginForm>,
) -> impl IntoResponse {
    let owner = match SaasOwnerRepository::find_by_username(&state.db.master_pool, &form.username).await {
        Ok(Some(o)) => o,
        _ => return Html(SaasView::render_login(Some("Invalid credentials".to_string()))).into_response(),
    };

    if !verify(&form.password, &owner.password_hash).unwrap_or(false) {
        return Html(SaasView::render_login(Some("Invalid credentials".to_string()))).into_response();
    }

    // Reuse session util for owner (master-db sessions or simple cookie for now)
    let session_id = format!("saas_{}", Uuid::new_v4());
    let mut response = Redirect::to("/onboard").into_response(); // SaaS owner goes to onboarding to create tenants
    SessionUtil::set_session_cookie(&mut response, &session_id);
    response
}
