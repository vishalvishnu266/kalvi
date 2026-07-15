-- ============================================================
-- SchoolDesk ERP - Authentication (per tenant)
--   * users    : local accounts. Role gates access (Phase 0.3).
--   * sessions : opaque server-side session tokens (cookie-backed).
--
-- We keep both tables per-tenant so that a person with the same email
-- at two institutions has two distinct logins/passwords — the norm for
-- multi-tenant ERPs.
-- ============================================================

CREATE TABLE IF NOT EXISTS users (
    id            TEXT PRIMARY KEY,           -- UUID
    email         TEXT NOT NULL UNIQUE COLLATE NOCASE,
    password_hash TEXT NOT NULL,              -- Argon2id encoded hash
    full_name     TEXT NOT NULL,

    -- Role gates access. Extended in Phase 0.3 with per-route checks.
    -- Accepted values are validated in Rust; DB CHECK is a safety net.
    role          TEXT NOT NULL DEFAULT 'admin'
                     CHECK (role IN (
                        'admin', 'teacher', 'accountant',
                        'librarian', 'student', 'guardian'
                     )),

    status        TEXT NOT NULL DEFAULT 'active'
                     CHECK (status IN ('active', 'suspended')),

    -- Optional soft links to related domain entities (populated in later
    -- phases so a Teacher/Student can see "their" data).
    teacher_id    TEXT,   -- FK to teachers.id (Phase 1)
    student_id    TEXT REFERENCES students(id) ON DELETE SET NULL,

    last_login_at DATETIME,
    created_at    DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at    DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_users_role   ON users(role);
CREATE INDEX IF NOT EXISTS idx_users_status ON users(status);

-- ------------------------------------------------------------
-- Opaque server-side sessions.
-- id   : cryptographically-random 32-byte token, base64url-encoded.
-- We DELETE rows on logout; also periodically prune expired sessions.
-- ------------------------------------------------------------
CREATE TABLE IF NOT EXISTS sessions (
    id          TEXT PRIMARY KEY,
    user_id     TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at  DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at  DATETIME NOT NULL,
    ip          TEXT,
    user_agent  TEXT
);

CREATE INDEX IF NOT EXISTS idx_sessions_user_id    ON sessions(user_id);
CREATE INDEX IF NOT EXISTS idx_sessions_expires_at ON sessions(expires_at);

-- ------------------------------------------------------------
-- Seed a default admin so operators can log in immediately.
--   Email    : admin@school.edu
--   Password : admin123    (⚠ CHANGE ON FIRST LOGIN — dev only!)
-- The password_hash below is a pre-computed Argon2id hash of "admin123"
-- using the OWASP-recommended parameters (m=19456, t=2, p=1).
-- ------------------------------------------------------------
INSERT OR IGNORE INTO users (id, email, password_hash, full_name, role, status)
VALUES (
    'seed-admin',
    'admin@school.edu',
    '$argon2id$v=19$m=19456,t=2,p=1$c2VlZHNhbHRzZWVkc2FsdA$oO4kavlM3lzsUmwWNaeQMYPewInhScxNi9E3FLDcUgo',
    'System Admin',
    'admin',
    'active'
);
