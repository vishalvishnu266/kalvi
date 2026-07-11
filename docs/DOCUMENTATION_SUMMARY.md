# Documentation Summary

All docs were rewritten on 2026-07-11 alongside a full server rewrite. The high-level design (multi-tenant, path-based routing, per-tenant SQLite) is unchanged. What changed:

- **Templates:** Askama replaces Maud.
- **Migrations:** centralised under `server/migrations/`, applied with `sqlx::migrate!`.
- **Validation:** removed the `validator` crate; handlers trim/verify inputs directly.
- **Onboarding:** captures the initial admin username & password (no more `SEED_ADMIN.sql`).
- **Session store:** stays in the tenant DB.

If you find a doc that still references Maud, `SEED_ADMIN.sql`, or the old async-trait extractors, it is out of date — open an issue or update it.
