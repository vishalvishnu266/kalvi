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

use crate::repositories::Repositories;
use crate::repositories::auth::{NewUser, User};
use crate::services::{ServiceError, ServiceResult};

#[derive(Clone)]
pub struct AuthService {
    repos: Arc<Repositories>,
}

impl AuthService {
    pub fn new(repos: Arc<Repositories>) -> Self { Self { repos } }

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
}
