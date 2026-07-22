-- =====================================================================
-- 018 USER SESSIONS: server-side sessions stored per-tenant
-- =====================================================================
--
-- Web sign-in issues an opaque random token stored client-side in a
-- cookie. The token maps to a row in this table (belonging to the
-- tenant DB the user signed into), and every session-gated request
-- validates that mapping.
--
-- Rows are only ever inserted here — we mark them "revoked" via a
-- non-null `revoked_at`, and a background sweep can delete rows that
-- have been revoked or expired for a long time. This gives us an
-- audit trail (last-N logins per user) for free.
--
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS user_session (
    id            INTEGER PRIMARY KEY,
    token         TEXT    NOT NULL UNIQUE,
    user_id       INTEGER NOT NULL REFERENCES user_account(id) ON DELETE CASCADE,
    created_at    TEXT    NOT NULL DEFAULT (datetime('now')),
    expires_at    TEXT    NOT NULL,
    last_seen_at  TEXT    NOT NULL DEFAULT (datetime('now')),
    revoked_at    TEXT,
    user_agent    TEXT,
    remote_ip     TEXT
);

CREATE INDEX IF NOT EXISTS ix_user_session_user   ON user_session(user_id);
CREATE INDEX IF NOT EXISTS ix_user_session_active ON user_session(expires_at) WHERE revoked_at IS NULL;
