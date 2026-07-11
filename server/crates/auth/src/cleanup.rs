// Session cleanup task
// Runs periodically to remove expired sessions

use sqlx::SqlitePool;
use std::time::Duration;
use tokio::time::interval;

use crate::session::SessionManager;

/// Start background cleanup task
/// Removes expired sessions every hour
pub fn start_cleanup_task(pool: SqlitePool) {
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(3600)); // 1 hour
        
        loop {
            interval.tick().await;
            
            let session_manager = SessionManager::new(pool.clone());
            
            match session_manager.cleanup_expired_sessions().await {
                Ok(count) => {
                    if count > 0 {
                        println!("🧹 Cleaned up {} expired sessions", count);
                    }
                }
                Err(e) => {
                    eprintln!("❌ Session cleanup error: {}", e);
                }
            }
        }
    });
}

/// Initialize session tables for a tenant database
pub async fn init_session_tables(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let session_manager = SessionManager::new(pool.clone());
    session_manager.create_table().await
}
