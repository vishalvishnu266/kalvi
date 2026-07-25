use axum::{http::StatusCode, Json};
use serde::Deserialize;

use crate::http::{ServiceHttpError, TenantScope};
use crate::models::auth::User;
use crate::services::auth as auth_svc;

#[derive(Deserialize)]
pub struct RegisterBody {
    username: String,
    email: Option<String>,
    password: String,
    #[serde(default)]
    roles: Vec<String>,
}

pub async fn register(
    scope: TenantScope,
    Json(b): Json<RegisterBody>,
) -> Result<Json<User>, ServiceHttpError> {
    let roles: Vec<&str> = b.roles.iter().map(|s| s.as_str()).collect();
    let u = auth_svc::register(
        &scope.pool,
        &b.username,
        b.email.as_deref(),
        &b.password,
        &roles,
    )
    .await?;
    Ok(Json(u))
}

#[derive(Deserialize)]
pub struct LoginBody {
    identifier: String,
    password: String,
}

pub async fn login(
    scope: TenantScope,
    Json(b): Json<LoginBody>,
) -> Result<Json<User>, ServiceHttpError> {
    Ok(Json(
        auth_svc::login(&scope.pool, &b.identifier, &b.password).await?,
    ))
}

#[derive(Deserialize)]
pub struct ChangePwBody {
    user_id: i64,
    old_password: String,
    new_password: String,
}

pub async fn change_password(
    scope: TenantScope,
    Json(b): Json<ChangePwBody>,
) -> Result<StatusCode, ServiceHttpError> {
    auth_svc::change_password(&scope.pool, b.user_id, &b.old_password, &b.new_password).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn whoami(scope: TenantScope) -> Json<serde_json::Value> {
    Json(serde_json::json!({ "tenant": scope.tenant.as_str() }))
}
