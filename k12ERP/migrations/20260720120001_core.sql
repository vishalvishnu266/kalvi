-- =====================================================================
-- 001 CORE: school profile, academic year, term
-- =====================================================================
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS school (
    id              INTEGER PRIMARY KEY CHECK (id = 1),
    name            TEXT NOT NULL,
    code            TEXT,
    address         TEXT,
    phone           TEXT,
    email           TEXT,
    logo_path       TEXT,
    currency        TEXT NOT NULL DEFAULT 'USD',
    timezone        TEXT NOT NULL DEFAULT 'UTC',
    created_at      TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at      TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS academic_year (
    id          INTEGER PRIMARY KEY,
    name        TEXT NOT NULL UNIQUE,
    start_date  TEXT NOT NULL,
    end_date    TEXT NOT NULL,
    is_current  INTEGER NOT NULL DEFAULT 0 CHECK (is_current IN (0,1)),
    CHECK (start_date < end_date)
);

CREATE UNIQUE INDEX IF NOT EXISTS ux_academic_year_current
    ON academic_year(is_current) WHERE is_current = 1;

CREATE TABLE IF NOT EXISTS term (
    id                  INTEGER PRIMARY KEY,
    academic_year_id    INTEGER NOT NULL REFERENCES academic_year(id) ON DELETE CASCADE,
    name                TEXT NOT NULL,
    start_date          TEXT NOT NULL,
    end_date            TEXT NOT NULL,
    UNIQUE (academic_year_id, name),
    CHECK (start_date < end_date)
);

CREATE INDEX IF NOT EXISTS ix_term_year ON term(academic_year_id);
