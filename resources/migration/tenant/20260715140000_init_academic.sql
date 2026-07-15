-- ============================================================
-- SchoolDesk ERP - Academic foundations (per tenant)
--   * academic_years : the AY (2025-26 etc.) — most modules reference this
--   * tenant_settings: simple key/value config store (institution_type,
--                      default academic year, etc.)
-- ============================================================

CREATE TABLE IF NOT EXISTS academic_years (
    id            TEXT PRIMARY KEY,               -- UUID
    name          TEXT NOT NULL UNIQUE,           -- e.g. "2025-26"
    start_date    DATE NOT NULL,
    end_date      DATE NOT NULL,
    is_current    INTEGER NOT NULL DEFAULT 0      -- boolean 0/1
                       CHECK (is_current IN (0, 1)),
    status        TEXT NOT NULL DEFAULT 'active'
                       CHECK (status IN ('active', 'archived')),
    created_at    DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at    DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CHECK (start_date < end_date)
);

-- Only ONE academic year can be current at a time.
CREATE UNIQUE INDEX IF NOT EXISTS uq_academic_years_current
    ON academic_years(is_current) WHERE is_current = 1;

CREATE INDEX IF NOT EXISTS idx_academic_years_status ON academic_years(status);

-- Seed the current AY so downstream modules have something to reference.
INSERT OR IGNORE INTO academic_years (id, name, start_date, end_date, is_current, status)
VALUES
    ('ay-2025-26', '2025-26', '2025-06-01', '2026-05-31', 1, 'active'),
    ('ay-2024-25', '2024-25', '2024-06-01', '2025-05-31', 0, 'archived');

-- ------------------------------------------------------------
-- Tenant-level key/value settings.
-- Kept intentionally generic so we don't need a migration per new setting.
-- ------------------------------------------------------------
CREATE TABLE IF NOT EXISTS tenant_settings (
    key         TEXT PRIMARY KEY,
    value       TEXT NOT NULL,
    updated_at  DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT OR IGNORE INTO tenant_settings (key, value) VALUES
    ('institution_type', 'school'),         -- 'school' | 'university'
    ('display_name',     'Demo Institution'),
    ('locale',           'en-IN'),
    ('timezone',         'Asia/Kolkata');
