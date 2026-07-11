# School ERP System

Multi-tenant education management platform built with **Rust + Axum + Hotwire Turbo + Tailwind CSS + SQLite**.

## 🚀 Quick Start

```bash
cargo run
```

Then open http://localhost:3000/.

## ✨ End-to-end flow

1. **Home** — `/` shows a landing page with an "Onboard" button.
2. **Onboard** — `/onboard` accepts institution details **plus** an initial admin username & password.
3. Server creates a row in `db/master.db`, provisions `db/{slug}.db` (runs tenant migrations automatically), and inserts the seeded admin user.
4. **Login** — `/t/{slug}/login` authenticates against the tenant's `users` table and sets a session cookie backed by the tenant's `sessions` table.
5. **Dashboard** — `/t/{slug}/dashboard` requires a valid session for that tenant.
6. **Logout** — `POST /t/{slug}/logout` deletes the session and clears the cookie.

## 🧱 Architecture

- **Master DB** (`db/master.db`) — tenant registry only.
- **Tenant DB** (`db/{slug}.db`) — everything else for that tenant: `users`, `sessions`, `students`.
- **Tenant middleware** — resolves `/t/{slug}/...` paths, injects the correct SQLite pool + `TenantContext` into request extensions.
- **Session middleware** — reads the session cookie and injects a `Session` into extensions (if valid).
- **`RequireAuth` extractor** — pulls `Session` + tenant pool from extensions and loads the `User` from the tenant DB.

## 📁 Layout

```
.
├── Cargo.toml                     # Project manifest
├── db/                            # Database storage and migrations
│   ├── migrations/
│   │   ├── master/                # tenants table
│   │   └── tenant/                # users, sessions, students tables
│   └── *.db                       # SQLite database files
├── src/
│   ├── config/                    # AppState, DatabaseManager
│   ├── controllers/               # Route handlers
│   ├── middleware/                # Tenant, Session, Auth middleware
│   ├── models/                    # Data models
│   ├── repositories/              # Database access logic
│   ├── services/                  # Business logic
│   ├── views/                     # HTML rendering (raw strings/Tailwind)
│   ├── web_utils/                 # Helpers, Constants
│   ├── main.rs                    # Entry point
│   └── Routes.rs                  # Router definition
└── docs/                          # Documentation
```

## 🛠️ Tech Stack

- **Axum 0.8** — async web framework
- **SQLite via sqlx 0.8** — one DB per tenant, `sqlx::migrate!` for schema
- **Hotwire Turbo 8** — progressive enhancement via CDN import
- **Tailwind CSS** — utility-first CSS for styling
- **bcrypt** — password hashing
- **cookie** — session cookie management

## 📚 More docs

See [`docs/`](docs/) for architecture and guides.
