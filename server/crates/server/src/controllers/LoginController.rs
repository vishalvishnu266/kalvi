use axum::{
    extract::Extension,
    http::HeaderMap,
    response::{IntoResponse, Redirect, Response},
    Form,
};
use serde::Deserialize;
use crate::config::AppState::AppState;
use crate::middleware::TenantMiddleware::TenantContext;
use sqlx::SqlitePool;

use crate::repositories::UserRepository;
use crate::web_utils::{SessionUtils, HtmlHelper};
use crate::views::LoginView;

pub async fn show_login(Extension(ctx): Extension<TenantContext>) -> Response {
    LoginView::render_login(&ctx, None).into_response()
}

#[derive(Debug, Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

pub async fn process_login(
    Extension(pool): Extension<SqlitePool>,
    Extension(ctx): Extension<TenantContext>,
    headers: HeaderMap,
    Form(form): Form<LoginForm>,
) -> Response {
    let user = match UserRepository::get_user_by_username(&pool, form.username.trim()).await {
        Ok(Some(u)) => u,
        Ok(None) => return LoginView::render_login(&ctx, Some("Invalid username or password".into())).into_response(),
        Err(_) => {
            return LoginView::render_login(&ctx, Some("Server error, please try again".into())).into_response()
        }
    };

    if !user.is_active {
        return LoginView::render_login(&ctx, Some("This account is inactive".into())).into_response();
    }

    let ok = bcrypt::verify(&form.password, &user.password_hash).unwrap_or(false);
    if !ok {
        return LoginView::render_login(&ctx, Some("Invalid username or password".into())).into_response();
    }

    let ip = SessionUtils::extract_client_ip(&headers);
    let ua = SessionUtils::extract_user_agent(&headers);

    let sess = match UserRepository::create_session(&pool, user.id, ip, ua).await {
        Ok(s) => s,
        Err(_) => return LoginView::render_login(&ctx, Some("Failed to create session".into())).into_response(),
    };

    let mut resp = Redirect::to(&format!("/t/{}/dashboard", ctx.slug)).into_response();
    SessionUtils::set_session_cookie(&mut resp, &sess.id);
    resp
}
