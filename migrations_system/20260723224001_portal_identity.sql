PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS portal_user (
    id             INTEGER PRIMARY KEY,
    username       TEXT NOT NULL UNIQUE,
    email          TEXT NOT NULL UNIQUE,
    password_hash  TEXT NOT NULL,
    is_active      INTEGER NOT NULL DEFAULT 1 CHECK (is_active IN (0,1)),
    created_at     TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS portal_membership (
    id               INTEGER PRIMARY KEY,
    portal_user_id   INTEGER NOT NULL REFERENCES portal_user(id) ON DELETE CASCADE,
    tenant_id        TEXT NOT NULL REFERENCES tenant(tenant_id) ON DELETE CASCADE,
    tenant_user_id   INTEGER NOT NULL,
    role             TEXT NOT NULL CHECK (role IN ('guardian','student')),
    created_at       TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE (portal_user_id, tenant_id, tenant_user_id, role)
);

CREATE INDEX IF NOT EXISTS ix_portal_membership_portal_user ON portal_membership(portal_user_id);
CREATE INDEX IF NOT EXISTS ix_portal_membership_tenant ON portal_membership(tenant_id);
