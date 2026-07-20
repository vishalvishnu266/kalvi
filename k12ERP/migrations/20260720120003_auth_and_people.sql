-- =====================================================================
-- 003 AUTH + PEOPLE: users, roles, permissions, student, guardian, staff
-- =====================================================================
PRAGMA foreign_keys = ON;

-- ---------- Auth ----------
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

-- ---------- People ----------
CREATE TABLE IF NOT EXISTS department (
    id   INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS staff (
    id                INTEGER PRIMARY KEY,
    employee_no       TEXT NOT NULL UNIQUE,
    user_id           INTEGER UNIQUE REFERENCES user_account(id) ON DELETE SET NULL,
    department_id     INTEGER REFERENCES department(id) ON DELETE SET NULL,
    first_name        TEXT NOT NULL,
    last_name         TEXT NOT NULL,
    date_of_birth     TEXT,
    gender            TEXT CHECK (gender IN ('male','female','other')),
    phone             TEXT,
    email             TEXT,
    designation       TEXT,
    employment_type   TEXT CHECK (employment_type IN ('full_time','part_time','contract','intern')),
    date_of_joining   TEXT NOT NULL,
    date_of_leaving   TEXT,
    status            TEXT NOT NULL DEFAULT 'active'
                      CHECK (status IN ('active','on_leave','terminated','retired')),
    photo_path        TEXT,
    created_at        TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at        TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX IF NOT EXISTS ix_staff_name ON staff(last_name, first_name);

CREATE TABLE IF NOT EXISTS student (
    id                 INTEGER PRIMARY KEY,
    admission_no       TEXT NOT NULL UNIQUE,
    user_id            INTEGER UNIQUE REFERENCES user_account(id) ON DELETE SET NULL,
    first_name         TEXT NOT NULL,
    middle_name        TEXT,
    last_name          TEXT NOT NULL,
    date_of_birth      TEXT NOT NULL,
    gender             TEXT CHECK (gender IN ('male','female','other')),
    blood_group        TEXT,
    nationality        TEXT,
    religion           TEXT,
    photo_path         TEXT,
    admission_date     TEXT NOT NULL,
    status             TEXT NOT NULL DEFAULT 'active'
                       CHECK (status IN ('active','inactive','graduated','transferred','withdrawn')),
    address_line1      TEXT,
    address_line2      TEXT,
    city               TEXT,
    state              TEXT,
    postal_code        TEXT,
    country            TEXT,
    created_at         TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at         TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX IF NOT EXISTS ix_student_name ON student(last_name, first_name);
CREATE INDEX IF NOT EXISTS ix_student_status ON student(status);

CREATE TABLE IF NOT EXISTS guardian (
    id             INTEGER PRIMARY KEY,
    user_id        INTEGER UNIQUE REFERENCES user_account(id) ON DELETE SET NULL,
    first_name     TEXT NOT NULL,
    last_name      TEXT NOT NULL,
    phone          TEXT,
    email          TEXT,
    occupation     TEXT,
    address        TEXT,
    created_at     TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS student_guardian (
    student_id     INTEGER NOT NULL REFERENCES student(id) ON DELETE CASCADE,
    guardian_id    INTEGER NOT NULL REFERENCES guardian(id) ON DELETE CASCADE,
    relationship   TEXT NOT NULL,
    is_primary     INTEGER NOT NULL DEFAULT 0 CHECK (is_primary IN (0,1)),
    is_emergency   INTEGER NOT NULL DEFAULT 0 CHECK (is_emergency IN (0,1)),
    can_pickup     INTEGER NOT NULL DEFAULT 1 CHECK (can_pickup IN (0,1)),
    PRIMARY KEY (student_id, guardian_id)
);
CREATE INDEX IF NOT EXISTS ix_sg_guardian ON student_guardian(guardian_id);
