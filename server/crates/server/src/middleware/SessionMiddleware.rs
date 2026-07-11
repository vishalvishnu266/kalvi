use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use sqlx::SqlitePool;
use crate::config::AppState::AppState;
use crate::repositories::UserRepository;
use crate::web_utils::SessionUtils;

pub async fn session_middleware(
    _state: State<AppState>,
    mut req: Request,
    next: Next,
) -> Response {
    if let Some(session_id) = SessionUtils::extract_session_cookie(req.headers()) {
        if let Some(pool) = req.extensions().get::<SqlitePool>().cloned() {
            if let Ok(Some(sess)) = UserRepository::get_valid_session(&pool, &session_id).await {
                req.extensions_mut().insert(sess);
            }
        }
    }
    next.run(req).await
}
