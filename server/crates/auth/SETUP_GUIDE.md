# Authentication System Setup Guide

## Initial Setup

### 1. Create First Admin User

Since the authentication system is new, you need to create an admin user for each tenant.

**Option A: Using SQLite directly**
```bash
# First, onboard a tenant (e.g., "demo-school")
# This creates: tenant_demo-school.db

# Then add admin user to that database
sqlite3 tenant_demo-school.db

INSERT INTO users (username, email, password_hash, role) 
VALUES ('admin', 'admin@demo.edu', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5yvU1MQl8xvLu', 'admin');

-- Password hash above is for "admin123"
-- Username: admin
-- Password: admin123
```

**Option B: Create a seed endpoint (recommended for development)**

Add this temporary route to your tenant onboarding success page to seed the first admin user.

### 2. Login Flow

1. **Onboard Tenant**: `http://localhost:3000/onboard`
   - Create tenant with slug: `demo-school`

2. **Create Admin User**: Use SQLite or seed function

3. **Login**: `http://localhost:3000/login`
   - Username: `admin`
   - Password: `admin123`

4. **Dashboard**: Automatically redirected to `/dashboard`

## How It Works

### Session Management

- **Storage**: SQLite sessions table in master database
- **Expiry**: 24 hours of inactivity
- **Cookie-based**: Secure HTTP-only cookies

### Password Security

- **Hashing**: bcrypt with cost 12
- **Verification**: Constant-time comparison
- **Storage**: Only hashed passwords stored

### Routes

#### Control Plane Routes (No Auth Required)
- `GET /` - Home page
- `GET /onboard` - Tenant onboarding
- `POST /onboard` - Submit onboarding

#### Tenant-Scoped Routes (Require `/t/{slug}/` prefix)
- `GET /t/{slug}/login` - Login page
- `POST /t/{slug}/login` - Submit login
- `GET /t/{slug}/dashboard` - User dashboard (requires auth)
- `GET /t/{slug}/logout` - Logout

### Middleware

#### `RequireAuth`
Extracts authenticated user or redirects to login.

```rust
async fn protected_handler(RequireAuth(user): RequireAuth) -> impl IntoResponse {
    // user is guaranteed to be authenticated
    Html(format!("Hello, {}", user.username))
}
```

#### `OptionalAuth`
Provides user if authenticated, None otherwise.

```rust
async fn optional_handler(OptionalAuth(user): OptionalAuth) -> impl IntoResponse {
    match user {
        Some(u) => format!("Hello, {}", u.username),
        None => "Hello, guest".to_string(),
    }
}
```

## User Roles

Three built-in roles:
- **Admin**: Full system access
- **Teacher**: Teaching and class management
- **Staff**: Limited access

Check roles in handlers:
```rust
async fn admin_only(RequireAuth(user): RequireAuth) -> impl IntoResponse {
    if user.role != UserRole::Admin {
        return StatusCode::FORBIDDEN.into_response();
    }
    // Admin-only logic
}
```

## Database Schema

### Users Table (in each tenant database)

```sql
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    role TEXT NOT NULL CHECK(role IN ('admin', 'teacher', 'staff')),
    is_active BOOLEAN NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);
```

### Sessions Table (in master database)

Automatically created by tower-sessions-sqlx-store.

## Testing

### Test Login
```bash
# 1. Onboard tenant
curl -X POST http://localhost:3000/onboard \
  -d "slug=test-school" \
  -d "name=Test School" \
  -d "contact_email=admin@test.edu" \
  -d "contact_phone=1234567890" \
  -d "address=123 Main St"

# 2. Create admin user (use sqlite3)

# 3. Test login
curl -X POST http://localhost:3000/login \
  -d "username=admin" \
  -d "password=admin123" \
  -c cookies.txt

# 4. Access protected route
curl http://localhost:3000/dashboard \
  -b cookies.txt
```

## Security Best Practices

### ⚠️ Important for Production

1. **Change Default Password**
   - Never use "admin123" in production
   - Require password change on first login

2. **HTTPS Only**
   - Enable secure cookies in production
   - Use HTTPS for all auth routes

3. **Session Security**
   - Rotate session IDs after login
   - Implement CSRF protection
   - Add rate limiting on login

4. **Password Policy**
   - Minimum 8 characters
   - Require complexity (numbers, symbols)
   - Password expiry policy

5. **Account Security**
   - Lock accounts after failed attempts
   - Email verification for new users
   - Two-factor authentication (future)

## Future Enhancements

- [ ] Password reset via email
- [ ] Remember me checkbox
- [ ] Two-factor authentication
- [ ] OAuth integration (Google, Microsoft)
- [ ] Account lockout after failed attempts
- [ ] Audit log for user actions
- [ ] User profile management
- [ ] Password change functionality

## Troubleshooting

### Issue: Can't login
**Check:**
1. User exists in tenant database
2. Password hash is valid
3. User is active (`is_active = 1`)
4. Username/password are correct

### Issue: Redirected to login immediately
**Cause:** Session expired or invalid

**Solution:** Clear cookies and login again

### Issue: 500 error on login
**Check:**
1. Master database has sessions table
2. Tenant database has users table
3. Session store is properly initialized

---

**Status**: ✅ Authentication system ready for use!
