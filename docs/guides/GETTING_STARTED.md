# Getting Started - School ERP

> **Quick start guide for AI agents and developers**

## 🚀 First Time Setup

### Prerequisites
- Rust toolchain installed
- SQLite3 (optional, for inspecting databases)

### Start the Server
```bash
cd server
cargo run
```

You should see:
```
🚀 School ERP Server listening on http://localhost:3000

📋 Onboard a tenant: http://localhost:3000/onboard
🔐 Login (tenant-scoped): http://localhost:3000/t/{tenant-slug}/login
📊 Dashboard: http://localhost:3000/t/{tenant-slug}/dashboard

ℹ️  Custom session system active!
💡 Each tenant has isolated users, sessions, and data
```

### Create Your First Tenant

1. **Visit onboarding page:**
   ```
   http://localhost:3000/onboard
   ```

2. **Fill the form:**
   - Slug: `demo-school` (lowercase, hyphens only)
   - Name: `Demo School`
   - Email: `admin@demo.school`
   - Phone: `1234567890`
   - Address: `123 Main St`

3. **Submit and note the success message**

### Seed Admin User

```bash
sqlite3 tenant_demo-school.db < server/SEED_ADMIN.sql
```

This creates:
- **Admin:** username `admin`, password `admin123`
- **Teacher:** username `teacher1`, password `admin123`
- **Staff:** username `staff1`, password `admin123`

⚠️ **Change these passwords in production!**

### Login

```
http://localhost:3000/t/demo-school/login
Username: admin
Password: admin123
```

You'll be redirected to the dashboard!

---

## 📁 Project Structure

```
school-erp/
├── docs/                    # All documentation (you are here)
│   ├── README.md           # Documentation index
│   ├── ROADMAP.md          # Future plans
│   ├── STATUS.md           # Current state
│   ├── architecture/       # System design
│   ├── guides/            # How-to guides
│   ├── reference/         # Technical reference
│   └── implementation/    # Implementation notes
│
├── server/                # Rust backend
│   ├── Cargo.toml         # Workspace definition
│   ├── SEED_ADMIN.sql     # Create test users
│   │
│   └── crates/            # Rust crates
│       ├── auth/          # Authentication & sessions
│       ├── tenant/        # Tenant management
│       ├── student/       # Student features
│       ├── shared/        # Shared utilities
│       └── server/        # Main application
│
└── notes.txt             # Development session notes
```

---

## 🎯 Core Concepts

### Multi-Tenancy
- **One database per tenant** (school/institution)
- Path-based routing: `/t/{tenant-slug}/...`
- Complete data isolation
- Tenants registered in `master.db`
- Tenant data in `tenant_{slug}.db`

### Authentication
- Custom session system (no tower-sessions)
- Sessions stored in tenant database
- Secure HTTP-only cookies
- Role-based access: Admin, Teacher, Staff
- 24-hour session expiry

### URL Structure
```
Control Plane (Master DB):
  /              - Landing page
  /onboard       - Tenant onboarding

Tenant Routes (Tenant DB):
  /t/{slug}/login      - Login
  /t/{slug}/dashboard  - Dashboard
  /t/{slug}/logout     - Logout
  
  Future:
  /t/{slug}/students   - Student list
  /t/{slug}/users      - User management (admin)
  /t/{slug}/admin/*    - Admin features
```

---

## 💻 Development Workflow

### Adding a New Feature

1. **Identify the domain** (student, user, attendance, etc.)

2. **Create feature file:**
   ```bash
   # Example: List students
   touch server/crates/student/src/list_students.rs
   ```

3. **Follow the pattern:**
   ```rust
   // 1. Models
   // 2. Database queries
   // 3. Business logic
   // 4. Templates (Maud)
   // 5. HTTP handlers
   // 6. Routes
   ```

4. **Register in lib.rs:**
   ```rust
   mod list_students;
   
   pub fn routes() -> Router<AppState> {
       Router::new()
           .merge(list_students::routes())
   }
   ```

5. **Test:**
   ```bash
   cargo run
   # Visit your new route
   ```

### Database Changes

For tenant databases:
```sql
-- Make changes directly to tenant DB
sqlite3 tenant_demo-school.db

-- Example: Add column
ALTER TABLE students ADD COLUMN grade_level INTEGER;
```

For production, create migrations in the tenant initialization code.

### Using Hotwire Turbo

See [HOTWIRE_GUIDE.md](HOTWIRE_GUIDE.md) for complete patterns.

**Quick example:**
```rust
// Turbo Frame for inline editing
turbo-frame#student-{id} {
    // Click edit → form appears
    // Submit → content updates (no page reload)
}
```

---

## 🔑 Key Files to Know

### Entry Point
- `server/crates/server/src/main.rs` - Application startup

### Middleware
- `server/crates/shared/src/middleware.rs` - Tenant resolution
- `server/crates/auth/src/session_middleware.rs` - Session loading

### Core Features
- `server/crates/auth/src/` - All authentication code
- `server/crates/tenant/src/` - Tenant onboarding
- `server/crates/student/src/` - Student features (mostly empty, ready to build)

### Utilities
- `server/crates/shared/src/db.rs` - Database manager
- `server/crates/auth/src/session.rs` - Session manager

---

## 🛠️ Common Tasks

### View Database
```bash
# Master database (tenants)
sqlite3 master.db
SELECT * FROM tenants;

# Tenant database (users, sessions, students)
sqlite3 tenant_demo-school.db
SELECT * FROM users;
SELECT * FROM sessions;
SELECT * FROM students;
```

### Clean Start
```bash
# Remove all databases
rm *.db

# Restart server (will recreate master.db)
cargo run

# Onboard again
```

### Check Active Sessions
```bash
sqlite3 tenant_demo-school.db
SELECT id, user_id, last_activity, is_active 
FROM sessions 
WHERE is_active = 1;
```

### Create New User
```bash
sqlite3 tenant_demo-school.db

-- Hash password first (use admin123 hash for testing)
INSERT INTO users (username, email, password_hash, role)
VALUES ('newuser', 'new@school.edu', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5yvU1MQl8xvLu', 'teacher');
```

---

## 📚 Next Steps

### For New Features
1. Read [Feature Development](FEATURE_DEVELOPMENT.md)
2. Check [Code Conventions](../reference/CODE_CONVENTIONS.md)
3. See [ROADMAP.md](../ROADMAP.md) for priorities

### For Understanding the System
1. Read [System Architecture](../architecture/SYSTEM_ARCHITECTURE.md)
2. Check [Multi-Tenant Design](../architecture/MULTI_TENANT_DESIGN.md)
3. See [Database Schema](../architecture/DATABASE_SCHEMA.md)

### For Building UI
1. Read [Hotwire Guide](HOTWIRE_GUIDE.md)
2. Check [UI Components](../reference/UI_COMPONENTS.md)
3. Use Bootstrap 5 + Icons

---

## 🆘 Troubleshooting

### Server won't start
- Check if port 3000 is in use
- Run `cargo clean` then `cargo build`

### Can't login
- Verify user exists in tenant database
- Check password hash is correct
- Ensure user is active (`is_active = 1`)

### Session expires immediately
- Check system time is correct
- Verify session `expires_at` is in future

### Database errors
- Ensure all migrations have run
- Check database file permissions
- Verify SQLite version compatibility

---

## 💡 Tips for AI Agents

1. **Always check current implementation** - Read existing code before adding new features
2. **Follow established patterns** - Look at existing features for examples
3. **Use extractors** - `RequireAuth(user)` for protected routes
4. **Tenant-aware** - All routes should use `/t/{slug}/` prefix
5. **Test manually** - Run the server and test in browser

---

**Ready to build?** See [ROADMAP.md](../ROADMAP.md) for next priorities!
