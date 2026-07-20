-- =====================================================================
-- 015 HEALTH + DISCIPLINE
-- =====================================================================
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS health_record (
    id           INTEGER PRIMARY KEY,
    student_id   INTEGER NOT NULL UNIQUE REFERENCES student(id) ON DELETE CASCADE,
    height_cm    REAL CHECK (height_cm IS NULL OR height_cm >= 0),
    weight_kg    REAL CHECK (weight_kg IS NULL OR weight_kg >= 0),
    allergies    TEXT,
    conditions   TEXT,
    updated_at   TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS vaccination (
    id           INTEGER PRIMARY KEY,
    student_id   INTEGER NOT NULL REFERENCES student(id) ON DELETE CASCADE,
    vaccine_name TEXT NOT NULL,
    dose         TEXT,
    given_on     TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS ix_vacc_student ON vaccination(student_id);

CREATE TABLE IF NOT EXISTS clinic_visit (
    id                   INTEGER PRIMARY KEY,
    student_id           INTEGER NOT NULL REFERENCES student(id) ON DELETE CASCADE,
    visited_at           TEXT NOT NULL DEFAULT (datetime('now')),
    complaint            TEXT,
    treatment            TEXT,
    attended_by_staff_id INTEGER REFERENCES staff(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS discipline_incident (
    id                    INTEGER PRIMARY KEY,
    student_id            INTEGER NOT NULL REFERENCES student(id) ON DELETE CASCADE,
    date                  TEXT NOT NULL,
    description           TEXT NOT NULL,
    severity              TEXT CHECK (severity IN ('low','medium','high')),
    action_taken          TEXT,
    reported_by_staff_id  INTEGER REFERENCES staff(id) ON DELETE SET NULL,
    created_at            TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX IF NOT EXISTS ix_discipline_student ON discipline_incident(student_id, date);
