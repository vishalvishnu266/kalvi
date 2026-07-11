# School ERP System

Multi-tenant educational institution management system built with Rust, Axum, Hotwire Turbo, and SQLite.

## 🚀 Quick Start

```bash
cd server
cargo run
```

Visit `http://localhost:3000` to get started!

## 📚 Documentation

**👉 Start here: [docs/README.md](docs/README.md)**

All documentation is organized in the `docs/` directory:

### 📖 Essential Reading
- **[Getting Started](docs/guides/GETTING_STARTED.md)** - Quick start for new developers/AI agents
- **[Current Status](docs/STATUS.md)** - What's implemented and ready
- **[Development Roadmap](docs/ROADMAP.md)** - Prioritized features to build next

### 🏗️ Architecture
- **[System Architecture](docs/architecture/SYSTEM_ARCHITECTURE.md)** - Complete system overview
- **[Multi-Tenant Design](docs/architecture/MULTI_TENANT_DESIGN.md)** - Tenant isolation strategy
- **[Session System](docs/architecture/SESSION_SYSTEM.md)** - Custom session implementation

### 📝 Development Guides
- **[Authentication Guide](docs/guides/AUTHENTICATION.md)** - Auth system and session management
- **[Hotwire Guide](docs/guides/HOTWIRE_GUIDE.md)** - Turbo Frames & Streams patterns

## ✅ What's Complete

- ✅ Multi-tenant system (one DB per school)
- ✅ Tenant onboarding with beautiful UI
- ✅ Custom session system (direct SQLite)
- ✅ User authentication (login/logout)
- ✅ Role-based access (Admin, Teacher, Staff)
- ✅ Modern responsive UI (Bootstrap 5)
- ✅ Secure password hashing (bcrypt)

## 🎯 What's Next

**Priority 1: Student Management**
- List students with search/filter
- Add/edit/delete students
- Student profiles

**Priority 2: User Management**
- Admin panel for creating users
- User activation/deactivation

**Priority 3: Session Admin Panel**
- View active sessions
- Force logout capabilities

See [ROADMAP.md](docs/ROADMAP.md) for complete plan.

## 🛠️ Technology Stack

- **Rust** - Systems programming language
- **Axum** - Modern async web framework
- **SQLite** - Embedded database (one per tenant)
- **Hotwire Turbo** - Progressive enhancement
- **Maud** - Type-safe HTML templating
- **Bootstrap 5** - Responsive UI framework

## 📁 Project Structure

```
school-erp/
├── docs/              # 📚 All documentation
│   ├── README.md      # Documentation index (start here)
│   ├── STATUS.md      # Current implementation status
│   ├── ROADMAP.md     # Development roadmap
│   ├── architecture/  # System design docs
│   ├── guides/       # How-to guides
│   └── implementation/ # Technical notes
│
├── server/           # 🦀 Rust backend
│   ├── crates/
│   │   ├── auth/     # Authentication & sessions
│   │   ├── tenant/   # Tenant management
│   │   ├── student/  # Student features
│   │   ├── shared/   # Shared utilities
│   │   └── server/   # Application entry
│   └── SEED_ADMIN.sql # Test user creation
│
└── notes.txt         # Development session notes
```

## 🎓 For AI Agents

**Starting a new development session?**

1. Read: [docs/README.md](docs/README.md)
2. Check: [docs/STATUS.md](docs/STATUS.md) - What's done
3. Plan: [docs/ROADMAP.md](docs/ROADMAP.md) - What's next
4. Build: Follow patterns in [docs/guides/](docs/guides/)

**All conventions and patterns are documented in `docs/`!**

## 🧪 Testing

### 1. Onboard a Tenant
```
http://localhost:3000/onboard
Slug: demo-school
```

### 2. Seed Admin User
```bash
cd server
sqlite3 tenant_demo-school.db < SEED_ADMIN.sql
```

### 3. Login
```
http://localhost:3000/t/demo-school/login
Username: admin
Password: admin123
```

## 🔐 Security Features

- ✅ bcrypt password hashing (cost 12)
- ✅ HTTP-only secure cookies
- ✅ Session validation on every request
- ✅ Role-based access control
- ✅ Complete tenant data isolation
- ✅ IP address & user agent tracking

## 🌟 Key Features

### Multi-Tenancy
- One database per school/institution
- Path-based routing: `/t/{slug}/...`
- Complete data isolation
- Easy backup and restore per tenant

### Custom Sessions
- Direct SQLite storage (no external deps)
- Sessions stored in tenant database
- 24-hour expiry with auto-cleanup
- Admin can view/revoke sessions

### Modern UI
- Hotwire Turbo for smooth UX
- No full page reloads
- Bootstrap 5 responsive design
- Professional gradient themes

## 📄 License

MIT

---

**Complete documentation:** [docs/README.md](docs/README.md)
