use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use sqlx::SqlitePool;
use crate::AppState;

pub async fn session_middleware(
    _state: State<AppState>,
    mut req: Request,
    next: Next,
) -> Response {
    if let Some(session_id) = auth::session::extract_session_cookie(req.headers()) {
        if let Some(pool) = req.extensions().get::<SqlitePool>().cloned() {
            if let Ok(Some(sess)) = auth::repository::get_valid_session(&pool, &session_id).await {
                req.extensions_mut().insert(sess);
            }
        }
    }
    next.run(req).await
}
