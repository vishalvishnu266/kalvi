-- ============================================================
-- SchoolDesk ERP - Master DB schema
-- One master.db for the whole platform. Tracks tenants.
-- ============================================================

CREATE TABLE IF NOT EXISTS tenants (
    id                TEXT PRIMARY KEY,            -- URL slug, e.g. "demo"
    name              TEXT NOT NULL,               -- Display name, e.g. "Demo Public School"
    institution_type  TEXT NOT NULL DEFAULT 'school'
                          CHECK (institution_type IN ('school','university')),
    status            TEXT NOT NULL DEFAULT 'active'
                          CHECK (status IN ('active','suspended','archived')),
    created_at        DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at        DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_tenants_status ON tenants(status);

-- Seed the demo tenant so /web/demo/... continues to work.
INSERT OR IGNORE INTO tenants (id, name, institution_type, status)
VALUES ('demo', 'Demo Public School', 'school', 'active');
