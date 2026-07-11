// Session cleanup task
// Runs periodically to remove expired sessions across all tenant databases.

use sqlx::SqlitePool;
use std::sync::Arc;
use std::time::Duration;
use tokio::task::JoinHandle;
use tokio::time::interval;

use ::shared::TenantDatabaseManager;

use crate::session::SessionManager;

/// How often the background task wakes up to prune expired sessions.
const CLEANUP_INTERVAL_SECS: u64 = 3600; // 1 hour

/// Start a background task that periodically removes expired sessions from
/// every active tenant database.
///
/// The task automatically discovers tenants by reading the master `tenants`
/// table on each tick, so newly-onboarded tenants are picked up without
/// restarting the server.
///
/// The returned [`JoinHandle`] can be dropped safely; the task will continue
/// running for the lifetime of the process.
pub fn start_cleanup_task(db_manager: Arc<TenantDatabaseManager>) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut ticker = interval(Duration::from_secs(CLEANUP_INTERVAL_SECS));
        // The first tick fires immediately - skip it so we don't run during
        // startup before tenant pools may be warmed up.
        ticker.tick().await;

        loop {
            ticker.tick().await;
            run_cleanup_cycle(&db_manager).await;
        }
    })
}

/// Perform a single cleanup pass over every active tenant.
async fn run_cleanup_cycle(db_manager: &TenantDatabaseManager) {
    let master_pool = db_manager.get_master_pool();

    let tenants: Vec<String> = match sqlx::query_scalar::<_, String>(
        "SELECT database_name FROM tenants WHERE is_active = 1",
    )
    .fetch_all(&master_pool)
    .await
    {
        Ok(rows) => rows,
        Err(e) => {
            eprintln!("❌ Session cleanup: failed to list tenants: {}", e);
            return;
        }
    };

    let mut total_removed: u64 = 0;
    for database_name in tenants {
        match db_manager.get_tenant_pool(&database_name).await {
            Ok(pool) => {
                let manager = SessionManager::new(pool);
                match manager.cleanup_expired_sessions().await {
                    Ok(count) => total_removed += count,
                    Err(e) => eprintln!(
                        "❌ Session cleanup failed for tenant '{}': {}",
                        database_name, e
                    ),
                }
            }
            Err(e) => eprintln!(
                "❌ Session cleanup: could not open pool for '{}': {}",
                database_name, e
            ),
        }
    }

    if total_removed > 0 {
        println!("🧹 Removed {} expired session(s) across tenants", total_removed);
    }
}

/// Initialize session tables for a tenant database
pub async fn init_session_tables(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let session_manager = SessionManager::new(pool.clone());
    session_manager.create_table().await
}
