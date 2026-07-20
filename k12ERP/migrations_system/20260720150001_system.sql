-- =====================================================================
-- SYSTEM (control-plane) DATABASE
-- One row per tenant with lifecycle metadata.
-- =====================================================================
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS tenant (
    id          INTEGER PRIMARY KEY,
    tenant_id   TEXT NOT NULL UNIQUE,        -- external short id (subdomain-safe)
    name        TEXT NOT NULL,
    status      TEXT NOT NULL DEFAULT 'active'
                CHECK (status IN ('active','disabled','deleted')),
    plan        TEXT,
    notes       TEXT,
    created_at  TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at  TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX IF NOT EXISTS ix_tenant_status ON tenant(status);

-- Admin users for the control-plane (separate from per-tenant users).
CREATE TABLE IF NOT EXISTS admin_user (
    id             INTEGER PRIMARY KEY,
    username       TEXT NOT NULL UNIQUE,
    email          TEXT UNIQUE,
    password_hash  TEXT NOT NULL,
    is_active      INTEGER NOT NULL DEFAULT 1 CHECK (is_active IN (0,1)),
    created_at     TEXT NOT NULL DEFAULT (datetime('now'))
);
