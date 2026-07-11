# School ERP - Server

Multi-tenant educational institution management system built with Rust, Axum, Hotwire Turbo, and SQLite.

## 🚀 Quick Start

```bash
cd server
cargo run
```

Visit: `http://localhost:3000`

## 📚 Documentation

**All documentation is now in the central `docs/` directory.**

👉 **Start here:** [../docs/README.md](../docs/README.md)

### Key Documents
- [Getting Started](../docs/guides/GETTING_STARTED.md) - Quick start guide
- [System Architecture](../docs/architecture/SYSTEM_ARCHITECTURE.md) - How it works
- [Development Roadmap](../docs/ROADMAP.md) - Future plans
- [Current Status](../docs/STATUS.md) - What's complete

## 📁 Project Structure

```
server/
├── crates/
│   ├── auth/        # Authentication & sessions
│   ├── tenant/      # Tenant management
│   ├── student/     # Student features
│   ├── shared/      # Shared utilities
│   └── server/      # Application entry point
│
├── SEED_ADMIN.sql   # Create test users
└── README.md        # This file
```

## 🛠️ Technology Stack

- **Axum** - Async web framework
- **SQLite** - One database per tenant
- **Maud** - Type-safe HTML templates
- **Hotwire Turbo** - Progressive enhancement
- **Custom Sessions** - Direct SQLite storage
- **bcrypt** - Password hashing
- **Bootstrap 5** - Modern UI

## 🎯 Key Features

✅ **Multi-Tenant Architecture**
- One database per school/institution
- Complete data isolation
- Path-based routing: `/t/{slug}/...`

✅ **Custom Session System**
- Direct SQLite storage in tenant DB
- No external dependencies
- IP and user agent tracking
- Admin control ready

✅ **Modern UI**
- Hotwire Turbo for smooth UX
- Bootstrap 5 responsive design
- Bootstrap Icons throughout
- Professional gradients

✅ **Security**
- bcrypt password hashing
- HTTP-only secure cookies
- Role-based access control
- Session validation

## 🚦 Available Routes

### Control Plane
- `GET /` - Landing page
- `GET /onboard` - Tenant onboarding

### Tenant Routes
- `GET /t/{slug}/login` - Login page
- `POST /t/{slug}/login` - Process login
- `GET /t/{slug}/dashboard` - User dashboard
- `GET /t/{slug}/logout` - Logout

## 🗄️ Database Architecture

```
master.db                  # Tenant registry
  └─ tenants table

tenant_{slug}.db          # Per-tenant data
  ├─ users table
  ├─ sessions table
  └─ students table
```

## 🧪 Testing

### 1. Onboard a Tenant
```
http://localhost:3000/onboard
Slug: demo-school
```

### 2. Seed Admin User
```bash
sqlite3 tenant_demo-school.db < SEED_ADMIN.sql
```

### 3. Login
```
http://localhost:3000/t/demo-school/login
Username: admin
Password: admin123
```

## 🆘 Troubleshooting

### Can't start server
```bash
cargo clean && cargo build && cargo run
```

### Can't login
```bash
# Verify user exists
sqlite3 tenant_demo-school.db "SELECT * FROM users;"
```

### Session issues
```bash
# Check active sessions
sqlite3 tenant_demo-school.db "SELECT * FROM sessions WHERE is_active=1;"
```

## 📖 Development

### Adding a New Feature
1. Create file: `crates/{domain}/src/feature_name.rs`
2. Register in `lib.rs`
3. Follow patterns in existing code
4. See [Feature Development Guide](../docs/guides/FEATURE_DEVELOPMENT.md)

### Using Hotwire Turbo
- See [Hotwire Guide](../docs/guides/HOTWIRE_GUIDE.md)
- Use Turbo Frames for inline editing
- Use Turbo Streams for multiple updates

## 🔮 Next Steps

See [Development Roadmap](../docs/ROADMAP.md) for priorities:

**Phase 1: Student Management** (Next)
- List students with search/filter
- Add student form
- View student profile
- Edit student
- Delete student

**Phase 2: User Management**
- User list (admin only)
- Create/edit users
- Activate/deactivate

**Phase 3: Session Management**
- Active sessions dashboard
- Force logout
- Login history

## 📄 License

MIT

---

**For complete documentation, see [../docs/](../docs/)**
