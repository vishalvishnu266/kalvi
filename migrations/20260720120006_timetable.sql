-- =====================================================================
-- 006 TIMETABLE: periods + timetable_slot (with double-booking guards)
-- =====================================================================
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS period (
    id         INTEGER PRIMARY KEY,
    name       TEXT NOT NULL UNIQUE,
    start_time TEXT NOT NULL,
    end_time   TEXT NOT NULL,
    is_break   INTEGER NOT NULL DEFAULT 0 CHECK (is_break IN (0,1)),
    CHECK (start_time < end_time)
);

CREATE TABLE IF NOT EXISTS timetable_slot (
    id                  INTEGER PRIMARY KEY,
    class_section_id    INTEGER NOT NULL REFERENCES class_section(id) ON DELETE CASCADE,
    subject_id          INTEGER REFERENCES subject(id) ON DELETE SET NULL,
    teacher_id          INTEGER REFERENCES staff(id) ON DELETE SET NULL,
    room_id             INTEGER REFERENCES room(id) ON DELETE SET NULL,
    day_of_week         INTEGER NOT NULL CHECK (day_of_week BETWEEN 1 AND 7),
    period_id           INTEGER NOT NULL REFERENCES period(id) ON DELETE RESTRICT,
    UNIQUE (class_section_id, day_of_week, period_id)
);

-- Partial unique indexes prevent double-booking without blocking NULLs
CREATE UNIQUE INDEX IF NOT EXISTS ux_slot_teacher
    ON timetable_slot(teacher_id, day_of_week, period_id)
    WHERE teacher_id IS NOT NULL;

CREATE UNIQUE INDEX IF NOT EXISTS ux_slot_room
    ON timetable_slot(room_id, day_of_week, period_id)
    WHERE room_id IS NOT NULL;
