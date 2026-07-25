//! Authentication + authorization services.
//!
//! Free functions over a tenant `SqlitePool` plus the shared
//! `SessionStore`. All SQL for the `user_account`, `role`,
//! `permission`, `user_role`, and `role_permission` tables lives
//! in this module.

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::Duration;
use rand_core::RngCore;
use sqlx::SqlitePool;

use crate::error::{RepoError, RepoResult};
use crate::models::auth::{NewSession, NewUser, Permission, Role, Session, User};
use crate::services::{ServiceError, ServiceResult};
use crate::session::SessionStore;

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
            .verify_password(password.as_bytes(), &parsed).is_ok(),
        Err(_) => false,
    }
}

fn mint_token() -> String {
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

// ── User CRUD (was UserRepo) ────────────────────────────────────────

pub async fn create_user(pool: &SqlitePool, u: &NewUser) -> RepoResult<User> {
    let id = sqlx::query_scalar::<_, i64>(
        r#"INSERT INTO user_account (username, email, password_hash, is_active)
           VALUES (?, ?, ?, ?) RETURNING id"#,
    )
    .bind(&u.username).bind(&u.email).bind(&u.password_hash).bind(u.is_active as i64)
    .fetch_one(pool).await?;
    get_user(pool, id).await
}

pub async fn get_user(pool: &SqlitePool, id: i64) -> RepoResult<User> {
    sqlx::query_as::<_, User>("SELECT * FROM user_account WHERE id = ?")
        .bind(id).fetch_optional(pool).await?
        .ok_or(RepoError::NotFound)
}

pub async fn find_user_by_username(pool: &SqlitePool, username: &str) -> RepoResult<Option<User>> {
    Ok(sqlx::query_as::<_, User>("SELECT * FROM user_account WHERE username = ?")
        .bind(username).fetch_optional(pool).await?)
}

pub async fn find_user_by_email(pool: &SqlitePool, email: &str) -> RepoResult<Option<User>> {
    Ok(sqlx::query_as::<_, User>("SELECT * FROM user_account WHERE email = ?")
        .bind(email).fetch_optional(pool).await?)
}

pub async fn touch_last_login(pool: &SqlitePool, id: i64) -> RepoResult<()> {
    sqlx::query("UPDATE user_account SET last_login_at = datetime('now') WHERE id = ?")
        .bind(id).execute(pool).await?;
    Ok(())
}

pub async fn update_password_hash(pool: &SqlitePool, id: i64, h: &str) -> RepoResult<()> {
    sqlx::query("UPDATE user_account SET password_hash = ? WHERE id = ?")
        .bind(h).bind(id).execute(pool).await?;
    Ok(())
}

pub async fn assign_role_id(pool: &SqlitePool, user_id: i64, role_id: i64) -> RepoResult<()> {
    sqlx::query("INSERT OR IGNORE INTO user_role (user_id, role_id) VALUES (?, ?)")
        .bind(user_id).bind(role_id).execute(pool).await?;
    Ok(())
}

pub async fn roles_of(pool: &SqlitePool, user_id: i64) -> RepoResult<Vec<Role>> {
    Ok(sqlx::query_as::<_, Role>(
        r#"SELECT r.* FROM role r
           INNER JOIN user_role ur ON ur.role_id = r.id
           WHERE ur.user_id = ? ORDER BY r.name"#,
    ).bind(user_id).fetch_all(pool).await?)
}

pub async fn permissions_of(pool: &SqlitePool, user_id: i64) -> RepoResult<Vec<Permission>> {
    Ok(sqlx::query_as::<_, Permission>(
        r#"SELECT DISTINCT p.* FROM permission p
           INNER JOIN role_permission rp ON rp.permission_id = p.id
           INNER JOIN user_role ur       ON ur.role_id = rp.role_id
           WHERE ur.user_id = ? ORDER BY p.code"#,
    ).bind(user_id).fetch_all(pool).await?)
}

pub async fn find_role_by_name(pool: &SqlitePool, name: &str) -> RepoResult<Option<Role>> {
    Ok(sqlx::query_as::<_, Role>("SELECT * FROM role WHERE name = ?")
        .bind(name).fetch_optional(pool).await?)
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
    if find_user_by_username(pool, username).await?.is_some() {
        return Err(ServiceError::conflict("username taken"));
    }
    let h = hash(password)?;
    let user = create_user(pool, &NewUser {
        username: username.into(),
        email:    email.map(str::to_string),
        password_hash: h,
        is_active: true,
    }).await?;
    for name in roles {
        if let Some(r) = find_role_by_name(pool, name).await? {
            assign_role_id(pool, user.id, r.id).await?;
        }
    }
    Ok(user)
}

pub async fn login(pool: &SqlitePool, identifier: &str, password: &str) -> ServiceResult<User> {
    let user = if identifier.contains('@') {
        find_user_by_email(pool, identifier).await?
    } else {
        find_user_by_username(pool, identifier).await?
    };
    let user = user.ok_or(ServiceError::Unauthorized)?;
    if !user.is_active { return Err(ServiceError::Unauthorized); }
    if !verify(password, &user.password_hash) {
        return Err(ServiceError::Unauthorized);
    }
    touch_last_login(pool, user.id).await?;
    Ok(user)
}

pub async fn change_password(
    pool: &SqlitePool, user_id: i64, old: &str, new: &str,
) -> ServiceResult<()> {
    if new.len() < 8 {
        return Err(ServiceError::validation("password must be >= 8 chars"));
    }
    let user = get_user(pool, user_id).await?;
    if !verify(old, &user.password_hash) {
        return Err(ServiceError::Unauthorized);
    }
    let h = hash(new)?;
    update_password_hash(pool, user_id, &h).await?;
    Ok(())
}

// ── Sessions ────────────────────────────────────────────────────────

pub async fn issue_session(
    sessions: &SessionStore,
    user_id: i64,
    user_agent: Option<String>,
    remote_ip: Option<String>,
) -> ServiceResult<Session> {
    issue_session_with_ttl(sessions, user_id, DEFAULT_SESSION_TTL_DAYS, user_agent, remote_ip).await
}

pub async fn issue_session_with_ttl(
    sessions: &SessionStore,
    user_id: i64,
    ttl_days: i64,
    user_agent: Option<String>,
    remote_ip: Option<String>,
) -> ServiceResult<Session> {
    let expires_at = chrono::Utc::now().naive_utc() + Duration::days(ttl_days);
    let token = mint_token();
    Ok(sessions.create(&NewSession {
        token, user_id, tenant_id: None, expires_at, user_agent, remote_ip,
    }).await?)
}

pub async fn resolve_session(
    pool: &SqlitePool,
    sessions: &SessionStore,
    token: &str,
) -> ServiceResult<(Session, User)> {
    let session = sessions.find_active_by_token(token).await?
        .ok_or(ServiceError::Unauthorized)?;
    let user = get_user(pool, session.user_id).await?;
    if !user.is_active {
        return Err(ServiceError::Unauthorized);
    }
    let _ = sessions.touch(session.id).await;
    Ok((session, user))
}

pub async fn revoke_session(sessions: &SessionStore, token: &str) -> ServiceResult<()> {
    sessions.revoke_by_token(token).await?;
    Ok(())
}
