# Multi-Tenant Design

## Strategy: database per tenant

Each tenant gets its own SQLite database file named `db/{slug}.db`. This gives us:

- **Hard isolation** — no cross-tenant SQL is possible; there is no `tenant_id` column to accidentally forget in a WHERE clause.
- **Per-tenant backup / restore / delete** — copy or drop a single file.
- **Simple scaling** for small/medium tenants — SQLite handles it well.

The tradeoff is more open file handles and no cross-tenant reporting from a single query.

## Tenant registry

`master.db` contains a `tenants` table with slug, name, contact info, and `database_name`. This is the only place slugs are resolved.

## Routing

Path convention: `/t/{slug}/...`. `TenantMiddleware` extracts the slug, looks up the tenant, and injects the tenant `SqlitePool` into request extensions.

Handlers pull the pool with `Extension<SqlitePool>` — they don't need to know which tenant they're operating on.

## Pool caching

`TenantDatabaseManager` holds `Arc<RwLock<HashMap<String, SqlitePool>>>` so opening the same tenant twice reuses the pool.

## Provisioning a new tenant

The `POST /onboard` handler:

1. Validates that the slug is non-empty and unique.
2. Inserts a row into `master.db#tenants`.
3. Calls `db_manager.tenant_pool(&database_name)` which:
   - Creates `db/{slug}.db` on disk (`.create_if_missing(true)`).
   - Runs `TENANT_MIGRATOR` against it (idempotent via `_sqlx_migrations`).
4. Calls `AuthService::create_admin_user(&pool, username, password)` with the credentials from the form to seed the initial admin.
5. Renders a success page linking to `/t/{slug}/login`.
