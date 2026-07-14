use axum::{
    body::Body,
    http::Request,
    middleware::Next,
    response::Response,
};
use tracing::{info_span, Instrument};
use crate::util::id_util;

#[derive(Clone, Debug)]
pub struct RequestId(pub String);

pub async fn request_id_middleware(mut req: Request<Body>, next: Next) -> Response {
    let request_id = id_util::generate_uuid();
    
    // Attach to request extensions for access in controllers
    req.extensions_mut().insert(RequestId(request_id.clone()));
    
    // Create a tracing span that includes the Request ID
    let span = info_span!(
        "request",
        id = %request_id,
        method = %req.method(),
        uri = %req.uri().path()
    );

    // Run the rest of the middleware chain inside this span
    let mut response = next.run(req).instrument(span).await;
    
    // Also attach the ID to the response headers for debugging
    response.headers_mut().insert("X-Request-ID", request_id.parse().unwrap());
    
    response
}
