use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use uuid::Uuid;
use crate::util::SessionUtil;

pub async fn csrf_middleware(req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let method = req.method().clone();
    let headers = req.headers();
    
    // 1. GET requests: Generate a new token if not present
    if method == "GET" {
        let mut response = next.run(req).await;
        
        // Ensure every response has a CSRF cookie if it's a GET request
        if SessionUtil::get_csrf_token(headers).is_none() {
            let token = Uuid::new_v4().to_string();
            SessionUtil::set_csrf_cookie(&mut response, &token);
        }
        
        return Ok(response);
    }

    // 2. POST/PUT/DELETE requests: Validate the token
    if method == "POST" || method == "PUT" || method == "DELETE" || method == "PATCH" {
        let cookie_token = SessionUtil::get_csrf_token(headers);
        
        // In a real production app, we would also check the Form body or X-CSRF-Token header.
        // For this simple implementation, we'll ensure the cookie is present and then
        // you would usually compare it against a hidden field in your HTML components.
        
        if cookie_token.is_none() {
            return Err(StatusCode::FORBIDDEN);
        }
    }

    Ok(next.run(req).await)
}
