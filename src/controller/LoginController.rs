use axum::{
    extract::{Extension, Form, State},
    response::{Html, IntoResponse, Redirect},
};
use serde::Deserialize;
use bcrypt::verify;
use uuid::Uuid;
use chrono::{Utc, Duration};
use crate::config::AppState::AppState;
use crate::middleware::TenantMiddleware::TenantContext;
use crate::repository::{UserRepository::UserRepository, TenantRepository::TenantRepository};
use crate::model::User::Session;
use crate::view::{LoginView, CommonLoginView};
use crate::util::SessionUtil;

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
    let tenant = match TenantRepository::find_by_slug(&state.db.master_pool, &slug).await {
        Ok(Some(t)) => t,
        _ => return Html(CommonLoginView::render_common_login(Some("Institution not found".to_string()))).into_response(),
    };

    let tenant_pool = match state.db.get_tenant_pool(&tenant.database_name).await {
        Ok(p) => p,
        Err(_) => return Html(CommonLoginView::render_common_login(Some("Database error".to_string()))).into_response(),
    };

    let user = match UserRepository::find_by_username(&tenant_pool, &form.username).await {
        Ok(Some(u)) => u,
        _ => return Html(CommonLoginView::render_common_login(Some("Invalid credentials".to_string()))).into_response(),
    };

    if !verify(&form.password, &user.password_hash).unwrap_or(false) {
        return Html(CommonLoginView::render_common_login(Some("Invalid credentials".to_string()))).into_response();
    }

    let session_id = Uuid::new_v4().to_string();
    let session = Session {
        id: session_id.clone(),
        user_id: user.id,
        user_agent: None,
        client_ip: None,
        expires_at: (Utc::now() + Duration::days(7)).naive_utc(),
        created_at: None,
    };

    if UserRepository::save_session(&tenant_pool, &session).await.is_err() {
        return Html(CommonLoginView::render_common_login(Some("Failed to create session".to_string()))).into_response();
    }

    let mut response = Redirect::to(&format!("/{}/dashboard", tenant.slug)).into_response();
    SessionUtil::set_session_cookie(&mut response, &session_id);
    SessionUtil::set_tenant_cookie(&mut response, &tenant.slug);
    response
}

pub async fn show_login(Extension(ctx): Extension<TenantContext>) -> Html<String> {
    Html(LoginView::render_login(&ctx.tenant, None))
}

pub async fn process_login(
    Extension(ctx): Extension<TenantContext>,
    Form(form): Form<LoginForm>,
) -> impl IntoResponse {
    let user = match UserRepository::find_by_username(&ctx.pool, &form.username).await {
        Ok(Some(u)) => u,
        _ => return Html(LoginView::render_login(&ctx.tenant, Some("Invalid username or password".to_string()))).into_response(),
    };

    if !verify(&form.password, &user.password_hash).unwrap_or(false) {
        return Html(LoginView::render_login(&ctx.tenant, Some("Invalid username or password".to_string()))).into_response();
    }

    // Create session
    let session_id = Uuid::new_v4().to_string();
    let session = Session {
        id: session_id.clone(),
        user_id: user.id,
        user_agent: None,
        client_ip: None,
        expires_at: (Utc::now() + Duration::days(7)).naive_utc(),
        created_at: None,
    };

    if UserRepository::save_session(&ctx.pool, &session).await.is_err() {
        return Html(LoginView::render_login(&ctx.tenant, Some("Failed to create session".to_string()))).into_response();
    }

    let mut response = Redirect::to(&format!("/{}/dashboard", ctx.tenant.slug)).into_response();
    SessionUtil::set_session_cookie(&mut response, &session_id);
    SessionUtil::set_tenant_cookie(&mut response, &ctx.tenant.slug);
    response
}
