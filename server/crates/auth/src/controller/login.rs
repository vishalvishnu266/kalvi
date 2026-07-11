use axum::{
    extract::Extension,
    http::HeaderMap,
    response::{IntoResponse, Redirect, Response},
    Form,
};
use serde::Deserialize;
use shared::{AppState, TenantContext};
use sqlx::SqlitePool;

use crate::repository;
use crate::session;
use crate::view::login::login_page;

fn render_login(ctx: &TenantContext, error: Option<String>) -> Response {
    login_page(ctx, error).into_response()
}

pub async fn show_login(Extension(ctx): Extension<TenantContext>) -> Response {
    render_login(&ctx, None)
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
    let user = match repository::get_user_by_username(&pool, form.username.trim()).await {
        Ok(Some(u)) => u,
        Ok(None) => return render_login(&ctx, Some("Invalid username or password".into())),
        Err(_) => {
            return render_login(&ctx, Some("Server error, please try again".into()))
        }
    };

    if !user.is_active {
        return render_login(&ctx, Some("This account is inactive".into()));
    }

    let ok = bcrypt::verify(&form.password, &user.password_hash).unwrap_or(false);
    if !ok {
        return render_login(&ctx.slug, Some("Invalid username or password".into()));
    }

    let ip = session::extract_client_ip(&headers);
    let ua = session::extract_user_agent(&headers);

    let sess = match repository::create_session(&pool, user.id, ip, ua).await {
        Ok(s) => s,
        Err(_) => return render_login(&ctx.slug, Some("Failed to create session".into())),
    };

    let mut resp = Redirect::to(&format!("/t/{}/dashboard", ctx.slug)).into_response();
    session::set_session_cookie(&mut resp, &sess.id);
    resp
}

