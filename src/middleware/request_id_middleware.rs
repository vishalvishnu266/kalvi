use axum::{
    http::{Request, HeaderValue},
    middleware::Next,
    response::Response,
};
use tracing::{info_span, Instrument};
use crate::util::id_util::generate_uuid;

#[derive(Clone, Debug)]
pub struct RequestId(pub String);

pub async fn request_id_middleware<B>(mut req: Request<B>, next: Next) -> Response {
    let id = generate_uuid();
    
    // Insert into extensions for later retrieval
    req.extensions_mut().insert(RequestId(id.clone()));
    
    let span = info_span!("request", id = %id);
    
    let mut response = next.run(req).instrument(span).await;
    
    // Add to response headers
    if let Ok(header_value) = HeaderValue::from_str(&id) {
        response.headers_mut().insert("X-Request-ID", header_value);
    }
    
    response
}
