# Tenant-Scoped Authentication Architecture

## ✅ Corrected Architecture

### Database Separation

**Master Database (`master.db`)**
- ✅ Tenant registry (tenants table)
- ✅ Session storage (tower_sessions table)
- ❌ NO user data
- ❌ NO tenant-specific ERP data

**Tenant Databases (`tenant_{slug}.db`)**
- ✅ Users table (tenant-specific users)
- ✅ Students table
- ✅ All ERP-related data
- ❌ NO sessions (sessions are in master DB but tenant-aware)

### Why Sessions in Master DB?

**Decision**: Sessions stored in master DB, but contain tenant slug

**Rationale**:
1. **Single Cookie**: One session cookie works across all tenant contexts
2. **Simpler Architecture**: No need for per-tenant session stores
3. **Tenant Isolation**: Session contains tenant slug, validates on each request
4. **Security**: User belongs to ONE tenant, stored in session

**Session Data**:
```rust
{
    "user_id": 1,           // User ID from tenant database
    "tenant_slug": "demo-school"  // Which tenant this user belongs to
}
```

### URL Structure

All tenant-specific routes use `/t/{slug}/` prefix:

**Control Plane (Master DB)**:
- `GET /` - Home page
- `GET /onboard` - Tenant onboarding
- `POST /onboard` - Create tenant

**Tenant-Scoped (Tenant DB + Session)**:
- `GET /t/{slug}/login` - Login for specific tenant
- `POST /t/{slug}/login` - Process login
- `GET /t/{slug}/dashboard` - User dashboard
- `GET /t/{slug}/logout` - Logout

### Authentication Flow

#### 1. Login Flow
```
User visits: /t/demo-school/login
    ↓
Middleware extracts tenant: demo-school
    ↓
Looks up tenant in master DB
    ↓
Gets tenant database: tenant_demo-school.db
    ↓
User submits credentials
    ↓
Validates against users table in tenant DB
    ↓
On success: Store in session:
    - user_id: 1
    - tenant_slug: demo-school
    ↓
Redirect to: /t/demo-school/dashboard
```

#### 2. Protected Route Access
```
User visits: /t/demo-school/dashboard
    ↓
Middleware extracts tenant from URL: demo-school
    ↓
Loads session from master DB
    ↓
Checks session contains:
    - user_id exists?
    - tenant_slug matches URL tenant?
    ↓
If yes: Load user from tenant DB
    ↓
If user active: Allow access
    ↓
If no: Redirect to /t/demo-school/login
```

### Security Model

**Tenant Isolation**:
- ✅ Users cannot access other tenants
- ✅ Session validates tenant slug matches URL
- ✅ Each tenant has isolated database
- ✅ No cross-tenant data leakage

**Session Validation**:
```rust
// In RequireAuth middleware
let session_tenant = session.get("tenant_slug");
let url_tenant = path.extract_tenant();

if session_tenant != url_tenant {
    // Redirect to correct tenant login
    return Redirect::to(&format!("/t/{}/login", url_tenant));
}
```

## Implementation Details

### Master DB Schema

```sql
-- Tenants (control plane)
CREATE TABLE tenants (
    id INTEGER PRIMARY KEY,
    slug TEXT UNIQUE,
    name TEXT,
    database_name TEXT,
    ...
);

-- Sessions (managed by tower-sessions)
CREATE TABLE tower_sessions (
    id TEXT PRIMARY KEY,
    data BLOB,
    expiry_date INTEGER
);
```

### Tenant DB Schema

```sql
-- Users (per tenant)
CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    username TEXT UNIQUE,
    email TEXT,
    password_hash TEXT,
    role TEXT,
    ...
);

-- Students (per tenant)
CREATE TABLE students (
    id INTEGER PRIMARY KEY,
    name TEXT,
    ...
);
```

### Middleware Stack

```
Request: /t/demo-school/dashboard
    ↓
[1] Session Layer (global)
    - Loads session from master DB
    - Injects Session into request
    ↓
[2] Tenant Middleware
    - Extracts tenant slug from URL
    - Looks up tenant in master DB
    - Loads tenant database pool
    - Injects tenant pool + context
    ↓
[3] RequireAuth Extractor
    - Gets session
    - Gets tenant context
    - Validates user belongs to tenant
    - Loads user from tenant DB
    ↓
Handler receives:
    - RequireAuth(user)
    - Extension(tenant_context)
    - Extension(tenant_pool)
```

## Benefits of This Architecture

### ✅ Pros
1. **Single Session Cookie**: Users don't need multiple cookies
2. **Tenant Isolation**: Complete data separation
3. **Scalability**: Each tenant can be moved to separate DB server
4. **Security**: Cross-tenant access impossible
5. **Simple Routing**: Clear `/t/{slug}/` pattern

### ⚠️ Considerations
1. Sessions in master DB (acceptable for this use case)
2. Session validates tenant slug (adds small overhead)
3. Users belong to ONE tenant only (by design)

## Migration Notes

### What Changed

**Before**:
- ❌ Auth routes were global (`/login`, `/dashboard`)
- ❌ Sessions in master DB, no tenant validation
- ❌ Users could be shared across tenants

**After**:
- ✅ Auth routes are tenant-scoped (`/t/{slug}/login`)
- ✅ Sessions contain tenant slug
- ✅ Users belong to specific tenant
- ✅ Complete tenant isolation

### Testing

```bash
# 1. Onboard tenant
curl POST http://localhost:3000/onboard
  -d slug=school-a

# 2. Seed users in tenant DB
sqlite3 tenant_school-a.db < SEED_ADMIN.sql

# 3. Login to tenant
curl POST http://localhost:3000/t/school-a/login
  -d username=admin
  -d password=admin123

# 4. Access dashboard
GET http://localhost:3000/t/school-a/dashboard
```

## Future Enhancements

### Multi-Tenant Users (Optional)
If you need users to access multiple tenants:
```sql
-- Add to master DB
CREATE TABLE user_tenant_access (
    user_id INTEGER,
    tenant_id INTEGER,
    role TEXT,
    PRIMARY KEY (user_id, tenant_id)
);
```

Then modify session to:
```rust
{
    "user_id": 1,
    "accessible_tenants": ["school-a", "school-b"],
    "current_tenant": "school-a"
}
```

---

**Status**: ✅ Tenant-scoped authentication fully implemented!
