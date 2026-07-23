# Scripts

Helper scripts for spinning up and populating a **dev** or **demo** instance.

Nothing in `src/` depends on any of these — the application itself ships with
**zero built-in mock data**. These scripts exist purely so you can populate a
fresh instance without hand-typing rows.

| Script              | What it does                                                                                 |
| ------------------- | -------------------------------------------------------------------------------------------- |
| `dev_reset.sh`      | **One command**: stop server → wipe `data/` → rebuild → start → seed. See `docs/DEV_SETUP.md`. |
| `dev_reset.ps1`     | Windows / PowerShell equivalent of `dev_reset.sh`.                                            |
| `seed_demo.sh`      | API-based seeder used by `dev_reset.sh`. Also runnable on its own against any live server.   |
| `seed_demo.sql`     | SQL-based alternative — pipes rows straight into a tenant DB (`sqlite3 data/tenants/demo.db < …`). |

---

## Fastest path: `dev_reset.sh`

```bash
bash scripts/dev_reset.sh
```
or on Windows:
```powershell
pwsh scripts/dev_reset.ps1
```

After it prints `Done`, open <http://127.0.0.1:3000/web/login> and sign in as
any of the seeded personas (see [`docs/DEV_SETUP.md`](../docs/DEV_SETUP.md)).

---

## Running `seed_demo.sh` on its own

```bash
# start the server first
cargo run --release

# then, in another shell:
BASE_URL=http://127.0.0.1:3000 bash scripts/seed_demo.sh
```

Overridable env vars: `BASE_URL`, `TENANT`, `TENANT_NAME`, `ADMIN_USER`, `ADMIN_PASS`.

The script is idempotent — repeated runs are safe.

---

## `seed_demo.sql` — offline / bulk-load option

```bash
sqlite3 data/tenants/demo.db < scripts/seed_demo.sql
```

Requires the tenant DB to already exist (create the tenant once via
`POST /admin/api/tenants` or run any request that resolves the tenant, which
will run the migrations for you).

Uses `INSERT OR IGNORE` throughout — safe to re-apply.
