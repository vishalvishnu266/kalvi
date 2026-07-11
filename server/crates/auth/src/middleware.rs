use axum::{
    extract::FromRequestParts,
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use sqlx::SqlitePool;

use crate::shared::{User, db};
use crate::session::Session;

/// Extractor that requires authentication
pub struct RequireAuth(pub User);

impl<S> FromRequestParts<S> for RequireAuth
where
    S: Send + Sync,
{
    type Rejection = axum::response::Response;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        // Extract session from extensions (injected by session middleware)
        let session = parts
            .extensions
            .get::<Session>()
            .ok_or_else(|| {
                // No session - redirect to login
                let tenant_context = parts.extensions.get::<::shared::middleware::TenantContext>();
                let path = parts.uri.path();
                let login_url = if let Some(ctx) = tenant_context {
                    format!("/t/{}/login?redirect={}", ctx.slug, urlencoding::encode(path))
                } else {
                    format!("/login?redirect={}", urlencoding::encode(path))
                };
                Redirect::to(&login_url).into_response()
            })?;
        
        // Validate session
        if !session.is_valid() {
            // Session expired or inactive
            let tenant_context = parts.extensions.get::<::shared::middleware::TenantContext>();
            let path = parts.uri.path();
            let login_url = if let Some(ctx) = tenant_context {
                format!("/t/{}/login?redirect={}", ctx.slug, urlencoding::encode(path))
            } else {
                format!("/login?redirect={}", urlencoding::encode(path))
            };
            return Err(Redirect::to(&login_url).into_response());
        }
        
        // Get tenant pool from extensions
        let pool = parts
            .extensions
            .get::<SqlitePool>()
            .ok_or_else(|| StatusCode::INTERNAL_SERVER_ERROR.into_response())?
            .clone();
        
        // Load user from database
        let user = db::get_user_by_id(&pool, session.user_id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?
            .ok_or_else(|| StatusCode::UNAUTHORIZED.into_response())?;
        
        // Check if user is active
        if !user.is_active {
            let tenant_context = parts.extensions.get::<::shared::middleware::TenantContext>();
            let login_url = if let Some(ctx) = tenant_context {
                format!("/t/{}/login?error=inactive", ctx.slug)
            } else {
                "/login?error=inactive".to_string()
            };
            return Err(Redirect::to(&login_url).into_response());
        }
        
        Ok(RequireAuth(user))
    }
}

/// Optional auth - doesn't require authentication but provides user if available
pub struct OptionalAuth(pub Option<User>);

impl<S> FromRequestParts<S> for OptionalAuth
where
    S: Send + Sync,
{
    type Rejection = axum::response::Response;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        // Try to get session from extensions
        let session = match parts.extensions.get::<Session>() {
            Some(s) if s.is_valid() => s,
            _ => return Ok(OptionalAuth(None)),
        };
        
        // Get tenant pool
        let pool = match parts.extensions.get::<SqlitePool>() {
            Some(p) => p.clone(),
            None => return Ok(OptionalAuth(None)),
        };
        
        // Try to load user
        let user = db::get_user_by_id(&pool, session.user_id)
            .await
            .ok()
            .flatten()
            .filter(|u| u.is_active);
        
        Ok(OptionalAuth(user))
    }
}
