use bcrypt::{hash, DEFAULT_COST};
use sqlx::SqlitePool;

/// Insert an admin user with a bcrypt-hashed password.
/// If the username already exists, this is a no-op.
pub async fn create_admin_user(
    pool: &SqlitePool,
    username: &str,
    password: &str,
) -> Result<(), String> {
    let hashed = hash(password, DEFAULT_COST).map_err(|e| format!("bcrypt error: {e}"))?;

    sqlx::query(
        "INSERT OR IGNORE INTO users (username, password_hash, role, is_active)
         VALUES (?, ?, 'admin', 1)",
    )
    .bind(username)
    .bind(hashed)
    .execute(pool)
    .await
    .map_err(|e| format!("db error: {e}"))?;

    Ok(())
}
