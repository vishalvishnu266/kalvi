# Dev setup — one command

> Blow away everything, rebuild, and end up with a live server and a fully
> seeded demo tenant. Repeatable and idempotent.

```bash
bash scripts/dev_reset.sh
```

Windows:
```powershell
pwsh scripts/dev_reset.ps1
```

That's it. When it prints `Done`, open <http://127.0.0.1:3000/web/login>.
For parent/student cross-tenant portal auth, open <http://127.0.0.1:3000/portal/login>.

---

## What `dev_reset.sh` actually does

1. **Stops any running server** — by PID file (`.dev_server.pid`) if present,
   otherwise by anything listening on the target port.
2. **Wipes `data/`** — deletes `system.db`, `tenants/*.db`, and their WAL /
   SHM sidecars. The top-level directory is recreated empty.
3. **Builds the server** (unless `SKIP_BUILD=1`).
4. **Starts it in the background** — logs to `.dev_server.log`, PID written
   to `.dev_server.pid`.
5. **Polls `GET /api/live`** until the server is healthy (60s timeout).
6. **Runs `scripts/seed_demo.sh`**, which:
   - Provisions the `demo` tenant via `POST /admin/api/tenants`.
   - Registers 7 role-scoped demo users (admin + principal + teacher +
     accountant + librarian + parent + student1).
   - Hires 8 staff and admits 10 students through the tenant API.

Total time on a warm cargo cache: ~10 seconds. Total time on a cold one:
however long `cargo build --release` takes.

---

## Env-var knobs

| Var          | Default                              | Effect                                                   |
| ------------ | ------------------------------------ | -------------------------------------------------------- |
| `PROFILE`    | `release`                            | Use `debug` for faster incremental rebuilds.             |
| `PORT`       | `3000`                               | Change the server bind port.                             |
| `BASE_URL`   | `http://127.0.0.1:$PORT`             | Override the URL the seeder posts to.                    |
| `DATA_DIR`   | `data`                               | Directory to wipe. Change with care.                     |
| `SKIP_BUILD` | `0`                                  | `1` = don't run `cargo build` (assume binary is current).|
| `FOREGROUND` | `0`                                  | `1` = keep the server attached to the current terminal.  |
| `LOG_FILE`   | `.dev_server.log`                    | Where the background server's stdout+stderr are written. |
| `PID_FILE`   | `.dev_server.pid`                    | Where the background server's PID is written.            |
| `SESSION_BACKEND` | `tenant_db`                      | Session storage backend: `tenant_db` or `memory_sqlite`.|
| `SESSION_SNAPSHOT_ROOT` | `data/sessions`            | Snapshot folder used by `memory_sqlite` backend.         |

---

## Seeded demo logins

All non-admin users share password `demo1234`.

| Login       | Password    | Role       | What they see                                                  |
| ----------- | ----------- | ---------- | -------------------------------------------------------------- |
| `admin`     | `admin123`  | admin      | Everything.                                                    |
| `principal` | `demo1234`  | principal  | Everything except tenant `settings.manage`.                    |
| `teacher`   | `demo1234`  | teacher    | Students, attendance, exams, timetable, communication.         |
| `accountant`| `demo1234`  | accountant | Fees + payroll + read students/staff/audit.                    |
| `librarian` | `demo1234`  | librarian  | Library only + read students.                                  |
| `parent`    | `demo1234`  | guardian   | Their own child's data + `fees.pay`.                           |
| `student1`  | `demo1234`  | student    | Their own attendance / fees / marks / timetable / library read.|

Sidebar entries and dashboard launcher tiles are filtered per role by RBAC
— see [`docs/RBAC.md`](RBAC.md) for how that pipeline works.

---

## Managing the background server

```bash
# tail logs
tail -f .dev_server.log

# stop the server
kill $(cat .dev_server.pid)

# start again without wiping the DB
BIND=0.0.0.0:3000 ./target/release/school_erp &
```

---

## Fully fresh vs top-up seed

- `dev_reset.sh` — full wipe + reseed. Use this when you want a **known
  baseline**, e.g. before a demo or after a schema migration.
- `seed_demo.sh` (standalone) — top-up. Only inserts rows that don't already
  exist. Use this when you want to add the demo dataset to a tenant without
  losing your other work.
- `seed_demo.sql` — same rows, delivered via a direct `sqlite3` pipe. Faster
  for bulk loads. Requires the tenant DB to already exist.
