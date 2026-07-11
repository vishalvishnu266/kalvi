# School ERP — Server

Rust workspace containing the multi-tenant school ERP server.

## Run

```bash
cargo run
```

Server listens on `http://localhost:3000`.

## Workspace

| Crate     | Purpose                                                              |
|-----------|----------------------------------------------------------------------|
| `shared`  | `TenantDatabaseManager`, `AppState`, `TenantContext`, tenant middleware, migration runners. |
| `auth`    | Login/logout/dashboard, session store (in tenant DB), `RequireAuth` extractor, admin seeding. |
| `tenant`  | Home page + tenant onboarding.                                       |
| `student` | Placeholder for future student features.                             |
| `server`  | `main.rs` — wires everything, applies master migrations at startup.  |

## Migrations

All migrations live in [`migrations/`](migrations):

- `migrations/master/` — applied to `master.db` on startup.
- `migrations/tenant/` — applied to every new `tenant_{slug}.db` the first time it is opened.

Both directories are embedded at compile time using `sqlx::migrate!` and applied via a `Migrator`. sqlx tracks applied versions in the `_sqlx_migrations` table inside each DB, so the calls are safe/idempotent.

To add a new tenant migration, drop a file named `NNNNNN_description.sql` into `migrations/tenant/`. It will be applied automatically to all existing (on next open) and future tenant databases.

## Templates

Askama templates live in [`templates/`](templates) and are compiled into the binary. See `askama.toml`.

- `base.html` includes Bootstrap 5 and Hotwire Turbo 8 (via CDN ES module).
- Every page template extends `base.html`.
- All form POSTs use standard `303 See Other` redirects, which Turbo Drive follows transparently.

## Routes

| Method | Path                          | Description                          |
|--------|-------------------------------|--------------------------------------|
| GET    | `/`                           | Landing page                         |
| GET    | `/onboard`                    | Onboarding form                      |
| POST   | `/onboard`                    | Create tenant + seed admin user      |
| GET    | `/t/{slug}/login`             | Login form                           |
| POST   | `/t/{slug}/login`             | Authenticate + set session cookie    |
| POST   | `/t/{slug}/logout`            | Destroy session + clear cookie       |
| GET    | `/t/{slug}/dashboard`         | Auth-required dashboard              |

## Databases

At runtime the workspace root will contain:

```
master.db                # tenant registry
tenant_<slug>.db         # per-tenant users, sessions, students
```

All `.db*` files are gitignored.
