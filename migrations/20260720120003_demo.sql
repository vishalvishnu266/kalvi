-- =====================================================================
-- 003 DEMO: a single throw-away table used by the two demo endpoints
-- (`src/services/demo.rs`, `src/api/demo.rs`, `src/web/demo.rs`).
-- It exists purely as a live example of the module wiring pattern:
-- migration → model → service → api handler → web handler → template.
-- Delete this file (and the matching Rust modules) as soon as you
-- start building real business modules.
-- =====================================================================
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS demo_message (
    id         INTEGER PRIMARY KEY,
    text       TEXT    NOT NULL,
    created_by INTEGER,
    created_at TEXT    NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS ix_demo_message_created
    ON demo_message(created_at DESC);
