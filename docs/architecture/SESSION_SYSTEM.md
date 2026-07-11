# Custom Session System Design

## Why Custom Sessions?

### ✅ Advantages Over tower-sessions

1. **Full Control** - Complete control over session lifecycle
2. **Tenant Isolation** - Sessions stored in tenant DB (true isolation)
3. **Admin Features** - View/manage all active sessions per tenant
4. **Audit Trail** - Track login history, IP addresses, user agents
5. **Security** - Force logout, session revocation, concurrent session limits
6. **Performance** - Optimized queries for our specific use case
7. **Simplicity** - No external dependency, easier to debug

### 🎯 Requirements

- ✅ Persist across server restarts (SQLite)
- ✅ Automatic expiry and cleanup
- ✅ Secure session IDs (cryptographically random)
- ✅ HTTP-only, secure cookies
- ✅ CSRF protection
- ✅ Session metadata (IP, user agent, last activity)
- ✅ Admin control panel to view/revoke sessions

## Database Schema

### Sessions Table (in each tenant DB)

```sql
CREATE TABLE sessions (
    id TEXT PRIMARY KEY,              -- UUID v4 (cryptographically random)
    user_id INTEGER NOT NULL,         -- Foreign key to users.id
    created_at TEXT NOT NULL,         -- ISO 8601 timestamp
    last_activity TEXT NOT NULL,      -- ISO 8601 timestamp
    expires_at TEXT NOT NULL,         -- ISO 8601 timestamp
    ip_address TEXT,                  -- Client IP (for audit)
    user_agent TEXT,                  -- Browser/client info
    is_active BOOLEAN DEFAULT 1,      -- Can be revoked by admin
    
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_sessions_user_id ON sessions(user_id);
CREATE INDEX idx_sessions_expires_at ON sessions(expires_at);
CREATE INDEX idx_sessions_is_active ON sessions(is_active);
```

### Session History Table (optional, for audit)

```sql
CREATE TABLE session_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id TEXT NOT NULL,
    user_id INTEGER NOT NULL,
    action TEXT NOT NULL,             -- 'created', 'renewed', 'expired', 'revoked'
    created_at TEXT NOT NULL,
    ip_address TEXT,
    user_agent TEXT,
    
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
```

## Implementation Design

### 1. Session Manager

```rust
pub struct SessionManager {
    pool: SqlitePool,
}

impl SessionManager {
    // Create new session
    async fn create_session(
        &self,
        user_id: i64,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<Session, Error>;
    
    // Get session by ID
    async fn get_session(&self, session_id: &str) -> Result<Option<Session>, Error>;
    
    // Update last activity
    async fn touch_session(&self, session_id: &str) -> Result<(), Error>;
    
    // Revoke session (logout)
    async fn revoke_session(&self, session_id: &str) -> Result<(), Error>;
    
    // Revoke all sessions for a user
    async fn revoke_user_sessions(&self, user_id: i64) -> Result<(), Error>;
    
    // Get all active sessions for a user
    async fn get_user_sessions(&self, user_id: i64) -> Result<Vec<Session>, Error>;
    
    // Get all active sessions (for admin)
    async fn get_all_active_sessions(&self) -> Result<Vec<SessionWithUser>, Error>;
    
    // Cleanup expired sessions
    async fn cleanup_expired_sessions(&self) -> Result<u64, Error>;
}
```

### 2. Session Model

```rust
pub struct Session {
    pub id: String,              // UUID
    pub user_id: i64,
    pub created_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub is_active: bool,
}

pub struct SessionWithUser {
    pub session: Session,
    pub user: User,
}
```

### 3. Cookie Management

```rust
const COOKIE_NAME: &str = "session_id";
const SESSION_DURATION: Duration = Duration::from_secs(24 * 3600); // 24 hours

// Set session cookie
fn set_session_cookie(response: &mut Response, session_id: &str) {
    let cookie = Cookie::build(COOKIE_NAME, session_id)
        .http_only(true)          // Not accessible via JavaScript
        .secure(true)             // HTTPS only (in production)
        .same_site(SameSite::Lax) // CSRF protection
        .path("/")
        .max_age(SESSION_DURATION)
        .finish();
    
    response.headers_mut().insert(
        SET_COOKIE,
        cookie.to_string().parse().unwrap()
    );
}

// Clear session cookie
fn clear_session_cookie(response: &mut Response) {
    let cookie = Cookie::build(COOKIE_NAME, "")
        .http_only(true)
        .secure(true)
        .path("/")
        .max_age(Duration::ZERO)
        .finish();
    
    response.headers_mut().insert(
        SET_COOKIE,
        cookie.to_string().parse().unwrap()
    );
}
```

### 4. Middleware Flow

```rust
async fn session_middleware(
    Extension(pool): Extension<SqlitePool>,
    mut req: Request,
    next: Next,
) -> Response {
    // 1. Extract session ID from cookie
    let session_id = extract_session_id(&req);
    
    // 2. Load session from database
    let session_manager = SessionManager::new(pool);
    
    if let Some(id) = session_id {
        if let Ok(Some(session)) = session_manager.get_session(&id).await {
            // 3. Validate session
            if session.is_active && session.expires_at > Utc::now() {
                // 4. Update last activity (background task)
                tokio::spawn(async move {
                    let _ = session_manager.touch_session(&id).await;
                });
                
                // 5. Inject session into request
                req.extensions_mut().insert(session);
            }
        }
    }
    
    next.run(req).await
}
```

### 5. Background Cleanup Task

```rust
// Cleanup expired sessions every hour
async fn start_cleanup_task(pool: SqlitePool) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(3600));
        
        loop {
            interval.tick().await;
            
            let session_manager = SessionManager::new(pool.clone());
            match session_manager.cleanup_expired_sessions().await {
                Ok(count) => {
                    if count > 0 {
                        println!("🧹 Cleaned up {} expired sessions", count);
                    }
                }
                Err(e) => eprintln!("❌ Session cleanup error: {}", e),
            }
        }
    });
}
```

## Security Features

### 1. Session ID Generation

```rust
use uuid::Uuid;

fn generate_session_id() -> String {
    Uuid::new_v4().to_string()
}
```

### 2. CSRF Protection

Option 1: SameSite Cookie (already included)
Option 2: CSRF Token (for forms)

```rust
// Generate CSRF token from session
fn generate_csrf_token(session_id: &str) -> String {
    use sha2::{Sha256, Digest};
    
    let mut hasher = Sha256::new();
    hasher.update(session_id);
    hasher.update("secret_key"); // Use app secret
    format!("{:x}", hasher.finalize())
}
```

### 3. Session Fixation Prevention

```rust
// On login, create new session (don't reuse existing)
async fn login_user(
    session_manager: &SessionManager,
    user_id: i64,
    ip: Option<String>,
    ua: Option<String>,
) -> Result<Session, Error> {
    // Always create fresh session on login
    session_manager.create_session(user_id, ip, ua).await
}
```

### 4. Concurrent Session Limit

```rust
const MAX_SESSIONS_PER_USER: usize = 5;

async fn enforce_session_limit(
    session_manager: &SessionManager,
    user_id: i64,
) -> Result<(), Error> {
    let sessions = session_manager.get_user_sessions(user_id).await?;
    
    if sessions.len() >= MAX_SESSIONS_PER_USER {
        // Revoke oldest session
        if let Some(oldest) = sessions.first() {
            session_manager.revoke_session(&oldest.id).await?;
        }
    }
    
    Ok(())
}
```

## Admin Control Panel

### Active Sessions View

```rust
// GET /t/{slug}/admin/sessions
async fn list_active_sessions(
    RequireAuth(user): RequireAuth,
    Extension(pool): Extension<SqlitePool>,
) -> Result<Html<String>, StatusCode> {
    // Only admins can view
    if user.role != UserRole::Admin {
        return Err(StatusCode::FORBIDDEN);
    }
    
    let session_manager = SessionManager::new(pool);
    let sessions = session_manager.get_all_active_sessions().await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Html(render_sessions_page(&sessions)))
}

// POST /t/{slug}/admin/sessions/{id}/revoke
async fn revoke_session(
    RequireAuth(user): RequireAuth,
    Path((tenant_slug, session_id)): Path<(String, String)>,
    Extension(pool): Extension<SqlitePool>,
) -> Result<Redirect, StatusCode> {
    if user.role != UserRole::Admin {
        return Err(StatusCode::FORBIDDEN);
    }
    
    let session_manager = SessionManager::new(pool);
    session_manager.revoke_session(&session_id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Redirect::to(&format!("/t/{}/admin/sessions", tenant_slug)))
}
```

## Migration from tower-sessions

### Step 1: Remove Dependencies

```toml
# Remove from Cargo.toml
# tower-sessions = "0.12"
# tower-sessions-sqlx-store = { version = "0.12", features = ["sqlite"] }

# Add
uuid = { version = "1.6", features = ["v4", "serde"] }
cookie = "0.18"
```

### Step 2: Create Migration

```rust
// Run this when initializing tenant database
async fn create_sessions_table(pool: &SqlitePool) -> Result<(), sqlx::Error> {
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
            is_active BOOLEAN DEFAULT 1,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
        )
        "#
    ).execute(pool).await?;
    
    // Create indexes
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_sessions_user_id ON sessions(user_id)")
        .execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_sessions_expires_at ON sessions(expires_at)")
        .execute(pool).await?;
    
    Ok(())
}
```

## Benefits Summary

✅ **Full Control** - Own the session lifecycle
✅ **Tenant Isolation** - Sessions in tenant DB
✅ **Admin Features** - View/revoke sessions
✅ **Audit Trail** - Track all session activity
✅ **Security** - CSRF, session limits, secure cookies
✅ **Performance** - Optimized for our use case
✅ **Simplicity** - No external dependencies
✅ **Flexibility** - Easy to extend with custom features

## Recommended Approach

**I recommend implementing the custom session system** because:

1. You want admin control panel (view active sessions)
2. True tenant isolation (sessions in tenant DB)
3. Simpler codebase (no tower-sessions magic)
4. Better for production ERP (audit, security)
5. Easier to extend with custom features

Shall I implement this custom session system?
