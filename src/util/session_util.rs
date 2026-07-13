use axum::http::{header, HeaderMap, HeaderValue};
use axum::response::Response;
use cookie::Cookie;

pub const SESSION_COOKIE: &str = "kalvi_session";
pub const TENANT_COOKIE: &str = "kalvi_tenant";
pub const CSRF_COOKIE: &str = "kalvi_csrf";

pub fn get_tenant_slug(headers: &HeaderMap) -> Option<String> {
    headers.get(header::COOKIE)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| {
            for raw in v.split(';') {
                if let Ok(c) = Cookie::parse(raw.trim()) {
                    if c.name() == TENANT_COOKIE {
                        return Some(c.value().to_string());
                    }
                }
            }
            None
        })
}

pub fn set_tenant_cookie(response: &mut Response, slug: &str) {
    let cookie = Cookie::build((TENANT_COOKIE, slug.to_string()))
        .path("/")
        .http_only(false) // JavaScript might need to know the tenant for UI purposes
        .same_site(cookie::SameSite::Lax)
        .max_age(cookie::time::Duration::days(30))
        .build();

    if let Ok(v) = HeaderValue::from_str(&cookie.to_string()) {
        response.headers_mut().append(header::SET_COOKIE, v);
    }
}

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

pub fn clear_tenant_cookie(response: &mut Response) {
    let cookie = Cookie::build((TENANT_COOKIE, ""))
        .path("/")
        .http_only(false)
        .max_age(cookie::time::Duration::ZERO)
        .build();

    if let Ok(v) = HeaderValue::from_str(&cookie.to_string()) {
        response.headers_mut().append(header::SET_COOKIE, v);
    }
}

pub fn finalize_logout(redirect_url: &str) -> Response {
    use axum::response::{IntoResponse, Redirect};
    let mut response = Redirect::to(redirect_url).into_response();
    clear_session_cookie(&mut response);
    response
}

pub fn get_csrf_token(headers: &HeaderMap) -> Option<String> {
    headers.get(header::COOKIE)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| {
            for raw in v.split(';') {
                if let Ok(c) = Cookie::parse(raw.trim()) {
                    if c.name() == CSRF_COOKIE {
                        return Some(c.value().to_string());
                    }
                }
            }
            None
        })
}

pub fn set_csrf_cookie(response: &mut Response, token: &str) {
    let cookie = Cookie::build((CSRF_COOKIE, token.to_string()))
        .path("/")
        .http_only(true) 
        .same_site(cookie::SameSite::Lax)
        .max_age(cookie::time::Duration::hours(1))
        .build();

    if let Ok(v) = HeaderValue::from_str(&cookie.to_string()) {
        response.headers_mut().append(header::SET_COOKIE, v);
    }
}

pub fn finalize_login(session_id: &str, tenant_slug: &str) -> Response {
    use axum::response::{IntoResponse, Redirect};
    let mut response = Redirect::to(&format!("/web/{}/dashboard", tenant_slug)).into_response();
    set_session_cookie(&mut response, session_id);
    set_tenant_cookie(&mut response, tenant_slug);
    response
}

pub fn is_saas_session(headers: &HeaderMap) -> bool {
    get_session_id(headers)
        .map(|sid| sid.starts_with("saas_"))
        .unwrap_or(false)
}
