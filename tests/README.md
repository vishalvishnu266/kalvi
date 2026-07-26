# Integration tests

Out-of-process tests: each one issues real HTTP requests to a
locally-running `school_erp` server. There is intentionally no
in-process axum harness — the goal is to exercise the same binary
and routing that production uses.

## Layout

One test binary per HTTP surface, so failures are easy to localise
(`cargo test --test <name>` runs just that surface):

| File           | Surface                                | Notes                                   |
| -------------- | -------------------------------------- | --------------------------------------- |
| `health.rs`    | `/api/health`, `/api/live`, `/api/ready` | No tenant, no auth.                    |
| `admin.rs`     | `/admin/api/tenants*`                    | Control-plane CRUD (create / list / disable / soft-delete). |
| `auth.rs`      | `/api/{tenant}/auth/*`, `/web/{tenant}/login` | Register, login, whoami, bad-password rejection. |
| `demo.rs`      | `/api/{tenant}/demo/*`, `/web/{tenant}/`  | Reference module — ping, echo, list, dashboard. |
| `common/mod.rs`| shared helpers                            | Fixture, cookie-jar client, assertions. |

## Fixture + automatic teardown

Tests that need a live tenant use `common::Fixture`:

```rust
let mut fx = Fixture::new("demo");   // random tenant + admin user
fx.login();                          // captures session cookie in jar
let r = fx.client.get(fx.api_url("/demo/ping")).send()?;
// ... assertions ...
// Fixture is dropped at end of scope — soft-deletes the tenant
// via DELETE /admin/api/tenants/{id}, even if the test panicked.
```

The `Drop` impl calls `soft_delete_tenant` on a **fresh** client, so
a poisoned cookie jar can't stop cleanup. Failures during teardown
are logged (`eprintln!`) but never mask the original test failure.

## Running

Terminal 1 — start the server once:

```bash
bash scripts/dev_reset.sh          # wipes data/, builds, boots on :3000
```

Terminal 2 — run all suites:

```bash
cargo test
```

Or one surface at a time:

```bash
cargo test --test health
cargo test --test admin
cargo test --test auth
cargo test --test demo
```

Point at a different instance:

```bash
BASE_URL=http://127.0.0.1:4000 cargo test
```

Tests print `SKIP:` and pass without assertions when the server is
unreachable, so `cargo test` in a fresh checkout won't red-flag
someone who just wants to compile-check.

## Adding a new surface

When you build a new business module (say, `attendance`):

1. Copy `tests/demo.rs` to `tests/attendance.rs`.
2. Replace `demo` paths with `attendance` paths.
3. Add module-specific tests. Use `Fixture` for the tenant + auth
   plumbing so cleanup happens for free.

If your module needs a *different* seeded role/permission, either:
* extend `migrations/20260720120002_seed_rbac_stub.sql` (fine for a
  dev-only skeleton), or
* have the test create the role/permission via a new admin endpoint
  once you build one.
