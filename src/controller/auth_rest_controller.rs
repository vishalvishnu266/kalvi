use axum::{http::StatusCode, Json};

use crate::dto::auth_dto::{ChangePasswordRequest, LoginRequest, RegisterRequest};
use crate::entity::user::User;
use crate::exception::service_error::ServiceError;
use crate::security::tenant_scope::TenantScope;
use crate::service::auth_service;

pub async fn register(
    scope: TenantScope,
    Json(b): Json<RegisterRequest>,
) -> Result<Json<User>, ServiceError> {
    let roles: Vec<&str> = b.roles.iter().map(|s| s.as_str()).collect();
    let u = auth_service::register(
        &scope.pool,
        &b.username,
        b.email.as_deref(),
        &b.password,
        &roles,
    )
    .await?;
    Ok(Json(u))
}

pub async fn login(
    scope: TenantScope,
    Json(b): Json<LoginRequest>,
) -> Result<Json<User>, ServiceError> {
    Ok(Json(
        auth_service::login(&scope.pool, &b.identifier, &b.password).await?,
    ))
}

pub async fn change_password(
    scope: TenantScope,
    Json(b): Json<ChangePasswordRequest>,
) -> Result<StatusCode, ServiceError> {
    auth_service::change_password(&scope.pool, b.user_id, &b.old_password, &b.new_password).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn whoami(scope: TenantScope) -> Json<serde_json::Value> {
    Json(serde_json::json!({ "tenant": scope.tenant.as_str() }))
}
