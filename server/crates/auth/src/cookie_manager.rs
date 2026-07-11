// Cookie management for sessions
// Handles setting/clearing session cookies with proper security

use axum::{
    http::{header, HeaderValue, Request},
    response::Response,
};
use cookie::{Cookie, SameSite};
use std::time::Duration;

const COOKIE_NAME: &str = "session_id";
const SESSION_DURATION_SECS: u64 = 24 * 3600; // 24 hours

/// Extract session ID from request cookies
pub fn extract_session_id<B>(req: &Request<B>) -> Option<String> {
    let cookie_header = req.headers().get(header::COOKIE)?;
    let cookie_str = cookie_header.to_str().ok()?;
    
    // Parse cookies
    for cookie_str in cookie_str.split(';') {
        if let Ok(cookie) = Cookie::parse(cookie_str.trim()) {
            if cookie.name() == COOKIE_NAME {
                return Some(cookie.value().to_string());
            }
        }
    }
    
    None
}

/// Set session cookie in response
pub fn set_session_cookie(response: &mut Response, session_id: &str) {
    let cookie = Cookie::build((COOKIE_NAME, session_id))
        .path("/")
        .max_age(cookie::time::Duration::seconds(SESSION_DURATION_SECS as i64))
        .http_only(true)        // Not accessible via JavaScript
        .same_site(SameSite::Lax)  // CSRF protection
        // .secure(true)         // Uncomment for production (HTTPS only)
        .build();
    
    if let Ok(header_value) = HeaderValue::from_str(&cookie.to_string()) {
        response.headers_mut().insert(header::SET_COOKIE, header_value);
    }
}

/// Clear session cookie (logout)
pub fn clear_session_cookie(response: &mut Response) {
    let cookie = Cookie::build((COOKIE_NAME, ""))
        .path("/")
        .max_age(cookie::time::Duration::ZERO)
        .http_only(true)
        .same_site(SameSite::Lax)
        .build();
    
    if let Ok(header_value) = HeaderValue::from_str(&cookie.to_string()) {
        response.headers_mut().insert(header::SET_COOKIE, header_value);
    }
}

/// Extract client IP address from request
pub fn extract_client_ip<B>(req: &Request<B>) -> Option<String> {
    // Try X-Forwarded-For header first (if behind proxy)
    if let Some(forwarded) = req.headers().get("x-forwarded-for") {
        if let Ok(forwarded_str) = forwarded.to_str() {
            // Get first IP from comma-separated list
            if let Some(ip) = forwarded_str.split(',').next() {
                return Some(ip.trim().to_string());
            }
        }
    }
    
    // Try X-Real-IP header
    if let Some(real_ip) = req.headers().get("x-real-ip") {
        if let Ok(ip_str) = real_ip.to_str() {
            return Some(ip_str.to_string());
        }
    }
    
    // Fallback to connection info (not available in axum middleware easily)
    None
}

/// Extract user agent from request
pub fn extract_user_agent<B>(req: &Request<B>) -> Option<String> {
    req.headers()
        .get(header::USER_AGENT)
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string())
}
