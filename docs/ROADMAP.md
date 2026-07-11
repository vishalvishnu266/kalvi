# Roadmap

Prioritised list of what to build next.

## Phase 1 — Student management

- `GET /t/{slug}/students` — list, search, filter
- `GET/POST /t/{slug}/students/new` — add student
- `GET /t/{slug}/students/{id}` — student profile
- `GET/POST /t/{slug}/students/{id}/edit` — edit
- `POST /t/{slug}/students/{id}/delete` — delete

Migrations: extend `db/migrations/tenant/20260711000003_create_students.sql` or add a new migration with additional columns as fields are needed.

## Phase 2 — User management (admin only)

- `GET /t/{slug}/admin/users`
- `GET/POST /t/{slug}/admin/users/new`
- `GET/POST /t/{slug}/admin/users/{id}/edit`
- Deactivate / reactivate

## Phase 3 — Session admin

- `GET /t/{slug}/admin/sessions` — list active sessions
- Force-logout a session
- Periodic background cleanup of expired sessions

## Phase 4 — Dashboard enhancements

- Stat cards (student counts, active sessions, etc.)
- Recent activity

## Phase 5 — Attendance

- New tenant migration for `attendance`
- Mark attendance, view report

## Cross-cutting

- Rate-limit login attempts
- CSRF tokens for form POSTs
- Structured logging (`tracing`)
- Automated tests
- Dockerfile
