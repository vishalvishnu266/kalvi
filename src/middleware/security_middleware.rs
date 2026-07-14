use axum::{
    body::Body,
    http::{Request, HeaderValue},
    middleware::Next,
    response::Response,
};

pub async fn security_middleware(req: Request<Body>, next: Next) -> Response {
    let mut response = next.run(req).await;
    
    let headers = response.headers_mut();
    
    // 1. Prevent Clickjacking
    headers.insert("X-Frame-Options", HeaderValue::from_static("DENY"));
    
    // 2. Prevent XSS sniffing
    headers.insert("X-Content-Type-Options", HeaderValue::from_static("nosniff"));
    
    // 3. Enable HSTS (Strict Transport Security) - 1 year
    headers.insert("Strict-Transport-Security", HeaderValue::from_static("max-age=31536000; includeSubDomains"));
    
    // 4. Referrer Policy
    headers.insert("Referrer-Policy", HeaderValue::from_static("strict-origin-when-cross-origin"));
    
    // 5. Basic CSP (Can be tuned further)
    headers.insert("Content-Security-Policy", HeaderValue::from_static("default-src 'self'; script-src 'self' cdn.jsdelivr.net; style-src 'self' 'unsafe-inline' cdn.jsdelivr.net; img-src 'self' data:; font-src 'self' data:;"));

    response
}
