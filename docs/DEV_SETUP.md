# Dev setup

> Blow away everything, rebuild, and end up with a live server that
> has no tenants and no business modules — the framework skeleton on
> which the school ERP will be built.

```bash
bash scripts/dev_reset.sh
```

Windows:
```powershell
pwsh scripts/dev_reset.ps1
```

When it prints `Done`, open <http://127.0.0.1:3000/>. See
[`../scripts/README.md`](../scripts/README.md) for the exact `curl`
commands to create a tenant + admin user + hit the demo endpoints.

---

## What `dev_reset.sh` does

1. **Stops any running server** — by PID file (`.dev_server.pid`) if
   present, otherwise by anything listening on the target port.
2. **Wipes `data/`** — deletes `system.db`, `sessions.db`,
   `tenants/*.db`, and their WAL / SHM sidecars.
3. **Builds the server** (unless `SKIP_BUILD=1`).
4. **Starts it in the background** — logs to `.dev_server.log`, PID
   written to `.dev_server.pid`.
5. **Polls `GET /api/live`** until healthy (60s timeout).

No seed step: this codebase is intentionally free of business rows
right now.

---

## Env-var knobs

| Var          | Default                     | Effect                                           |
| ------------ | --------------------------- | ------------------------------------------------ |
| `PROFILE`    | `release`                   | Use `debug` for faster incremental rebuilds.     |
| `PORT`       | `3000`                      | Change the server bind port.                     |
| `BASE_URL`   | `http://127.0.0.1:$PORT`    | Override the URL used in log output.             |
| `DATA_DIR`   | `data`                      | Directory to wipe. Change with care.             |
| `SKIP_BUILD` | `0`                         | `1` = don't run `cargo build`.                   |
| `FOREGROUND` | `0`                         | `1` = keep the server attached to this terminal. |
| `LOG_FILE`   | `.dev_server.log`           | Background server's stdout+stderr.               |
| `PID_FILE`   | `.dev_server.pid`           | Background server's PID.                         |

---

## Framework layout at a glance

```
src/
  main.rs         bootstrap: config → system DB → session store → axum
  lib.rs          module map + public re-exports
  config.rs       env-driven Config
  db.rs           tenant SqlitePool factory + migrations
  system.rs       control-plane pool + shared row types
  session/        session store (dedicated sessions.db)
  tenancy.rs      TenantId validation
  error.rs        RepoError + RepoResult
  health_probes.rs, shutdown.rs
  middleware/     tenant + auth + tracing
  http/           router assembly (admin, tenant api, web, portal)
  api/            JSON handlers (admin, auth, demo)
  services/       free-fn service layer (auth, system, demo)
  models/         row / DTO types (auth, demo)
  web/            server-rendered pages (landing, login, dashboard, portal, demo, ...)

migrations/         tenant DB (auth base + demo)
migrations_system/  control-plane DB (tenant + portal user)
migrations_session/ session store DB
templates/          askama HTML templates
```

To add a real module (e.g. `attendance`), create:

* `migrations/NNNN_attendance.sql`
* `src/models/attendance.rs`
* `src/services/attendance.rs` (free fns + `perm::ATTENDANCE_*` codes)
* `src/api/attendance.rs` and/or `src/web/attendance.rs`
* Wire routes in `src/http/api_routes/mod.rs` / `src/http/web/mod.rs`
* Add nav item (`src/web/layout.rs`) and dashboard tile (`src/web/dashboard.rs`)

The `demo` module is the reference implementation for that pattern.
