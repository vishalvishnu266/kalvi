//! `/api/tenant/auth/*` — per-tenant authentication.

use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;

use crate::http::{ExtractServices, ExtractTenant, ServiceHttpError};
use crate::http::middleware::TenantScopeState;
use crate::repositories::auth::User;
use crate::tenancy::TenantId;

pub fn routes() -> Router<TenantScopeState> {
    Router::new()
        .route("/register",        post(register))
        .route("/login",           post(login))
        .route("/change-password", post(change_password))
        .route("/whoami",          get(whoami))
}

#[derive(Deserialize)]
struct RegisterBody {
    username: String,
    email: Option<String>,
    password: String,
    #[serde(default)]
    roles: Vec<String>,
}

async fn register(
    ExtractServices(app): ExtractServices,
    Json(b): Json<RegisterBody>,
) -> Result<Json<User>, ServiceHttpError> {
    let roles: Vec<&str> = b.roles.iter().map(|s| s.as_str()).collect();
    let u = app.auth
        .register(&b.username, b.email.as_deref(), &b.password, &roles).await?;
    Ok(Json(u))
}

#[derive(Deserialize)]
struct LoginBody { identifier: String, password: String }

async fn login(
    ExtractServices(app): ExtractServices,
    Json(b): Json<LoginBody>,
) -> Result<Json<User>, ServiceHttpError> {
    let u = app.auth.login(&b.identifier, &b.password).await?;
    Ok(Json(u))
}

#[derive(Deserialize)]
struct ChangePwBody { user_id: i64, old_password: String, new_password: String }

async fn change_password(
    ExtractServices(app): ExtractServices,
    Json(b): Json<ChangePwBody>,
) -> Result<axum::http::StatusCode, ServiceHttpError> {
    app.auth.change_password(b.user_id, &b.old_password, &b.new_password).await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}

async fn whoami(ExtractTenant(t): ExtractTenant) -> Json<serde_json::Value> {
    Json(serde_json::json!({ "tenant": t.as_str() }))
}
