# School ERP System

Multi-tenant education management platform built with **Rust + Axum + Askama + Hotwire Turbo + SQLite**.

## 🚀 Quick Start

```bash
cd server
cargo run
```

Then open http://localhost:3000/.

## ✨ End-to-end flow

1. **Home** — `/` shows a landing page with an "Onboard" button.
2. **Onboard** — `/onboard` accepts institution details **plus** an initial admin username & password.
3. Server creates a row in `master.db`, provisions `tenant_{slug}.db` (runs tenant migrations automatically), and inserts the seeded admin user.
4. **Login** — `/t/{slug}/login` authenticates against the tenant's `users` table and sets a session cookie backed by the tenant's `sessions` table.
5. **Dashboard** — `/t/{slug}/dashboard` requires a valid session for that tenant.
6. **Logout** — `POST /t/{slug}/logout` deletes the session and clears the cookie.

## 🧱 Architecture

- **Master DB** (`master.db`) — tenant registry only.
- **Tenant DB** (`tenant_{slug}.db`) — everything else for that tenant: `users`, `sessions`, `students`.
- **Tenant middleware** — resolves `/t/{slug}/...` paths, injects the correct SQLite pool + `TenantContext` into request extensions.
- **Session middleware** — reads the session cookie and injects a `Session` into extensions (if valid).
- **`RequireAuth` extractor** — pulls `Session` + tenant pool from extensions and loads the `User` from the tenant DB.

## 📁 Layout

```
server/
├── Cargo.toml                     # workspace
├── askama.toml                    # Askama template root
├── migrations/
│   ├── master/                    # tenants table
│   └── tenant/                    # users, sessions, students tables
├── templates/                     # Askama HTML templates
│   ├── base.html                  # Bootstrap 5 + Hotwire Turbo shell
│   ├── home.html
│   ├── onboarding/
│   └── auth/
└── crates/
    ├── shared/                    # DB manager, tenant middleware, AppState
    ├── auth/                      # login, logout, dashboard, session
    ├── tenant/                    # home, onboarding
    ├── student/                   # placeholder for future features
    └── server/                    # main.rs
```

## 🛠️ Tech Stack

- **Axum 0.8** — async web framework
- **SQLite via sqlx 0.8** — one DB per tenant, `sqlx::migrate!` for schema
- **Askama 0.13** — compile-time HTML templates
- **Hotwire Turbo 8** — progressive enhancement via CDN import in `base.html`
- **Bootstrap 5** — UI
- **bcrypt** — password hashing
- **cookie** — session cookie management

## 📚 More docs

See [`docs/`](docs/) for architecture and guides.
