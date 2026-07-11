// Feature: User Logout
// Simple logout functionality that clears session

use axum::{
    Router,
    response::{IntoResponse, Redirect, Response},
    routing::get,
    extract::Extension,
};
use sqlx::SqlitePool;

use crate::session::{Session, SessionManager};
use crate::cookie_manager;
use ::shared::middleware::AppState;

// ============================================================================
// HTTP Handlers
// ============================================================================

async fn logout_handler(
    Extension(pool): Extension<SqlitePool>,
    Extension(tenant_context): Extension<::shared::middleware::TenantContext>,
    Extension(session): Extension<Session>,
) -> Response {
    // Revoke session in database
    let session_manager = SessionManager::new(pool);
    let _ = session_manager.revoke_session(&session.id).await;
    
    // Create redirect response
    let mut response = Redirect::to(&format!("/t/{}/login", tenant_context.slug)).into_response();
    
    // Clear session cookie
    cookie_manager::clear_session_cookie(&mut response);
    
    response
}

// ============================================================================
// Routes
// ============================================================================

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/t/{tenant_slug}/logout", get(logout_handler))
}
