//! Authentication + authorization service.

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::Duration;
use rand_core::RngCore;
use sqlx::SqlitePool;

use crate::entity::permission::Permission;
use crate::entity::role::Role;
use crate::entity::session::{NewSession, Session};
use crate::entity::user::{NewUser, User};
use crate::exception::repo_error::RepoResult;
use crate::exception::service_error::{ServiceError, ServiceResult};
use crate::repository::{
    permission_repository, role_repository, session_repository, user_repository,
};

pub const DEFAULT_SESSION_TTL_DAYS: i64 = 14;

// ── Password helpers ────────────────────────────────────────────────

fn hash(password: &str) -> ServiceResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| ServiceError::Hash(e.to_string()))
}

fn verify(password: &str, hash: &str) -> bool {
    match PasswordHash::new(hash) {
        Ok(parsed) => Argon2::default()
            .verify_password(password.as_bytes(), &parsed)
            .is_ok(),
        Err(_) => false,
    }
}

fn mint_token() -> String {
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

// ── Read helpers exposed to filters / controllers ───────────────────

pub async fn get_user(pool: &SqlitePool, id: i64) -> RepoResult<User> {
    user_repository::get(pool, id).await
}

pub async fn roles_of(pool: &SqlitePool, user_id: i64) -> RepoResult<Vec<Role>> {
    role_repository::find_by_user(pool, user_id).await
}

pub async fn permissions_of(pool: &SqlitePool, user_id: i64) -> RepoResult<Vec<Permission>> {
    permission_repository::find_by_user(pool, user_id).await
}

// ── High-level auth workflows ───────────────────────────────────────

pub async fn register(
    pool: &SqlitePool,
    username: &str,
    email: Option<&str>,
    password: &str,
    roles: &[&str],
) -> ServiceResult<User> {
    if password.len() < 8 {
        return Err(ServiceError::validation("password must be >= 8 chars"));
    }
    if user_repository::find_by_username(pool, username)
        .await?
        .is_some()
    {
        return Err(ServiceError::conflict("username taken"));
    }
    let h = hash(password)?;
    let user = user_repository::create(
        pool,
        &NewUser {
            username: username.into(),
            email: email.map(str::to_string),
            password_hash: h,
            is_active: true,
        },
    )
    .await?;
    for name in roles {
        if let Some(r) = role_repository::find_by_name(pool, name).await? {
            role_repository::assign_to_user(pool, user.id, r.id).await?;
        }
    }
    Ok(user)
}

pub async fn login(pool: &SqlitePool, identifier: &str, password: &str) -> ServiceResult<User> {
    let user = if identifier.contains('@') {
        user_repository::find_by_email(pool, identifier).await?
    } else {
        user_repository::find_by_username(pool, identifier).await?
    };
    let user = user.ok_or(ServiceError::Unauthorized)?;
    if !user.is_active {
        return Err(ServiceError::Unauthorized);
    }
    if !verify(password, &user.password_hash) {
        return Err(ServiceError::Unauthorized);
    }
    user_repository::touch_last_login(pool, user.id).await?;
    Ok(user)
}

pub async fn change_password(
    pool: &SqlitePool,
    user_id: i64,
    old: &str,
    new: &str,
) -> ServiceResult<()> {
    if new.len() < 8 {
        return Err(ServiceError::validation("password must be >= 8 chars"));
    }
    let user = user_repository::get(pool, user_id).await?;
    if !verify(old, &user.password_hash) {
        return Err(ServiceError::Unauthorized);
    }
    let h = hash(new)?;
    user_repository::update_password_hash(pool, user_id, &h).await?;
    Ok(())
}

// ── Sessions ────────────────────────────────────────────────────────

pub async fn issue_session(
    sessions: &SqlitePool,
    user_id: i64,
    user_agent: Option<String>,
    remote_ip: Option<String>,
) -> ServiceResult<Session> {
    issue_session_with_ttl(
        sessions,
        user_id,
        DEFAULT_SESSION_TTL_DAYS,
        user_agent,
        remote_ip,
    )
    .await
}

pub async fn issue_session_with_ttl(
    sessions: &SqlitePool,
    user_id: i64,
    ttl_days: i64,
    user_agent: Option<String>,
    remote_ip: Option<String>,
) -> ServiceResult<Session> {
    let expires_at = chrono::Utc::now().naive_utc() + Duration::days(ttl_days);
    let token = mint_token();
    Ok(session_repository::create(
        sessions,
        &NewSession {
            token,
            user_id,
            tenant_id: None,
            expires_at,
            user_agent,
            remote_ip,
        },
    )
    .await?)
}

pub async fn resolve_session(
    pool: &SqlitePool,
    sessions: &SqlitePool,
    token: &str,
) -> ServiceResult<(Session, User)> {
    let session = session_repository::find_active_by_token(sessions, token)
        .await?
        .ok_or(ServiceError::Unauthorized)?;
    let user = user_repository::get(pool, session.user_id).await?;
    if !user.is_active {
        return Err(ServiceError::Unauthorized);
    }
    let _ = session_repository::touch(sessions, session.id).await;
    Ok((session, user))
}

pub async fn revoke_session(sessions: &SqlitePool, token: &str) -> ServiceResult<()> {
    session_repository::revoke_by_token(sessions, token).await?;
    Ok(())
}
