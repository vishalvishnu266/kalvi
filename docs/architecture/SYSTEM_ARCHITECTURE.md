# School ERP - System Architecture

> **Complete system overview for AI agents and developers**

## 🎯 System Overview

School ERP is a multi-tenant educational institution management system built with Rust, Axum, Hotwire Turbo, and SQLite.

### Key Characteristics
- **Multi-tenant:** One database per school/institution
- **Session-based auth:** Custom implementation, no external dependencies
- **Modern UI:** Hotwire Turbo + Bootstrap 5
- **Isolated data:** Complete tenant separation
- **Mobile-friendly:** Path-based routing works with all clients

---

## 🏗️ Architecture Layers

```
┌─────────────────────────────────────────┐
│          Browser / Mobile App           │
└─────────────────┬───────────────────────┘
                  │ HTTP/HTTPS
┌─────────────────▼───────────────────────┐
│         Axum Web Framework              │
│  ┌──────────────────────────────────┐   │
│  │   Tenant Middleware              │   │
│  │   (Resolves tenant from path)    │   │
│  └──────────────┬───────────────────┘   │
│  ┌──────────────▼───────────────────┐   │
│  │   Session Middleware             │   │
│  │   (Loads session from cookie)    │   │
│  └──────────────┬───────────────────┘   │
│  ┌──────────────▼───────────────────┐   │
│  │   Route Handlers                 │   │
│  │   (RequireAuth / OptionalAuth)   │   │
│  └──────────────┬───────────────────┘   │
└─────────────────┼───────────────────────┘
                  │
┌─────────────────▼───────────────────────┐
│       Database Layer (SQLite)           │
│  ┌────────────┐  ┌──────────────────┐   │
│  │ master.db  │  │ tenant_{slug}.db │   │
│  │            │  │                  │   │
│  │ - tenants  │  │ - users          │   │
│  │            │  │ - sessions       │   │
│  │            │  │ - students       │   │
│  │            │  │ - attendance     │   │
│  └────────────┘  └──────────────────┘   │
└─────────────────────────────────────────┘
```

---

## 🗄️ Database Architecture

### Master Database (`master.db`)

**Purpose:** Control plane - manages tenant registry

**Schema:**
```sql
CREATE TABLE tenants (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    slug TEXT NOT NULL UNIQUE,           -- URL identifier
    name TEXT NOT NULL,                  -- Full institution name
    contact_email TEXT NOT NULL,
    contact_phone TEXT NOT NULL,
    address TEXT NOT NULL,
    database_name TEXT NOT NULL UNIQUE,  -- tenant_{slug}
    is_active BOOLEAN NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL
);

CREATE INDEX idx_tenants_slug ON tenants(slug);
```

**Access Pattern:**
- Read: On every tenant route (`/t/{slug}/...`)
- Write: During tenant onboarding only
- Size: Small (one row per tenant)

### Tenant Database (`tenant_{slug}.db`)

**Purpose:** Data plane - all tenant-specific data

**Schema:**
```sql
-- User Management
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    role TEXT NOT NULL CHECK(role IN ('admin', 'teacher', 'staff')),
    is_active BOOLEAN NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL
);

-- Session Management
CREATE TABLE sessions (
    id TEXT PRIMARY KEY,                 -- UUID v4
    user_id INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    last_activity TEXT NOT NULL,
    expires_at TEXT NOT NULL,
    ip_address TEXT,                     -- Audit trail
    user_agent TEXT,                     -- Audit trail
    is_active BOOLEAN NOT NULL DEFAULT 1,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Student Management
CREATE TABLE students (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    email TEXT,
    phone TEXT,
    date_of_birth TEXT,
    enrollment_date TEXT NOT NULL DEFAULT (date('now')),
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
    -- More fields to be added
);

-- Future tables:
-- - classes
-- - subjects  
-- - grades
-- - attendance
-- - fees
```

**Access Pattern:**
- Read/Write: High frequency
- Isolation: Complete per tenant
- Size: Grows with tenant data

---

## 🔄 Request Flow

### 1. Tenant Resolution
```
Request: GET /t/demo-school/dashboard
    ↓
Tenant Middleware:
    - Extract slug: "demo-school"
    - Query master.db: SELECT * FROM tenants WHERE slug = ?
    - Load tenant pool: tenant_demo-school.db
    - Inject into request: Extension(tenant_pool)
    ↓
Continue to next middleware
```

### 2. Session Loading
```
Session Middleware:
    - Extract cookie: session_id
    - Query tenant DB: SELECT * FROM sessions WHERE id = ?
    - Validate: is_active AND expires_at > now()
    - Inject into request: Extension(session)
    ↓
Continue to handler
```

### 3. Authentication Check
```
Handler with RequireAuth:
    - Extract session from request
    - Load user: SELECT * FROM users WHERE id = session.user_id
    - Check is_active
    - Provide: RequireAuth(user)
    ↓
Handler logic executes
```

### 4. Response
```
Handler returns:
    - HTML (Maud template)
    - OR JSON (API response)
    - OR Redirect
    ↓
Middleware may:
    - Set cookies (login)
    - Clear cookies (logout)
    ↓
Send to client
```

---

## 📦 Crate Structure

```
server/crates/
├── server/          # Application entry point
│   └── main.rs      # Server setup, middleware stack
│
├── shared/          # Shared utilities
│   ├── db.rs        # TenantDatabaseManager
│   ├── middleware.rs # Tenant resolution middleware
│   └── ...
│
├── auth/            # Authentication & sessions
│   ├── session.rs   # SessionManager
│   ├── login.rs     # Login handlers
│   ├── logout.rs    # Logout handlers
│   ├── dashboard.rs # User dashboard
│   └── middleware.rs # RequireAuth extractors
│
├── tenant/          # Tenant management
│   ├── onboarding.rs # Tenant creation
│   └── home.rs      # Landing page
│
└── student/         # Student features
    ├── shared.rs    # Student models
    └── ...          # Future features
```

---

## 🔐 Security Architecture

### Session Security
```
Session Creation (Login):
1. Validate credentials
2. Generate UUID v4 session ID
3. Store in tenant DB with metadata
4. Set HTTP-only cookie
5. Cookie attributes:
   - HttpOnly: true (no JavaScript access)
   - SameSite: Lax (CSRF protection)
   - Secure: true (HTTPS in production)
   - Max-Age: 24 hours
```

### Password Security
```
Storage:
- bcrypt hash (cost 12)
- Salt automatically handled
- Stored in users.password_hash

Verification:
- Constant-time comparison
- No password ever stored in plain text
```

### Tenant Isolation
```
Database Level:
- Separate SQLite file per tenant
- No shared tables
- No cross-tenant queries possible

Application Level:
- Tenant slug from URL
- Validated against master DB
- Pool injection per request
- No global state
```

---

## 🚀 Scalability Considerations

### Current (Single Server)
```
Capacity:
- 100+ tenants
- 1,000+ users per tenant
- 10,000+ requests/minute
- SQLite handles this easily
```

### Future (Multi-Server)
```
Options:
1. Move tenant DBs to PostgreSQL
2. Distribute tenants across servers
3. Use read replicas
4. Add caching layer
```

---

## 🔄 Data Flow Examples

### Login Flow
```
POST /t/demo-school/login
  ↓
1. Tenant middleware → load tenant DB
2. Validate credentials
3. Create session in tenant DB
4. Set session cookie
5. Redirect to dashboard
  ↓
GET /t/demo-school/dashboard
  ↓
1. Tenant middleware → load tenant DB
2. Session middleware → load session
3. RequireAuth → load user
4. Render dashboard
5. Return HTML
```

### Protected Resource Access
```
GET /t/demo-school/students
  ↓
1. Tenant middleware → verify tenant exists
2. Session middleware → load session from cookie
3. RequireAuth extractor:
   - No session? → Redirect to login
   - Invalid session? → Redirect to login
   - Inactive user? → Redirect to login
   - Valid? → Provide user to handler
4. Handler checks user.role
5. Query students from tenant DB
6. Render HTML with Turbo
```

---

## 🎨 UI Architecture

### Hotwire Turbo
```
Traditional:
  Click → Full page reload → White screen → New page

With Turbo:
  Click → Partial update → No reload → Smooth transition
```

### Turbo Frames
```html
<!-- Replace only this frame on navigation -->
<turbo-frame id="student_list">
  <a href="/t/{slug}/students?page=2">Next</a>
  <!-- Only content inside frame updates -->
</turbo-frame>
```

### Turbo Streams
```rust
// Multiple updates in one response
turbo-stream action="replace" target="item_1" { ... }
turbo-stream action="append" target="notifications" { ... }
turbo-stream action="remove" target="old_item" { ... }
```

---

## 📊 Performance Characteristics

### Database
- **Read latency:** < 1ms (in-process SQLite)
- **Write latency:** < 5ms (WAL mode)
- **Concurrent reads:** Unlimited
- **Concurrent writes:** Serialized (but fast)

### Session Operations
- **Session lookup:** < 1ms (indexed query)
- **Session create:** < 5ms (INSERT + cookie set)
- **Session validate:** < 1ms (in-memory after load)

### Request Handling
- **Tenant resolution:** ~1ms (indexed lookup)
- **Session loading:** ~1ms (indexed lookup)  
- **Total overhead:** < 5ms per request
- **Handler time:** Varies by feature

---

## 🛡️ Error Handling

### Database Errors
```rust
match db::get_user(...).await {
    Ok(Some(user)) => /* use user */,
    Ok(None) => /* not found */,
    Err(e) => /* database error */,
}
```

### Authentication Errors
```rust
RequireAuth extractor:
- No session → Redirect to login
- Expired session → Redirect to login
- Inactive user → Redirect to login with error
- Database error → 500 error
```

### Tenant Errors
```rust
Tenant middleware:
- Tenant not found → 404
- Tenant inactive → 403
- Database error → 500
```

---

## 📝 Configuration

### Environment Variables (Future)
```bash
DATABASE_PATH=./data
SESSION_DURATION=86400
BCRYPT_COST=12
SERVER_PORT=3000
```

### Compile-Time Constants
```rust
const COOKIE_NAME: &str = "session_id";
const SESSION_DURATION_SECS: u64 = 24 * 3600;
const MAX_SESSIONS_PER_USER: usize = 5;
```

---

## 🔮 Future Enhancements

### Planned
- [ ] Email notifications
- [ ] File uploads (student photos, documents)
- [ ] Report generation (PDF)
- [ ] Data export (CSV, Excel)
- [ ] Audit logging

### Possible
- [ ] Multi-language support
- [ ] OAuth integration
- [ ] Two-factor authentication
- [ ] Real-time notifications (WebSocket)
- [ ] Mobile app (using same backend)

---

**See also:**
- [Multi-Tenant Design](MULTI_TENANT_DESIGN.md)
- [Session System](SESSION_SYSTEM.md)
- [Database Schema](DATABASE_SCHEMA.md)
