# Scripts

Helper scripts for spinning up a **dev** instance of the framework
skeleton. Business modules and their seed data have been removed — the
app currently ships with only:

* A minimal auth base (`user_account`, `role`, `permission`,
  `user_role`, `role_permission`) plus a seed that grants the `admin`
  role the single `demo.view` permission.
* One reference demo module (`demo_message` table, `GET /api/{tenant}/demo/ping`,
  `POST /api/{tenant}/demo/echo`, `GET /web/{tenant}/demo`) that shows
  the wiring pattern for new domains.

| Script          | What it does                                                                       |
| --------------- | ---------------------------------------------------------------------------------- |
| `dev_reset.sh`  | Stop server → wipe `data/` → rebuild → start → wait for `/api/live`.               |
| `dev_reset.ps1` | Windows / PowerShell equivalent of `dev_reset.sh`.                                 |

---

## Fast path

```bash
bash scripts/dev_reset.sh
```

or on Windows:

```powershell
pwsh scripts/dev_reset.ps1
```

When it prints `Done`, open <http://127.0.0.1:3000/> and provision a
tenant + admin by hand:

```bash
# 1. create a tenant (control-plane API)
curl -sX POST http://127.0.0.1:3000/admin/api/tenants \
     -H 'content-type: application/json' \
     -d '{"tenant_id":"demo","name":"Demo School"}'

# 2. register an admin user inside that tenant
curl -sX POST http://127.0.0.1:3000/api/demo/auth/register \
     -H 'content-type: application/json' \
     -d '{"username":"admin","email":"admin@demo.example","password":"admin123","roles":["admin"]}'

# 3. try the demo endpoints
curl -s http://127.0.0.1:3000/api/demo/demo/ping
```

Sign in at <http://127.0.0.1:3000/web/login> as `admin` / `admin123`
against tenant `demo` and navigate to the Demo tile to see the paired
server-rendered surface.
