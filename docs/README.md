# School ERP Documentation

> **For AI Agents:** This is the central documentation hub. Start here for any development task.

## 📋 Documentation Structure

### 🏗️ Architecture & Design
High-level system architecture and design decisions.

- **[System Architecture](architecture/SYSTEM_ARCHITECTURE.md)** - Complete system overview
- **[Multi-Tenant Design](architecture/MULTI_TENANT_DESIGN.md)** - Tenant isolation strategy
- **[Session System](architecture/SESSION_SYSTEM.md)** - Custom session implementation
- **[Database Schema](architecture/DATABASE_SCHEMA.md)** - All database tables and relationships

### 📖 Development Guides
Step-by-step guides for building features.

- **[Getting Started](guides/GETTING_STARTED.md)** - Quick start for new AI agents
- **[Feature Development](guides/FEATURE_DEVELOPMENT.md)** - How to add new features
- **[Hotwire Integration](guides/HOTWIRE_GUIDE.md)** - Turbo Frames & Streams patterns
- **[Authentication Flow](guides/AUTHENTICATION.md)** - Login, logout, session management

### 🔧 Implementation Reference
Technical implementation details and code patterns.

- **[Code Conventions](reference/CODE_CONVENTIONS.md)** - Coding standards and patterns
- **[API Endpoints](reference/API_ENDPOINTS.md)** - All available routes
- **[Database Operations](reference/DATABASE_OPS.md)** - Common DB queries
- **[UI Components](reference/UI_COMPONENTS.md)** - Reusable UI patterns

### 🚀 Next Steps & Roadmap
Future development plans and priorities.

- **[Development Roadmap](ROADMAP.md)** - Prioritized feature list
- **[Current Status](STATUS.md)** - What's complete, what's next

---

## 🎯 Quick Navigation by Task

### "I need to add a new feature"
1. Read [Feature Development Guide](guides/FEATURE_DEVELOPMENT.md)
2. Check [Code Conventions](reference/CODE_CONVENTIONS.md)
3. See [Hotwire Guide](guides/HOTWIRE_GUIDE.md) for UI patterns

### "I need to understand the system"
1. Start with [System Architecture](architecture/SYSTEM_ARCHITECTURE.md)
2. Read [Multi-Tenant Design](architecture/MULTI_TENANT_DESIGN.md)
3. Check [Database Schema](architecture/DATABASE_SCHEMA.md)

### "I need to work on authentication"
1. Read [Authentication Flow](guides/AUTHENTICATION.md)
2. Check [Session System](architecture/SESSION_SYSTEM.md)
3. See [API Endpoints](reference/API_ENDPOINTS.md)

### "I'm starting fresh"
1. Read [Getting Started](guides/GETTING_STARTED.md)
2. Check [Current Status](STATUS.md)
3. Follow [Development Roadmap](ROADMAP.md)

---

## 📦 Project Structure

```
school-erp/
├── docs/                      # This directory
│   ├── README.md             # You are here
│   ├── ROADMAP.md            # Future plans
│   ├── STATUS.md             # Current state
│   ├── architecture/         # System design docs
│   ├── guides/              # How-to guides
│   ├── reference/           # Technical reference
│   └── implementation/      # Implementation notes
│
├── server/                   # Rust backend
│   ├── crates/
│   │   ├── auth/            # Authentication
│   │   ├── tenant/          # Tenant management
│   │   ├── student/         # Student features
│   │   └── shared/          # Shared utilities
│   └── SEED_ADMIN.sql       # Create test users
│
└── notes.txt                # Session summaries
```

---

## 🔑 Key Concepts

### Multi-Tenancy
- One database per tenant (school/institution)
- Path-based routing: `/t/{tenant-slug}/...`
- Complete data isolation

### Authentication
- Custom session system (no external dependencies)
- Sessions stored in tenant database
- Role-based access: Admin, Teacher, Staff

### Technology Stack
- **Backend**: Rust + Axum
- **Frontend**: Hotwire Turbo + Bootstrap 5
- **Database**: SQLite (per tenant)
- **Templates**: Maud (Rust HTML)

---

## 📝 Maintenance

### Adding New Documentation
1. Create file in appropriate directory
2. Update this README.md with link
3. Follow markdown conventions

### Updating Existing Docs
1. Check if information is still accurate
2. Update relevant sections
3. Update "Last Updated" date

### Removing Outdated Docs
1. Move to `docs/archive/` if historical value
2. Update references in other docs
3. Remove from this index

---

## 🎓 Learning Resources

### External Links
- [Axum Documentation](https://docs.rs/axum)
- [Hotwire Turbo Handbook](https://turbo.hotwired.dev/)
- [Maud Template Guide](https://maud.lambda.xyz/)
- [SQLite Documentation](https://sqlite.org/docs.html)

### Internal Examples
- View Student: `server/crates/student/src/`
- Authentication: `server/crates/auth/src/`
- Tenant Onboarding: `server/crates/tenant/src/`

---

**Last Updated:** 2026-07-11  
**Version:** 2.0 (Reorganized documentation structure)
