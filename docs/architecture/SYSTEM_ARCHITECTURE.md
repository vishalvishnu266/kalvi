# System Architecture

## Overview

```
┌──────────────────────────────────────────────────────────────────┐
│                          Axum Router                             │
│                                                                  │
│  tenant::routes()       auth::routes()                           │
│    GET  /                 GET  /t/{slug}/login                   │
│    GET  /onboard          POST /t/{slug}/login                   │
│    POST /onboard          POST /t/{slug}/logout                  │
│                           GET  /t/{slug}/dashboard               │
│                                                                  │
│  layer: session_middleware  (reads cookie -> Session)            │
│  layer: tenant_middleware   (resolves /t/{slug} -> tenant pool)  │
└──────────────────────────────────────────────────────────────────┘
                              │
                              ▼
                  ┌────────────────────┐
                  │  AppState          │
                  │  db_manager: Arc<T>│
                  └────────────────────┘
                              │
             ┌────────────────┴────────────────┐
             ▼                                 ▼
     ┌────────────────┐               ┌────────────────┐
     │  master.db     │               │ tenant_<slug>  │
     │  tenants table │               │  users         │
     └────────────────┘               │  sessions      │
                                      │  students      │
                                      └────────────────┘
```

## Request lifecycle

1. Request enters the Axum router.
2. `tenant_middleware` inspects the path:
   - If it starts with `/t/{slug}/`, look up the tenant in `master.db`, open (or reuse) the tenant pool via `TenantDatabaseManager::tenant_pool`, inject the tenant `SqlitePool`, master `SqlitePool`, and `TenantContext` into request extensions.
   - Otherwise inject only the master pool.
3. `session_middleware` extracts the `session_id` cookie. If a pool is present in extensions, it loads the row from `sessions` (checking `expires_at > now`) and injects a `Session`.
4. Route handler runs. If it uses `RequireAuth`, the extractor pulls `Session` + tenant pool and loads the `User`.

## Templates

Askama templates in `server/templates/` are compiled into the binary. `base.html` provides the Bootstrap + Hotwire Turbo shell. All feature templates extend it.

## Migrations

`sqlx::migrate!("../../migrations/master")` and `sqlx::migrate!("../../migrations/tenant")` (paths relative to the `shared` crate) embed the SQL files at compile time. `MASTER_MIGRATOR` runs once during `TenantDatabaseManager::new()`. `TENANT_MIGRATOR` runs the first time each tenant pool is opened.
