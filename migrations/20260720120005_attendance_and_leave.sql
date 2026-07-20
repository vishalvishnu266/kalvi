-- =====================================================================
-- 005 ATTENDANCE + LEAVE (student & staff)
-- =====================================================================
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS student_attendance (
    id                 INTEGER PRIMARY KEY,
    student_id         INTEGER NOT NULL REFERENCES student(id) ON DELETE CASCADE,
    class_section_id   INTEGER NOT NULL REFERENCES class_section(id) ON DELETE RESTRICT,
    date               TEXT NOT NULL,
    status             TEXT NOT NULL CHECK (status IN ('present','absent','late','excused','half_day')),
    remarks            TEXT,
    marked_by_staff_id INTEGER REFERENCES staff(id) ON DELETE SET NULL,
    marked_at          TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE (student_id, date)
);
CREATE INDEX IF NOT EXISTS ix_stu_att_class_date
    ON student_attendance(class_section_id, date);

CREATE TABLE IF NOT EXISTS staff_attendance (
    id            INTEGER PRIMARY KEY,
    staff_id      INTEGER NOT NULL REFERENCES staff(id) ON DELETE CASCADE,
    date          TEXT NOT NULL,
    status        TEXT NOT NULL CHECK (status IN ('present','absent','late','leave','half_day')),
    check_in      TEXT,
    check_out     TEXT,
    UNIQUE (staff_id, date)
);

-- Split leave requests to keep FK integrity (no polymorphic FKs in SQLite)
CREATE TABLE IF NOT EXISTS student_leave_request (
    id             INTEGER PRIMARY KEY,
    student_id     INTEGER NOT NULL REFERENCES student(id) ON DELETE CASCADE,
    from_date      TEXT NOT NULL,
    to_date        TEXT NOT NULL,
    reason         TEXT,
    status         TEXT NOT NULL DEFAULT 'pending'
                   CHECK (status IN ('pending','approved','rejected','cancelled')),
    approver_id    INTEGER REFERENCES staff(id) ON DELETE SET NULL,
    created_at     TEXT NOT NULL DEFAULT (datetime('now')),
    CHECK (from_date <= to_date)
);

CREATE TABLE IF NOT EXISTS staff_leave_request (
    id             INTEGER PRIMARY KEY,
    staff_id       INTEGER NOT NULL REFERENCES staff(id) ON DELETE CASCADE,
    from_date      TEXT NOT NULL,
    to_date        TEXT NOT NULL,
    reason         TEXT,
    status         TEXT NOT NULL DEFAULT 'pending'
                   CHECK (status IN ('pending','approved','rejected','cancelled')),
    approver_id    INTEGER REFERENCES staff(id) ON DELETE SET NULL,
    created_at     TEXT NOT NULL DEFAULT (datetime('now')),
    CHECK (from_date <= to_date)
);
