use axum::{
    extract::{State, Request, Path},
    middleware::Next,
    response::{Response, IntoResponse, Html},
};
use crate::state::AppState;
use sqlx::SqlitePool;
use tracing::error;
use tracing::log::info;

pub async fn public_middleware(
    req: Request,
    next: Next,
) -> Response {
    info!("public middleware");
    //TODO
    // need to handle / when session is present, redirect to /web/{tenant_id}/
    next.run(req).await
}
