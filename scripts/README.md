# End-to-end smoke test

This folder contains scripts you can use against a **live server** to verify
the multi-tenant flow end-to-end:

* Provision two tenants via the admin control-plane API.
* Hit the per-tenant API with different `x-tenant-id` headers.
* Prove that data written into tenant `acme` is NOT visible when the same
  endpoint is called with tenant `globex` (tenant DB isolation).
* Exercise the discipline endpoint that uses `RequestCtx` and confirm the
  actor attribution shows up in the resulting notification.
* Poke the negative paths (unknown tenant → 404, disabled tenant → 403).

## 1. Start the server

From the repo root:

```bash
# Optional: point the server at a scratch data dir so a smoke test doesn't
# pollute your real DBs. Both dirs will be auto-created.
export SYSTEM_DB_URL='sqlite://data/system.db?mode=rwc'
export TENANT_DB_ROOT='data/tenants'
export BIND='127.0.0.1:3000'
export RUST_LOG='school_erp=info,tower_http=info'

cargo run --release
```

You should see `listening on 127.0.0.1:3000` in the terminal.

Tenant DB files will appear at `data/tenants/<tenant_id>.db`.

## 2. Run the smoke test

```bash
BASE_URL=http://127.0.0.1:3000 bash scripts/e2e_smoke.sh
```

Works on Linux, macOS, WSL, or Git-Bash on Windows. The script prints each
step, the request it sends, and the response. A clean run ends with
`ALL CHECKS PASSED ✔`.

Optional overrides:

```bash
BASE_URL=http://192.168.1.100:3000 \
ACME=schoolA GLOBEX=schoolB \
bash scripts/e2e_smoke.sh
```

If you have `jq` on `PATH`, the script uses it to pretty-print JSON;
otherwise it passes responses through unchanged.

## 3. What the script exercises

| Step | Call | Purpose |
|-----:|------|---------|
| 1 | `GET  /api/live` and `GET /api/ready` | Server is up & healthy |
| 2 | `POST /api/admin/tenants` (`acme`) | Provisioning: creates `data/tenants/acme.db` and runs migrations against it |
| 3 | `POST /api/admin/tenants` (`globex`) | Second tenant → separate DB file |
| 4 | `GET  /api/admin/tenants` | Both tenants listed in the system DB |
| 5 | `POST /api/tenant/people/staff` with `x-tenant-id: acme` | Writes into `acme.db` only |
| 6 | `POST /api/tenant/people/staff` with `x-tenant-id: globex` | Writes into `globex.db` only |
| 7 | `GET  /api/tenant/people/staff` per tenant | Each tenant only sees *their* staff — this is the isolation check |
| 8 | `POST /api/tenant/people/students/admit` (acme) | Admits a student in `acme.db` |
| 9 | `POST /api/tenant/discipline/` (acme, with `x-user-id: 42`) | Exercises the new `RequestCtx` plumbing; the notification title should mention `"reported by user #42"` |
| 10 | Same call with `x-tenant-id: does-not-exist` | Expected **404 Not Found** — unknown tenant |
| 11 | `POST /api/admin/tenants/globex/disable` then a globex call | Expected **403 Forbidden** — disabled tenant |
| 12 | `POST /api/admin/tenants/globex/enable` | Re-enable so subsequent runs work |

## 4. Verifying the tenant DBs directly (optional)

If you have `sqlite3` installed:

```bash
sqlite3 data/tenants/acme.db   "SELECT count(*) FROM staff;"
sqlite3 data/tenants/globex.db "SELECT count(*) FROM staff;"
sqlite3 data/system.db         "SELECT tenant_id, status FROM tenant;"
```

You should see the counts diverge — proof that the two tenants are backed
by separate SQLite files.

## 5. Cleanup

Because everything is in flat SQLite files under `data/`, the fastest reset is:

```bash
# Stop the server first (Ctrl+C), then:
rm -rf data/
```

Next `cargo run` will recreate `data/system.db` and an empty
`data/tenants/` folder.
