-- =====================================================================
-- 011 TRANSPORT
-- =====================================================================
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS vehicle (
    id              INTEGER PRIMARY KEY,
    reg_number      TEXT NOT NULL UNIQUE,
    model           TEXT,
    capacity        INTEGER CHECK (capacity IS NULL OR capacity > 0),
    driver_staff_id INTEGER REFERENCES staff(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS route (
    id         INTEGER PRIMARY KEY,
    name       TEXT NOT NULL UNIQUE,
    vehicle_id INTEGER REFERENCES vehicle(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS route_stop (
    id          INTEGER PRIMARY KEY,
    route_id    INTEGER NOT NULL REFERENCES route(id) ON DELETE CASCADE,
    name        TEXT NOT NULL,
    stop_order  INTEGER NOT NULL,
    pickup_time TEXT,
    drop_time   TEXT,
    fare_cents  INTEGER NOT NULL DEFAULT 0 CHECK (fare_cents >= 0),
    UNIQUE (route_id, stop_order),
    UNIQUE (route_id, name)
);

CREATE TABLE IF NOT EXISTS student_transport (
    id                INTEGER PRIMARY KEY,
    student_id        INTEGER NOT NULL REFERENCES student(id) ON DELETE CASCADE,
    route_stop_id     INTEGER NOT NULL REFERENCES route_stop(id) ON DELETE RESTRICT,
    academic_year_id  INTEGER NOT NULL REFERENCES academic_year(id) ON DELETE RESTRICT,
    valid_from        TEXT NOT NULL,
    valid_to          TEXT,
    UNIQUE (student_id, academic_year_id),
    CHECK (valid_to IS NULL OR valid_from <= valid_to)
);
