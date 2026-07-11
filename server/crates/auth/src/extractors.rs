use axum::{
    extract::FromRequestParts,
    http::request::Parts,
    response::{IntoResponse, Redirect, Response},
};
use shared::TenantContext;
use sqlx::SqlitePool;

use crate::models::{self, User};
use crate::session::Session;

/// Extractor: requires an authenticated user in the current tenant.
/// Redirects to the tenant login page on failure.
pub struct RequireAuth(pub User);

impl<S> FromRequestParts<S> for RequireAuth
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let login_url = parts
            .extensions
            .get::<TenantContext>()
            .map(|c| format!("/t/{}/login", c.slug))
            .unwrap_or_else(|| "/".to_string());

        let session = parts
            .extensions
            .get::<Session>()
            .cloned()
            .ok_or_else(|| Redirect::to(&login_url).into_response())?;

        let pool = parts
            .extensions
            .get::<SqlitePool>()
            .cloned()
            .ok_or_else(|| Redirect::to(&login_url).into_response())?;

        let user = models::get_user_by_id(&pool, session.user_id)
            .await
            .map_err(|_| Redirect::to(&login_url).into_response())?
            .ok_or_else(|| Redirect::to(&login_url).into_response())?;

        if !user.is_active {
            return Err(Redirect::to(&login_url).into_response());
        }

        Ok(RequireAuth(user))
    }
}
