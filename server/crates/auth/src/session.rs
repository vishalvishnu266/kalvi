// Custom Session Management
// Simple, direct SQLite storage with full control

use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};

/// Session model
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Session {
    pub id: String,
    pub user_id: i64,
    pub created_at: String,
    pub last_activity: String,
    pub expires_at: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub is_active: bool,
}

impl Session {
    /// Check if session is valid (not expired and active)
    pub fn is_valid(&self) -> bool {
        if !self.is_active {
            return false;
        }
        
        if let Ok(expires) = DateTime::parse_from_rfc3339(&self.expires_at) {
            expires.with_timezone(&Utc) > Utc::now()
        } else {
            false
        }
    }
}

/// Session with user information (for admin panel)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionWithUser {
    pub session: Session,
    pub username: String,
    pub email: String,
    pub role: String,
}

/// Session configuration
#[derive(Debug, Clone)]
pub struct SessionConfig {
    pub duration: Duration,
    pub update_interval: Duration,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            duration: Duration::hours(24),        // 24 hours
            update_interval: Duration::minutes(5), // Update last_activity every 5 min
        }
    }
}

/// Session Manager - handles all session operations
#[derive(Clone)]
pub struct SessionManager {
    pool: SqlitePool,
    config: SessionConfig,
}

impl SessionManager {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            pool,
            config: SessionConfig::default(),
        }
    }
    
    pub fn with_config(pool: SqlitePool, config: SessionConfig) -> Self {
        Self { pool, config }
    }
    
    /// Create sessions table in database
    pub async fn create_table(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS sessions (
                id TEXT PRIMARY KEY,
                user_id INTEGER NOT NULL,
                created_at TEXT NOT NULL,
                last_activity TEXT NOT NULL,
                expires_at TEXT NOT NULL,
                ip_address TEXT,
                user_agent TEXT,
                is_active BOOLEAN NOT NULL DEFAULT 1,
                FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
            )
            "#
        )
        .execute(&self.pool)
        .await?;
        
        // Create indexes for performance
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_sessions_user_id ON sessions(user_id)")
            .execute(&self.pool)
            .await?;
        
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_sessions_expires_at ON sessions(expires_at)")
            .execute(&self.pool)
            .await?;
        
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_sessions_is_active ON sessions(is_active)")
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }
    
    /// Create a new session
    pub async fn create_session(
        &self,
        user_id: i64,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<Session, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let expires_at = now + self.config.duration;
        
        let session = Session {
            id: id.clone(),
            user_id,
            created_at: now.to_rfc3339(),
            last_activity: now.to_rfc3339(),
            expires_at: expires_at.to_rfc3339(),
            ip_address,
            user_agent,
            is_active: true,
        };
        
        sqlx::query(
            r#"
            INSERT INTO sessions (id, user_id, created_at, last_activity, expires_at, ip_address, user_agent, is_active)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&session.id)
        .bind(session.user_id)
        .bind(&session.created_at)
        .bind(&session.last_activity)
        .bind(&session.expires_at)
        .bind(&session.ip_address)
        .bind(&session.user_agent)
        .bind(session.is_active)
        .execute(&self.pool)
        .await?;
        
        Ok(session)
    }
    
    /// Get session by ID
    pub async fn get_session(&self, session_id: &str) -> Result<Option<Session>, sqlx::Error> {
        sqlx::query_as::<_, Session>(
            "SELECT id, user_id, created_at, last_activity, expires_at, ip_address, user_agent, is_active 
             FROM sessions WHERE id = ?"
        )
        .bind(session_id)
        .fetch_optional(&self.pool)
        .await
    }
    
    /// Update last activity timestamp
    pub async fn touch_session(&self, session_id: &str) -> Result<(), sqlx::Error> {
        let now = Utc::now().to_rfc3339();
        
        sqlx::query("UPDATE sessions SET last_activity = ? WHERE id = ?")
            .bind(now)
            .bind(session_id)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }
    
    /// Revoke a session (logout)
    pub async fn revoke_session(&self, session_id: &str) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE sessions SET is_active = 0 WHERE id = ?")
            .bind(session_id)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }
    
    /// Revoke all sessions for a user
    pub async fn revoke_user_sessions(&self, user_id: i64) -> Result<u64, sqlx::Error> {
        let result = sqlx::query("UPDATE sessions SET is_active = 0 WHERE user_id = ?")
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        
        Ok(result.rows_affected())
    }
    
    /// Get all active sessions for a user
    pub async fn get_user_sessions(&self, user_id: i64) -> Result<Vec<Session>, sqlx::Error> {
        sqlx::query_as::<_, Session>(
            "SELECT id, user_id, created_at, last_activity, expires_at, ip_address, user_agent, is_active 
             FROM sessions 
             WHERE user_id = ? AND is_active = 1 
             ORDER BY last_activity DESC"
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
    }
    
    /// Get all active sessions (for admin panel)
    pub async fn get_all_active_sessions(&self) -> Result<Vec<SessionWithUser>, sqlx::Error> {
        sqlx::query_as::<_, SessionWithUser>(
            r#"
            SELECT 
                s.id,
                s.user_id,
                s.created_at,
                s.last_activity,
                s.expires_at,
                s.ip_address,
                s.user_agent,
                s.is_active,
                u.username,
                u.email,
                u.role
            FROM sessions s
            JOIN users u ON s.user_id = u.id
            WHERE s.is_active = 1
            ORDER BY s.last_activity DESC
            "#
        )
        .fetch_all(&self.pool)
        .await
    }
    
    /// Cleanup expired sessions
    pub async fn cleanup_expired_sessions(&self) -> Result<u64, sqlx::Error> {
        let now = Utc::now().to_rfc3339();
        
        let result = sqlx::query("DELETE FROM sessions WHERE expires_at < ?")
            .bind(now)
            .execute(&self.pool)
            .await?;
        
        Ok(result.rows_affected())
    }
    
    /// Count active sessions
    pub async fn count_active_sessions(&self) -> Result<i64, sqlx::Error> {
        let (count,): (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM sessions WHERE is_active = 1"
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(count)
    }
    
    /// Count active sessions for a user
    pub async fn count_user_sessions(&self, user_id: i64) -> Result<i64, sqlx::Error> {
        let (count,): (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM sessions WHERE user_id = ? AND is_active = 1"
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(count)
    }
    
    /// Enforce session limit per user (revoke oldest if limit exceeded)
    pub async fn enforce_session_limit(&self, user_id: i64, max_sessions: usize) -> Result<(), sqlx::Error> {
        let sessions = self.get_user_sessions(user_id).await?;
        
        if sessions.len() > max_sessions {
            // Revoke oldest sessions
            let to_revoke = sessions.len() - max_sessions;
            for session in sessions.iter().rev().take(to_revoke) {
                self.revoke_session(&session.id).await?;
            }
        }
        
        Ok(())
    }
}

// Custom FromRow for SessionWithUser
impl sqlx::FromRow<'_, sqlx::sqlite::SqliteRow> for SessionWithUser {
    fn from_row(row: &sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        use sqlx::Row;
        
        Ok(SessionWithUser {
            session: Session {
                id: row.try_get("id")?,
                user_id: row.try_get("user_id")?,
                created_at: row.try_get("created_at")?,
                last_activity: row.try_get("last_activity")?,
                expires_at: row.try_get("expires_at")?,
                ip_address: row.try_get("ip_address")?,
                user_agent: row.try_get("user_agent")?,
                is_active: row.try_get("is_active")?,
            },
            username: row.try_get("username")?,
            email: row.try_get("email")?,
            role: row.try_get("role")?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_session_validation() {
        let mut session = Session {
            id: "test".to_string(),
            user_id: 1,
            created_at: Utc::now().to_rfc3339(),
            last_activity: Utc::now().to_rfc3339(),
            expires_at: (Utc::now() + Duration::hours(1)).to_rfc3339(),
            ip_address: None,
            user_agent: None,
            is_active: true,
        };
        
        assert!(session.is_valid());
        
        // Test expired session
        session.expires_at = (Utc::now() - Duration::hours(1)).to_rfc3339();
        assert!(!session.is_valid());
        
        // Test inactive session
        session.expires_at = (Utc::now() + Duration::hours(1)).to_rfc3339();
        session.is_active = false;
        assert!(!session.is_valid());
    }
}
