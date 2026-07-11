use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use crate::models::UserModel::User;
use crate::models::SessionModel::Session;
use crate::middleware::TenantMiddleware::TenantContext;
use crate::repositories::UserRepository;
use sqlx::SqlitePool;

pub async fn require_auth_middleware(mut req: Request, next: Next) -> Response {
    let path = req.uri().path().to_string();
    
    // 1. Skip auth check for login pages
    if path.contains("/login") || !path.starts_with("/t/") {
        return next.run(req).await;
    }

    // 2. Check if we have a session (injected by SessionMiddleware)
    let session = req.extensions().get::<Session>().cloned();
    let pool = req.extensions().get::<SqlitePool>().cloned();
    let ctx = req.extensions().get::<TenantContext>().cloned();

    if let (Some(sess), Some(pool), Some(ctx)) = (session, pool, ctx) {
        // 3. Verify user still exists and is active
        if let Ok(Some(user)) = UserRepository::get_user_by_id(&pool, sess.user_id).await {
            if user.is_active {
                // Attach user to extensions so controllers can use it
                req.extensions_mut().insert(user);
                return next.run(req).await;
            }
        }
        
        // If user is invalid/inactive, redirect to login
        return Redirect::to(&format!("/t/{}/login", ctx.slug)).into_response();
    }

    // 4. No session? Redirect to login
    if let Some(ctx) = req.extensions().get::<TenantContext>() {
        Redirect::to(&format!("/t/{}/login", ctx.slug)).into_response()
    } else {
        Redirect::to("/").into_response()
    }
}
