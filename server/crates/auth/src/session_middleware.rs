// Session middleware - loads session from cookie and injects into request

use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};
use sqlx::SqlitePool;

use crate::session::SessionManager;
use crate::cookie_manager;

/// Session middleware for tenant routes
/// Extracts session ID from cookie, loads from DB, and injects into request
pub async fn session_middleware(
    mut req: Request,
    next: Next,
) -> Response {
    // Extract tenant pool from extensions (injected by tenant_middleware)
    let pool = match req.extensions().get::<SqlitePool>() {
        Some(p) => p.clone(),
        None => {
            // No pool available, skip session loading
            return next.run(req).await;
        }
    };

    // Extract session ID from cookie
    let session_id = cookie_manager::extract_session_id(&req);

    if let Some(id) = session_id {
        // Try to load session from database
        let session_manager = SessionManager::new(pool.clone());

        if let Ok(Some(session)) = session_manager.get_session(&id).await {
            // Validate session
            if session.is_valid() {
                // Update last activity (in background to avoid blocking)
                let session_manager_clone = session_manager.clone();
                let id_clone = id.clone();
                tokio::spawn(async move {
                    let _ = session_manager_clone.touch_session(&id_clone).await;
                });

                // Inject session into request
                req.extensions_mut().insert(session);
            }
        }
    }

    next.run(req).await
}
