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
    let trace_id = Uuid::new_v4().to_string();
    
    // Capture request details
    let method = req.method().clone();
    let uri = req.uri().clone();
    
    // For body logging, we need to consume the body.
    let (parts, body) = req.into_parts();
    let bytes = to_bytes(body, 1024 * 64).await.unwrap_or_default(); // Limit to 64KB
    let body_str = String::from_utf8_lossy(&bytes).to_string();
    
    // Reconstruct request
    let req = Request::from_parts(parts, Body::from(bytes));
    
    let span = span!(Level::INFO, "http_request", %trace_id, %method, %uri);
    
    info!(parent: &span, body = %body_str, "request started");
    
    next.run(req).instrument(span).await
}
