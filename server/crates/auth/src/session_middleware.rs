use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};
use sqlx::SqlitePool;

use crate::session;

/// Reads the session cookie, looks up the session in the current tenant's
/// SQLite pool, and injects the `Session` into request extensions (if valid).
///
/// This middleware runs after `shared::tenant_middleware`, so the tenant
/// pool is already available in extensions. For non-tenant routes (no
/// tenant pool present) it is a no-op.
pub async fn session_middleware(mut req: Request, next: Next) -> Response {
    if let Some(session_id) = session::extract_session_cookie(req.headers()) {
        if let Some(pool) = req.extensions().get::<SqlitePool>().cloned() {
            if let Ok(Some(sess)) = session::get_valid_session(&pool, &session_id).await {
                req.extensions_mut().insert(sess);
            }
        }
    }
    next.run(req).await
}
