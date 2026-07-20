-- =====================================================================
-- 002 ACADEMIC STRUCTURE: grade, section, room, subject, class_section
-- Note: class_section.class_teacher_id references staff(id) which is
-- created in the people migration; we add that FK later via a trigger
-- pattern is not needed because SQLite defers FK check to insert time
-- only if the referenced table exists. So we split the FKs carefully:
--   - room and subject have no external FKs
--   - class_section references staff/room but staff is created next.
-- To keep ordering clean, we declare class_section without the staff FK
-- here and add it in the people migration via a supporting table.
-- =====================================================================
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS grade (
    id      INTEGER PRIMARY KEY,
    name    TEXT NOT NULL UNIQUE,
    level   INTEGER NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS section (
    id      INTEGER PRIMARY KEY,
    name    TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS room (
    id       INTEGER PRIMARY KEY,
    name     TEXT NOT NULL UNIQUE,
    capacity INTEGER,
    kind     TEXT CHECK (kind IN ('classroom','lab','library','auditorium','sports','other'))
);

CREATE TABLE IF NOT EXISTS subject (
    id          INTEGER PRIMARY KEY,
    code        TEXT NOT NULL UNIQUE,
    name        TEXT NOT NULL,
    is_elective INTEGER NOT NULL DEFAULT 0 CHECK (is_elective IN (0,1))
);
