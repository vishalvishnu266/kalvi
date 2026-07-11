use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use validator::Validate;

/// User roles in the system
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    Teacher,
    Staff,
}

impl UserRole {
    pub fn as_str(&self) -> &str {
        match self {
            UserRole::Admin => "admin",
            UserRole::Teacher => "teacher",
            UserRole::Staff => "staff",
        }
    }
}

/// User model
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub role: UserRole,
    pub is_active: bool,
    pub created_at: String,
}

/// Login form data
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct LoginForm {
    #[validate(length(min = 3, message = "Username must be at least 3 characters"))]
    pub username: String,
    
    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String,
}

/// User creation form (for admin)
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateUserForm {
    #[validate(length(min = 3, max = 50, message = "Username must be 3-50 characters"))]
    pub username: String,
    
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
    
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
    
    pub role: UserRole,
}

/// Database operations for users
pub mod db {
    use super::*;
    
    /// Get user by username
    pub async fn get_user_by_username(
        pool: &SqlitePool,
        username: &str,
    ) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "SELECT id, username, email, password_hash, role, is_active, created_at 
             FROM users WHERE username = ?"
        )
        .bind(username)
        .fetch_optional(pool)
        .await
    }
    
    /// Get user by ID
    pub async fn get_user_by_id(
        pool: &SqlitePool,
        id: i64,
    ) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "SELECT id, username, email, password_hash, role, is_active, created_at 
             FROM users WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(pool)
        .await
    }
    
    /// Create a new user
    pub async fn create_user(
        pool: &SqlitePool,
        username: &str,
        email: &str,
        password_hash: &str,
        role: UserRole,
    ) -> Result<User, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO users (username, email, password_hash, role) 
             VALUES (?, ?, ?, ?)"
        )
        .bind(username)
        .bind(email)
        .bind(password_hash)
        .bind(role.as_str())
        .execute(pool)
        .await?;
        
        let user_id = result.last_insert_rowid();
        
        get_user_by_id(pool, user_id).await
            .and_then(|opt| opt.ok_or(sqlx::Error::RowNotFound))
    }
    
    /// List all users
    pub async fn list_users(pool: &SqlitePool) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "SELECT id, username, email, password_hash, role, is_active, created_at 
             FROM users ORDER BY username"
        )
        .fetch_all(pool)
        .await
    }
    
    /// Check if username exists
    pub async fn username_exists(
        pool: &SqlitePool,
        username: &str,
    ) -> Result<bool, sqlx::Error> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM users WHERE username = ?"
        )
        .bind(username)
        .fetch_one(pool)
        .await?;
        
        Ok(count.0 > 0)
    }
}

/// Password hashing utilities
pub mod password {
    use bcrypt::{hash, verify, DEFAULT_COST};
    
    /// Hash a password
    pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
        hash(password, DEFAULT_COST)
    }
    
    /// Verify a password against a hash
    pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
        verify(password, hash)
    }
}
