# Documentation

Reference material for the School ERP codebase after the 2026-07-11 rewrite.

## Index

- [System Architecture](architecture/SYSTEM_ARCHITECTURE.md)
- [Multi-Tenant Design](architecture/MULTI_TENANT_DESIGN.md)
- [Session System](architecture/SESSION_SYSTEM.md)
- [Getting Started](guides/GETTING_STARTED.md)
- [Authentication](guides/AUTHENTICATION.md)
- [Hotwire Guide](guides/HOTWIRE_GUIDE.md)
- [Roadmap](ROADMAP.md)
- [Current Status](STATUS.md)

## Snapshot

- **Web framework:** Axum 0.8
- **Templates:** HTML rendering via raw strings in `src/views/`
- **Progressive enhancement:** Hotwire Turbo 8 (CDN import)
- **Database:** SQLite via sqlx 0.8 — one DB per tenant, migrations under `db/migrations/`
- **Auth:** Custom sessions, bcrypt password hashing, cookie-based
- **Multi-tenancy:** Path routing at `/t/{slug}/...`, tenants registered in `db/master.db`

## Notable directories

```
.
├── db/               # Database storage and migrations
│   └── migrations/   # master + tenant migrations
├── src/              # Source code (controllers, views, models, etc.)
└── docs/             # Documentation
```
