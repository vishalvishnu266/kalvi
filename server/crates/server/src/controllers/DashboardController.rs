use axum::{
    extract::Extension,
    response::{IntoResponse, Response},
};
use crate::middleware::TenantMiddleware::TenantContext;
use crate::views::DashboardView;
use crate::models::UserModel::User;
use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::response::Redirect;
use sqlx::SqlitePool;
use crate::repositories::UserRepository;
use crate::models::SessionModel::Session;

pub struct RequireAuth(pub User);

#[async_trait]
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

        let user = UserRepository::get_user_by_id(&pool, session.user_id)
            .await
            .map_err(|_| Redirect::to(&login_url).into_response())?
            .ok_or_else(|| Redirect::to(&login_url).into_response())?;

        Ok(RequireAuth(user))
    }
}

pub async fn show_dashboard(
    RequireAuth(user): RequireAuth,
    Extension(ctx): Extension<TenantContext>,
) -> Response {
    DashboardView::render_dashboard(&ctx, &user.username).into_response()
}
