use axum::http::{header, HeaderMap, HeaderValue};
use axum::response::Response;
use chrono::{Duration, Utc};
use cookie::{Cookie, SameSite};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::Session;

pub const COOKIE_NAME: &str = "session_id";
const SESSION_DURATION_DAYS: i64 = 7;

/// Create a new session row and return it.
pub async fn create_session(
    pool: &SqlitePool,
    user_id: i64,
    ip_address: Option<String>,
    user_agent: Option<String>,
) -> Result<Session, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let created_at = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let expires_at = (Utc::now() + Duration::days(SESSION_DURATION_DAYS))
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();

    sqlx::query(
        "INSERT INTO sessions (id, user_id, ip_address, user_agent, created_at, expires_at)
         VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(user_id)
    .bind(&ip_address)
    .bind(&user_agent)
    .bind(&created_at)
    .bind(&expires_at)
    .execute(pool)
    .await?;

    Ok(Session {
        id,
        user_id,
        ip_address,
        user_agent,
        created_at,
        expires_at,
    })
}

/// Load a session by ID if it exists and hasn't expired.
pub async fn get_valid_session(
    pool: &SqlitePool,
    session_id: &str,
) -> Result<Option<Session>, sqlx::Error> {
    let session: Option<Session> = sqlx::query_as::<_, Session>(
        "SELECT * FROM sessions WHERE id = ? AND expires_at > datetime('now')",
    )
    .bind(session_id)
    .fetch_optional(pool)
    .await?;

    Ok(session)
}

/// Delete a single session by ID.
pub async fn delete_session(pool: &SqlitePool, session_id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM sessions WHERE id = ?")
        .bind(session_id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Read the session cookie from a request's headers.
pub fn extract_session_cookie(headers: &HeaderMap) -> Option<String> {
    let cookie_header = headers.get(header::COOKIE)?.to_str().ok()?;
    for raw in cookie_header.split(';') {
        if let Ok(c) = Cookie::parse(raw.trim()) {
            if c.name() == COOKIE_NAME {
                return Some(c.value().to_string());
            }
        }
    }
    None
}

/// Set the session cookie on an outgoing response.
pub fn set_session_cookie(response: &mut Response, session_id: &str) {
    let cookie = Cookie::build((COOKIE_NAME, session_id.to_string()))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(cookie::time::Duration::days(SESSION_DURATION_DAYS))
        .build();

    if let Ok(v) = HeaderValue::from_str(&cookie.to_string()) {
        response.headers_mut().insert(header::SET_COOKIE, v);
    }
}

/// Clear the session cookie (logout).
pub fn clear_session_cookie(response: &mut Response) {
    let cookie = Cookie::build((COOKIE_NAME, ""))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(cookie::time::Duration::ZERO)
        .build();

    if let Ok(v) = HeaderValue::from_str(&cookie.to_string()) {
        response.headers_mut().insert(header::SET_COOKIE, v);
    }
}

pub fn extract_user_agent(headers: &HeaderMap) -> Option<String> {
    headers
        .get(header::USER_AGENT)
        .and_then(|h| h.to_str().ok())
        .map(str::to_string)
}

pub fn extract_client_ip(headers: &HeaderMap) -> Option<String> {
    if let Some(fwd) = headers.get("x-forwarded-for").and_then(|h| h.to_str().ok()) {
        if let Some(first) = fwd.split(',').next() {
            return Some(first.trim().to_string());
        }
    }
    headers
        .get("x-real-ip")
        .and_then(|h| h.to_str().ok())
        .map(str::to_string)
}
