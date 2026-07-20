-- =====================================================================
-- 012 HOSTEL
-- =====================================================================
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS hostel (
    id     INTEGER PRIMARY KEY,
    name   TEXT NOT NULL UNIQUE,
    type   TEXT CHECK (type IN ('boys','girls','mixed'))
);

CREATE TABLE IF NOT EXISTS hostel_room (
    id         INTEGER PRIMARY KEY,
    hostel_id  INTEGER NOT NULL REFERENCES hostel(id) ON DELETE CASCADE,
    room_no    TEXT NOT NULL,
    capacity   INTEGER NOT NULL CHECK (capacity > 0),
    UNIQUE (hostel_id, room_no)
);

CREATE TABLE IF NOT EXISTS hostel_allocation (
    id             INTEGER PRIMARY KEY,
    student_id     INTEGER NOT NULL REFERENCES student(id) ON DELETE CASCADE,
    hostel_room_id INTEGER NOT NULL REFERENCES hostel_room(id) ON DELETE RESTRICT,
    from_date      TEXT NOT NULL,
    to_date        TEXT,
    CHECK (to_date IS NULL OR from_date <= to_date)
);

-- One active allocation per student at a time
CREATE UNIQUE INDEX IF NOT EXISTS ux_hostel_active_alloc
    ON hostel_allocation(student_id) WHERE to_date IS NULL;
