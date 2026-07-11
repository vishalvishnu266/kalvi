# Current Status

_Last updated: 2026-07-11 (post-rewrite)_

## ✅ Complete

- Multi-tenant infrastructure (path-based `/t/{slug}/...` routing)
- Central migrations for master DB and per-tenant DB via `sqlx::migrate!`
- Tenant onboarding form that provisions a new tenant DB and seeds an admin user (username & password captured from the form)
- Login / logout / dashboard flow
- Cookie-based sessions stored in the tenant DB
- `RequireAuth` extractor (native axum 0.8 `FromRequestParts`)
- Askama templates with a shared `base.html` and Hotwire Turbo loaded via CDN

## 🚧 In progress / open

- Student CRUD (only the schema exists)
- User management UI (create/list users beyond the initial admin)
- Session admin (list active sessions, force logout)
- Background job for expired-session cleanup (schema has `expires_at`; queries already exclude expired rows, but a periodic delete would be nice)
- Automated tests
