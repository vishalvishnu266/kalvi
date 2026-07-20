-- =====================================================================
-- 016 DOCUMENTS + AUDIT LOG
-- Note: `document` uses a polymorphic owner (owner_type, owner_id) that
-- SQLite cannot enforce with a FK. Guard integrity in the Rust layer.
-- =====================================================================
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS document (
    id           INTEGER PRIMARY KEY,
    owner_type   TEXT NOT NULL CHECK (owner_type IN ('student','staff','guardian','invoice','payment','other')),
    owner_id     INTEGER NOT NULL,
    kind         TEXT,
    file_path    TEXT NOT NULL,
    mime_type    TEXT,
    size_bytes   INTEGER CHECK (size_bytes IS NULL OR size_bytes >= 0),
    uploaded_by_user_id INTEGER REFERENCES user_account(id) ON DELETE SET NULL,
    uploaded_at  TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX IF NOT EXISTS ix_document_owner ON document(owner_type, owner_id);

CREATE TABLE IF NOT EXISTS audit_log (
    id           INTEGER PRIMARY KEY,
    user_id      INTEGER REFERENCES user_account(id) ON DELETE SET NULL,
    entity       TEXT NOT NULL,
    entity_id    INTEGER NOT NULL,
    action       TEXT NOT NULL CHECK (action IN ('create','update','delete','login','logout','export')),
    diff_json    TEXT,
    ip_address   TEXT,
    user_agent   TEXT,
    created_at   TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX IF NOT EXISTS ix_audit_entity ON audit_log(entity, entity_id);
CREATE INDEX IF NOT EXISTS ix_audit_user   ON audit_log(user_id, created_at);
