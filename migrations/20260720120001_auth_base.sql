-- =====================================================================
-- 001 AUTH BASE: minimal user + role + permission tables so login,
-- session hydration, and the RBAC helpers keep working. Business
-- schema (students, staff, fees, ...) lives in later migrations and
-- is intentionally empty right now — add one migration per module as
-- you build it out.
-- =====================================================================
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS user_account (
    id             INTEGER PRIMARY KEY,
    username       TEXT NOT NULL UNIQUE,
    email          TEXT UNIQUE,
    password_hash  TEXT NOT NULL,
    is_active      INTEGER NOT NULL DEFAULT 1 CHECK (is_active IN (0,1)),
    last_login_at  TEXT,
    created_at     TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS role (
    id   INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS permission (
    id    INTEGER PRIMARY KEY,
    code  TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS role_permission (
    role_id       INTEGER NOT NULL REFERENCES role(id) ON DELETE CASCADE,
    permission_id INTEGER NOT NULL REFERENCES permission(id) ON DELETE CASCADE,
    PRIMARY KEY (role_id, permission_id)
);

CREATE TABLE IF NOT EXISTS user_role (
    user_id INTEGER NOT NULL REFERENCES user_account(id) ON DELETE CASCADE,
    role_id INTEGER NOT NULL REFERENCES role(id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, role_id)
);
