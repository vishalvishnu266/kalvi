use sqlx::SqlitePool;
use bcrypt::{hash, DEFAULT_COST};
use crate::repositories::UserRepository;

pub async fn create_admin_user(
    pool: &SqlitePool,
    username: &str,
    password: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let hashed = hash(password, DEFAULT_COST)?;

    UserRepository::create_user(pool, username, &hashed, "admin").await?;

    Ok(())
}
