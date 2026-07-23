//! Authentication + authorization:
//!
//! * `register` — creates a user with an Argon2-hashed password
//! * `login`    — verifies a password, touches `last_login_at`, returns the user
//! * `change_password` — verifies the old password before updating
//! * `assign_role` / `has_permission` — thin RBAC helpers

use std::sync::Arc;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::Duration;
use rand_core::RngCore;

use crate::repositories::Repositories;
use crate::repositories::auth::{NewSession, NewUser, Session, User};
use crate::session::SessionStore;
use crate::services::{ServiceError, ServiceResult};

/// Default web session lifetime. Fine as a starting point; move to config later.
pub const DEFAULT_SESSION_TTL_DAYS: i64 = 14;

#[derive(Clone)]
pub struct AuthService {
    repos: Arc<Repositories>,
    sessions: SessionStore,
}

impl AuthService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        let sessions = SessionStore::TenantDb(repos.sessions.clone());
        Self { repos, sessions }
    }

    pub fn with_session_store(
        repos: Arc<Repositories>,
        sessions: SessionStore,
    ) -> Self {
        Self { repos, sessions }
    }

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

    pub async fn register(
        &self, username: &str, email: Option<&str>, password: &str, roles: &[&str],
    ) -> ServiceResult<User> {
        if password.len() < 8 {
            return Err(ServiceError::validation("password must be >= 8 chars"));
        }
        if self.repos.users.find_by_username(username).await?.is_some() {
            return Err(ServiceError::conflict("username taken"));
        }

        let hash = Self::hash(password)?;
        let user = self.repos.users.create(&NewUser {
            username: username.into(),
            email:    email.map(str::to_string),
            password_hash: hash,
            is_active: true,
        }).await?;

        for role_name in roles {
            if let Some(role) = self.repos.roles.find_by_name(role_name).await? {
                self.repos.users.assign_role(user.id, role.id).await?;
            }
        }
        Ok(user)
    }

    /// Login by username-or-email + password. Returns the user on success.
    pub async fn login(&self, identifier: &str, password: &str) -> ServiceResult<User> {
        let user = if identifier.contains('@') {
            self.repos.users.find_by_email(identifier).await?
        } else {
            self.repos.users.find_by_username(identifier).await?
        };
        let user = user.ok_or(ServiceError::Unauthorized)?;
        if !user.is_active { return Err(ServiceError::Unauthorized); }
        if !Self::verify(password, &user.password_hash) {
            return Err(ServiceError::Unauthorized);
        }
        self.repos.users.touch_last_login(user.id).await?;
        Ok(user)
    }

    pub async fn change_password(
        &self, user_id: i64, old_password: &str, new_password: &str,
    ) -> ServiceResult<()> {
        if new_password.len() < 8 {
            return Err(ServiceError::validation("password must be >= 8 chars"));
        }
        let user = self.repos.users.get(user_id).await?;
        if !Self::verify(old_password, &user.password_hash) {
            return Err(ServiceError::Unauthorized);
        }
        let hash = Self::hash(new_password)?;
        self.repos.users.update_password_hash(user_id, &hash).await?;
        Ok(())
    }

    /// Admin-driven password reset (no old-password check).
    pub async fn reset_password(&self, user_id: i64, new_password: &str) -> ServiceResult<()> {
        if new_password.len() < 8 {
            return Err(ServiceError::validation("password must be >= 8 chars"));
        }
        let hash = Self::hash(new_password)?;
        self.repos.users.update_password_hash(user_id, &hash).await?;
        Ok(())
    }

    pub async fn assign_role(&self, user_id: i64, role_name: &str) -> ServiceResult<()> {
        let role = self.repos.roles.find_by_name(role_name).await?
            .ok_or(ServiceError::NotFound)?;
        self.repos.users.assign_role(user_id, role.id).await?;
        Ok(())
    }

    pub async fn has_permission(&self, user_id: i64, code: &str) -> ServiceResult<bool> {
        let perms = self.repos.users.permissions_of(user_id).await?;
        Ok(perms.iter().any(|p| p.code == code))
    }

    /// Convenience gate: returns `Forbidden(code)` if the user lacks the permission.
    pub async fn require_permission(&self, user_id: i64, code: &str) -> ServiceResult<()> {
        if self.has_permission(user_id, code).await? {
            Ok(())
        } else {
            Err(ServiceError::forbidden(code))
        }
    }

    // ---------------------------------------------------------- sessions

    /// Generate a cryptographically random opaque session token.
    fn mint_token() -> String {
        let mut bytes = [0u8; 32];
        OsRng.fill_bytes(&mut bytes);
        // URL-safe hex; 64 chars, ~256 bits of entropy.
        bytes.iter().map(|b| format!("{:02x}", b)).collect()
    }

    /// Issue a new session for `user_id` in the configured tenant session backend.
    /// Returns the created row so callers can grab the token to set as a cookie.
    pub async fn issue_session(
        &self,
        user_id: i64,
        user_agent: Option<String>,
        remote_ip: Option<String>,
    ) -> ServiceResult<Session> {
        self.issue_session_with_ttl(user_id, DEFAULT_SESSION_TTL_DAYS, user_agent, remote_ip).await
    }

    pub async fn issue_session_with_ttl(
        &self,
        user_id: i64,
        ttl_days: i64,
        user_agent: Option<String>,
        remote_ip: Option<String>,
    ) -> ServiceResult<Session> {
        let expires_at = chrono::Utc::now().naive_utc() + Duration::days(ttl_days);
        let token = Self::mint_token();
        let s = self.sessions.create(&NewSession {
            token, user_id, expires_at, user_agent, remote_ip,
        }).await?;
        Ok(s)
    }

    /// Resolve a session cookie value into `(session, user)`. Returns
    /// `Unauthorized` if the token is unknown, revoked, or expired, or
    /// if the user has been disabled since sign-in.
    pub async fn resolve_session(&self, token: &str) -> ServiceResult<(Session, User)> {
        let session = self.sessions.find_active_by_token(token).await?
            .ok_or(ServiceError::Unauthorized)?;
        let user = self.repos.users.get(session.user_id).await?;
        if !user.is_active {
            return Err(ServiceError::Unauthorized);
        }
        // Best-effort refresh of last_seen_at; failures shouldn't block the request.
        let _ = self.sessions.touch(session.id).await;
        Ok((session, user))
    }

    /// Revoke a specific session (used on logout).
    pub async fn revoke_session(&self, token: &str) -> ServiceResult<()> {
        self.sessions.revoke_by_token(token).await?;
        Ok(())
    }

    /// Persist session backend state (no-op for tenant-db sessions).
    pub async fn checkpoint_sessions(&self) -> ServiceResult<()> {
        self.sessions.checkpoint().await?;
        Ok(())
    }
}
