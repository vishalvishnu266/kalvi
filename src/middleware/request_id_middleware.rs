use axum::{
    body::Body,
    http::Request,
    middleware::Next,
    response::Response,
};
use crate::util::id_util;
use tracing::{info_span, Instrument};

pub async fn request_id_middleware(req: Request<Body>, next: Next) -> Response {
    let request_id = id_util::generate_random_id("req");
    
    // Create a tracing span that includes the request ID
    let span = info_span!(
        "request",
        id = %request_id,
        method = %req.method(),
        path = %req.uri().path()
    );

    async move {
        let mut response = next.run(req).await;
        // Add request ID to response header for debugging
        response.headers_mut().insert("X-Request-ID", request_id.parse().unwrap());
        response
    }
    .instrument(span)
    .await
}
