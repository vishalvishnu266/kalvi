# Session System

## Storage

Sessions live in the **tenant DB** in the `sessions` table:

```sql
CREATE TABLE sessions (
    id TEXT PRIMARY KEY,              -- UUID v4
    user_id INTEGER NOT NULL,         -- FK -> users.id
    ip_address TEXT,
    user_agent TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    expires_at TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
```

## Cookie

- Name: `session_id`
- HttpOnly, `SameSite=Lax`, path `/`, max-age 7 days.

## Flow

1. **Login** — after password verification, `session::create_session` inserts a row and returns the `Session`. The handler sets the cookie via `session::set_session_cookie` and issues a `303` redirect to the dashboard.
2. **Any subsequent request** — `session_middleware` reads the cookie, calls `session::get_valid_session` (which enforces `expires_at > now`), and injects the `Session` into request extensions.
3. **RequireAuth** — the extractor pulls the `Session` and tenant pool, then loads the `User`. On any failure it returns `Redirect::to("/t/{slug}/login")`.
4. **Logout** — deletes the row and clears the cookie.

## Cleanup

Expired sessions are ignored by `get_valid_session` (SQL filters `expires_at > now`). A background task to actually delete expired rows is planned (see [ROADMAP.md](../ROADMAP.md), Phase 3).
