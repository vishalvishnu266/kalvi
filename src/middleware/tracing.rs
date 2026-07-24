use axum::{
    body::Body,
    extract::Request,
    middleware::Next,
    response::Response,
};
use tracing::{info, span, Instrument, Level};
use uuid::Uuid;
use axum::body::to_bytes;

pub async fn trace_request(
    req: Request<Body>,
    next: Next,
) -> Response {
    let correlation_id = Uuid::new_v4().to_string();

let (parts, body) = req.into_parts();
    let method = parts.method.clone();
    let uri = parts.uri.clone();
    let headers = parts.headers.clone();

let bytes = to_bytes(body, 1024 * 64).await.unwrap_or_default();
    let body_str = String::from_utf8_lossy(&bytes).to_string();

let req = Request::from_parts(parts, Body::from(bytes));

    let span = span!(Level::INFO, "http", %correlation_id);
    tracing::info!(parent: &span, "Request started");
    tracing::info!(parent: &span, %method, %uri, "Route info");
    tracing::info!(parent: &span, ?headers, "Headers");
    tracing::info!(parent: &span, body = %body_str, "Body payload");
    next.run(req).instrument(span).await
}
