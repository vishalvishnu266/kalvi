use sqlx::SqlitePool;
use bcrypt::{hash, DEFAULT_COST};

pub async fn create_admin_user(
    pool: &SqlitePool,
    username: &str,
    password: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let hashed = hash(password, DEFAULT_COST)?;

    sqlx::query(
        "INSERT INTO users (username, password_hash, role, is_active) VALUES (?, ?, ?, 1)",
    )
    .bind(username)
    .bind(hashed)
    .bind("admin")
    .execute(pool)
    .await?;

    Ok(())
}
