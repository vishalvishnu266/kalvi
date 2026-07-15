//! Per-tenant user account model + Argon2 password helpers.
//!
//! Password hashing uses **Argon2id** — the modern OWASP recommendation for
//! new applications (memory-hard, resistant to GPU cracking).
//!
//! Sessions are opaque server-side tokens stored in the `sessions` table.
//! The cookie only carries the session id; all authorization data lives in
//! the DB, so revoking a session is a single DELETE.

use argon2::Argon2;
use password_hash::{
    rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

use crate::errors::AppError;

// -------------------------------------------------------------------
// Role
// -------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Admin,
    Teacher,
    Accountant,
    Librarian,
    Student,
    Guardian,
}

impl Role {
    pub fn as_str(&self) -> &'static str {
        match self {
            Role::Admin => "admin",
            Role::Teacher => "teacher",
            Role::Accountant => "accountant",
            Role::Librarian => "librarian",
            Role::Student => "student",
            Role::Guardian => "guardian",
        }
    }

    pub fn from_str_opt(s: &str) -> Option<Self> {
        match s {
            "admin" => Some(Role::Admin),
            "teacher" => Some(Role::Teacher),
            "accountant" => Some(Role::Accountant),
            "librarian" => Some(Role::Librarian),
            "student" => Some(Role::Student),
            "guardian" => Some(Role::Guardian),
            _ => None,
        }
    }
}

// -------------------------------------------------------------------
// User
// -------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Default)]
pub struct User {
    pub id: String,
    pub email: String,
    #[serde(skip_serializing)] // never leak the hash into JSON responses
    pub password_hash: String,
    pub full_name: String,
    pub role: String, // Kept as string to survive future role additions.
    pub status: String,
    pub teacher_id: Option<String>,
    pub student_id: Option<String>,
    pub last_login_at: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl User {
    pub fn is_active(&self) -> bool {
        self.status == "active"
    }

    pub fn role_enum(&self) -> Option<Role> {
        Role::from_str_opt(&self.role)
    }

    /// Two-letter initials for the navbar avatar.
    pub fn initials(&self) -> String {
        let parts: Vec<&str> = self.full_name.split_whitespace().collect();
        match parts.as_slice() {
            [] => "?".into(),
            [only] => only.chars().next().map(|c| c.to_ascii_uppercase().to_string()).unwrap_or_else(|| "?".into()),
            [first, .., last] => {
                let a = first.chars().next().unwrap_or('?').to_ascii_uppercase();
                let b = last.chars().next().unwrap_or('?').to_ascii_uppercase();
                format!("{a}{b}")
            }
        }
    }

    /// Look up a user by email (case-insensitive because the column is COLLATE NOCASE).
    pub async fn find_by_email(pool: &SqlitePool, email: &str) -> Result<Option<Self>, AppError> {
        Ok(sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ?")
            .bind(email.trim())
            .fetch_optional(pool)
            .await?)
    }

    pub async fn find_by_id(pool: &SqlitePool, id: &str) -> Result<Option<Self>, AppError> {
        Ok(sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await?)
    }

    /// Record login timestamp.
    pub async fn touch_last_login(pool: &SqlitePool, id: &str) -> Result<(), AppError> {
        sqlx::query("UPDATE users SET last_login_at = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    // ---------------------------------------------------------------
    // Admin operations
    // ---------------------------------------------------------------

    /// List users with optional filtering by role, status, and free-text search.
    /// Ordered by full_name.
    pub async fn list(
        pool: &SqlitePool,
        filters: &UserFilters,
    ) -> Result<Vec<Self>, AppError> {
        let mut sql = String::from("SELECT * FROM users WHERE 1=1");
        let mut binds: Vec<String> = Vec::new();

        if let Some(role) = filters.role.as_ref().filter(|s| !s.is_empty()) {
            sql.push_str(" AND role = ?");
            binds.push(role.clone());
        }
        if let Some(status) = filters.status.as_ref().filter(|s| !s.is_empty()) {
            sql.push_str(" AND status = ?");
            binds.push(status.clone());
        }
        if let Some(q) = filters.q.as_ref().filter(|s| !s.is_empty()) {
            sql.push_str(" AND (full_name LIKE ? OR email LIKE ?)");
            let pat = format!("%{}%", q);
            binds.push(pat.clone());
            binds.push(pat);
        }
        sql.push_str(" ORDER BY full_name");

        let mut query = sqlx::query_as::<_, User>(&sql);
        for b in &binds {
            query = query.bind(b);
        }
        Ok(query.fetch_all(pool).await?)
    }

    /// Insert a new user, hashing the plaintext password. Returns the id.
    pub async fn create(
        pool: &SqlitePool,
        email: &str,
        plaintext_password: &str,
        full_name: &str,
        role: Role,
    ) -> Result<String, AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        let hash = hash_password(plaintext_password)?;
        sqlx::query(
            r#"INSERT INTO users (id, email, password_hash, full_name, role, status)
               VALUES (?, ?, ?, ?, ?, 'active')"#,
        )
        .bind(&id)
        .bind(email.trim())
        .bind(&hash)
        .bind(full_name.trim())
        .bind(role.as_str())
        .execute(pool)
        .await?;
        Ok(id)
    }

    /// Update a user's editable profile fields (NOT the password).
    /// Returns the number of rows affected (0 if id not found).
    pub async fn update_profile(
        pool: &SqlitePool,
        id: &str,
        email: &str,
        full_name: &str,
        role: Role,
        status: &str,
    ) -> Result<u64, AppError> {
        let status = if status == "suspended" { "suspended" } else { "active" };
        let res = sqlx::query(
            r#"UPDATE users
               SET email = ?, full_name = ?, role = ?, status = ?, updated_at = CURRENT_TIMESTAMP
               WHERE id = ?"#,
        )
        .bind(email.trim())
        .bind(full_name.trim())
        .bind(role.as_str())
        .bind(status)
        .bind(id)
        .execute(pool)
        .await?;
        Ok(res.rows_affected())
    }

    /// Overwrite a user's password (admin reset OR self-service change).
    pub async fn set_password(
        pool: &SqlitePool,
        id: &str,
        new_plaintext: &str,
    ) -> Result<(), AppError> {
        let hash = hash_password(new_plaintext)?;
        sqlx::query(
            "UPDATE users SET password_hash = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?",
        )
        .bind(&hash)
        .bind(id)
        .execute(pool)
        .await?;
        Ok(())
    }

    /// Soft-delete a user by suspending them. We keep the row so historical
    /// audit trails (session.user_id → user) continue to resolve.
    pub async fn suspend(pool: &SqlitePool, id: &str) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE users SET status = 'suspended', updated_at = CURRENT_TIMESTAMP WHERE id = ?",
        )
        .bind(id)
        .execute(pool)
        .await?;
        // Also revoke all live sessions immediately.
        sqlx::query("DELETE FROM sessions WHERE user_id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn reactivate(pool: &SqlitePool, id: &str) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE users SET status = 'active', updated_at = CURRENT_TIMESTAMP WHERE id = ?",
        )
        .bind(id)
        .execute(pool)
        .await?;
        Ok(())
    }
}

// -------------------------------------------------------------------
// Filters + form structs
// -------------------------------------------------------------------

/// Query-string filters for the admin user list page.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct UserFilters {
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub q: Option<String>,
}

impl UserFilters {
    pub fn role_matches(&self, r: &str) -> bool {
        self.role.as_deref() == Some(r)
    }
    pub fn status_matches(&self, s: &str) -> bool {
        self.status.as_deref() == Some(s)
    }
    pub fn q_value(&self) -> String {
        self.q.clone().unwrap_or_default()
    }
}

/// Form payload for create/edit user.
#[derive(Debug, Default, Deserialize)]
pub struct UserForm {
    pub csrf_token: Option<String>,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub full_name: String,
    #[serde(default)]
    pub role: String,
    #[serde(default)]
    pub status: String,
    /// Only used on create, and optional on edit (blank = keep current).
    #[serde(default)]
    pub password: String,
}

impl UserForm {
    pub fn validate(&self, is_edit: bool) -> Result<(), String> {
        if self.full_name.trim().is_empty() {
            return Err("Full name is required.".into());
        }
        if self.email.trim().is_empty() {
            return Err("Email is required.".into());
        }
        if !self.email.contains('@') {
            return Err("Please enter a valid email address.".into());
        }
        if Role::from_str_opt(&self.role).is_none() {
            return Err("Please select a valid role.".into());
        }
        if !is_edit && self.password.len() < 8 {
            return Err("Password must be at least 8 characters.".into());
        }
        if is_edit && !self.password.is_empty() && self.password.len() < 8 {
            return Err("New password must be at least 8 characters.".into());
        }
        Ok(())
    }
}

/// Form payload for self-service change-password.
#[derive(Debug, Default, Deserialize)]
pub struct ChangePasswordForm {
    pub csrf_token: Option<String>,
    #[serde(default)]
    pub current_password: String,
    #[serde(default)]
    pub new_password: String,
    #[serde(default)]
    pub confirm_password: String,
}

impl ChangePasswordForm {
    pub fn validate(&self) -> Result<(), String> {
        if self.current_password.is_empty() {
            return Err("Current password is required.".into());
        }
        if self.new_password.len() < 8 {
            return Err("New password must be at least 8 characters.".into());
        }
        if self.new_password != self.confirm_password {
            return Err("New password and confirmation do not match.".into());
        }
        if self.new_password == self.current_password {
            return Err("New password must be different from the current password.".into());
        }
        Ok(())
    }
}

// -------------------------------------------------------------------
// Extra User methods (display helpers + seed helper)
// -------------------------------------------------------------------
impl User {
    /// Human-friendly label for the role. Used in list rendering.
    pub fn role_label(&self) -> &'static str {
        match self.role_enum() {
            Some(Role::Admin) => "Admin",
            Some(Role::Teacher) => "Teacher",
            Some(Role::Accountant) => "Accountant",
            Some(Role::Librarian) => "Librarian",
            Some(Role::Student) => "Student",
            Some(Role::Guardian) => "Guardian",
            None => "Unknown",
        }
    }

    pub fn status_badge_class(&self) -> &'static str {
        match self.status.as_str() {
            "active" => "badge-success-soft",
            "suspended" => "badge-danger-soft",
            _ => "badge-info-soft",
        }
    }

    /// Ensure the seed admin's password hash matches "admin123" for dev.
    ///
    /// Called at tenant-DB init after migrations. This avoids relying on a
    /// static SQL-embedded hash (whose exact bytes must match the argon2
    /// version compiled in). Idempotent — no-op if already correctly hashed.
    pub async fn ensure_dev_seed_password(
        pool: &SqlitePool,
        email: &str,
        plaintext: &str,
    ) -> Result<(), AppError> {
        if let Some(user) = Self::find_by_email(pool, email).await? {
            let already_good = verify_password(plaintext, &user.password_hash).unwrap_or(false);
            if !already_good {
                let new_hash = hash_password(plaintext)?;
                sqlx::query("UPDATE users SET password_hash = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                    .bind(&new_hash)
                    .bind(&user.id)
                    .execute(pool)
                    .await?;
                tracing::info!("Reset seed user password for {}", email);
            }
        }
        Ok(())
    }
}

// -------------------------------------------------------------------
// Argon2 helpers
// -------------------------------------------------------------------

/// Hash a password using Argon2id (default OWASP-recommended parameters as
/// exposed by the `argon2` crate: m=19456, t=2, p=1). Returns the encoded
/// PHC string, which contains the algorithm, params, salt, and hash.
pub fn hash_password(plaintext: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon = Argon2::default();
    let hash = argon
        .hash_password(plaintext.as_bytes(), &salt)
        .map_err(|e| AppError::Unexpected(format!("hash failure: {e}")))?;
    Ok(hash.to_string())
}

/// Verify a plaintext password against a stored PHC hash.
/// Never leak *why* verification failed to the caller — always show a
/// generic "invalid credentials" message to defend against user enumeration.
pub fn verify_password(plaintext: &str, encoded_hash: &str) -> Result<bool, AppError> {
    let parsed = PasswordHash::new(encoded_hash)
        .map_err(|e| AppError::Unexpected(format!("bad hash format: {e}")))?;
    Ok(Argon2::default()
        .verify_password(plaintext.as_bytes(), &parsed)
        .is_ok())
}
