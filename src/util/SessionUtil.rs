use axum::http::{header, HeaderMap, HeaderValue};
use axum::response::Response;
use cookie::Cookie;

pub const SESSION_COOKIE: &str = "kalvi_session";

pub fn get_session_id(headers: &HeaderMap) -> Option<String> {
    headers.get(header::COOKIE)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| {
            for raw in v.split(';') {
                if let Ok(c) = Cookie::parse(raw.trim()) {
                    if c.name() == SESSION_COOKIE {
                        return Some(c.value().to_string());
                    }
                }
            }
            None
        })
}

pub fn set_session_cookie(response: &mut Response, session_id: &str) {
    let cookie = Cookie::build((SESSION_COOKIE, session_id.to_string()))
        .path("/")
        .http_only(true)
        .same_site(cookie::SameSite::Lax)
        .max_age(cookie::time::Duration::days(7))
        .build();

    if let Ok(v) = HeaderValue::from_str(&cookie.to_string()) {
        response.headers_mut().insert(header::SET_COOKIE, v);
    }
}

pub fn clear_session_cookie(response: &mut Response) {
    let cookie = Cookie::build((SESSION_COOKIE, ""))
        .path("/")
        .http_only(true)
        .max_age(cookie::time::Duration::ZERO)
        .build();

    if let Ok(v) = HeaderValue::from_str(&cookie.to_string()) {
        response.headers_mut().insert(header::SET_COOKIE, v);
    }
}
