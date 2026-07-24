CREATE TABLE IF NOT EXISTS user_session (
    id            INTEGER PRIMARY KEY,
    token         TEXT    NOT NULL UNIQUE,
    user_id       INTEGER NOT NULL,
    tenant_id     TEXT,
    created_at    TEXT    NOT NULL DEFAULT (datetime('now')),
    expires_at    TEXT    NOT NULL,
    last_seen_at  TEXT    NOT NULL DEFAULT (datetime('now')),
    revoked_at    TEXT,
    user_agent    TEXT,
    remote_ip     TEXT
);

CREATE INDEX IF NOT EXISTS ix_user_session_user ON user_session(user_id);
CREATE INDEX IF NOT EXISTS ix_user_session_active ON user_session(expires_at) WHERE revoked_at IS NULL;
