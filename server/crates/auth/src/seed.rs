// Utility to seed initial admin user for testing
use sqlx::SqlitePool;
use crate::shared::{UserRole, password, db};

/// Create a default admin user for testing
pub async fn create_default_admin(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    // Check if admin already exists
    if db::username_exists(pool, "admin").await? {
        println!("ℹ️  Default admin user already exists");
        return Ok(());
    }
    
    // Create admin user
    let password_hash = password::hash_password("admin123")?;
    
    db::create_user(
        pool,
        "admin",
        "admin@school.edu",
        &password_hash,
        UserRole::Admin,
    ).await?;
    
    println!("✅ Created default admin user:");
    println!("   Username: admin");
    println!("   Password: admin123");
    println!("   ⚠️  Please change this password after first login!");
    
    Ok(())
}
