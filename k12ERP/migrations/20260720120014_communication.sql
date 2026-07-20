-- =====================================================================
-- 014 COMMUNICATION: announcements, messages, notifications
-- =====================================================================
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS announcement (
    id                 INTEGER PRIMARY KEY,
    title              TEXT NOT NULL,
    body               TEXT NOT NULL,
    audience           TEXT NOT NULL
                       CHECK (audience IN ('all','students','staff','guardians','class')),
    class_section_id   INTEGER REFERENCES class_section(id) ON DELETE CASCADE,
    published_at       TEXT NOT NULL DEFAULT (datetime('now')),
    expires_at         TEXT,
    created_by_user_id INTEGER REFERENCES user_account(id) ON DELETE SET NULL,
    CHECK (audience <> 'class' OR class_section_id IS NOT NULL)
);
CREATE INDEX IF NOT EXISTS ix_announcement_pub ON announcement(published_at);

CREATE TABLE IF NOT EXISTS message (
    id           INTEGER PRIMARY KEY,
    from_user_id INTEGER REFERENCES user_account(id) ON DELETE SET NULL,
    to_user_id   INTEGER REFERENCES user_account(id) ON DELETE SET NULL,
    subject      TEXT,
    body         TEXT NOT NULL,
    sent_at      TEXT NOT NULL DEFAULT (datetime('now')),
    read_at      TEXT
);
CREATE INDEX IF NOT EXISTS ix_message_inbox
    ON message(to_user_id, read_at);

CREATE TABLE IF NOT EXISTS notification (
    id           INTEGER PRIMARY KEY,
    user_id      INTEGER NOT NULL REFERENCES user_account(id) ON DELETE CASCADE,
    title        TEXT NOT NULL,
    body         TEXT,
    kind         TEXT,     -- 'fee_due','attendance','exam_result',...
    ref_type     TEXT,
    ref_id       INTEGER,
    created_at   TEXT NOT NULL DEFAULT (datetime('now')),
    read_at      TEXT
);
CREATE INDEX IF NOT EXISTS ix_notification_user
    ON notification(user_id, read_at);
