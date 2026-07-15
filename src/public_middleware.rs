use axum::{extract::Request, middleware::Next, response::Response};
use tracing::info;

pub async fn public_middleware(req: Request, next: Next) -> Response {
    info!("public middleware");
    next.run(req).await
}
