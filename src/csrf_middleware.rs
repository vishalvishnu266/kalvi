use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

/// Hardcoded token for development. Replace with a per-session token later.
pub const CSRF_TOKEN_VALUE: &str = "static_csrf_token_for_dev_12345";

const MAX_BODY_BYTES: usize = 1024 * 1024; // 1 MiB is plenty for HTML forms

/// Validate CSRF token for state-changing requests. Accepts either:
///  - `X-CSRF-Token` header (used by fetch / Turbo XHR), OR
///  - `csrf_token` field in a `application/x-www-form-urlencoded` body.
pub async fn csrf_middleware(
    request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let method = request.method().clone();

    if !matches!(method.as_str(), "POST" | "PUT" | "PATCH" | "DELETE") {
        return Ok(next.run(request).await);
    }

    // 1) Fast path: header.
    if let Some(v) = request.headers().get("X-CSRF-Token").and_then(|v| v.to_str().ok()) {
        if v == CSRF_TOKEN_VALUE {
            return Ok(next.run(request).await);
        }
    }

    // 2) Slow path: body inspection for form posts.
    let content_type = request
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();

    if !content_type.starts_with("application/x-www-form-urlencoded") {
        return Err(StatusCode::FORBIDDEN);
    }

    let (parts, body) = request.into_parts();
    let bytes = match to_bytes(body, MAX_BODY_BYTES).await {
        Ok(b) => b,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    let mut valid = false;
    for (k, v) in url::form_urlencoded::parse(&bytes) {
        if k == "csrf_token" && v == CSRF_TOKEN_VALUE {
            valid = true;
            break;
        }
    }

    if !valid {
        return Err(StatusCode::FORBIDDEN);
    }

    // Rebuild request with the same body so downstream extractors still work.
    let new_req = Request::from_parts(parts, Body::from(bytes));
    Ok(next.run(new_req).await)
}
