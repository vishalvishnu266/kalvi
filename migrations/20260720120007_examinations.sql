-- =====================================================================
-- 007 EXAMINATIONS + GRADING
-- =====================================================================
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS grading_scale (
    id     INTEGER PRIMARY KEY,
    name   TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS grade_band (
    id                INTEGER PRIMARY KEY,
    grading_scale_id  INTEGER NOT NULL REFERENCES grading_scale(id) ON DELETE CASCADE,
    letter            TEXT NOT NULL,
    min_percent       REAL NOT NULL,
    max_percent       REAL NOT NULL,
    grade_point       REAL,
    remarks           TEXT,
    CHECK (min_percent <= max_percent),
    UNIQUE (grading_scale_id, letter)
);

CREATE TABLE IF NOT EXISTS exam (
    id                INTEGER PRIMARY KEY,
    term_id           INTEGER NOT NULL REFERENCES term(id) ON DELETE CASCADE,
    name              TEXT NOT NULL,
    weightage         REAL NOT NULL DEFAULT 1.0,
    grading_scale_id  INTEGER REFERENCES grading_scale(id) ON DELETE SET NULL,
    UNIQUE (term_id, name)
);

CREATE TABLE IF NOT EXISTS exam_schedule (
    id                  INTEGER PRIMARY KEY,
    exam_id             INTEGER NOT NULL REFERENCES exam(id) ON DELETE CASCADE,
    class_section_id    INTEGER NOT NULL REFERENCES class_section(id) ON DELETE CASCADE,
    subject_id          INTEGER NOT NULL REFERENCES subject(id) ON DELETE RESTRICT,
    exam_date           TEXT NOT NULL,
    start_time          TEXT,
    end_time            TEXT,
    max_marks           REAL NOT NULL CHECK (max_marks > 0),
    pass_marks          REAL NOT NULL CHECK (pass_marks >= 0),
    room_id             INTEGER REFERENCES room(id) ON DELETE SET NULL,
    UNIQUE (exam_id, class_section_id, subject_id)
);
CREATE INDEX IF NOT EXISTS ix_exam_schedule_date ON exam_schedule(exam_date);

CREATE TABLE IF NOT EXISTS exam_result (
    id                  INTEGER PRIMARY KEY,
    exam_schedule_id    INTEGER NOT NULL REFERENCES exam_schedule(id) ON DELETE CASCADE,
    student_id          INTEGER NOT NULL REFERENCES student(id) ON DELETE CASCADE,
    marks_obtained      REAL,
    grade_letter        TEXT,
    is_absent           INTEGER NOT NULL DEFAULT 0 CHECK (is_absent IN (0,1)),
    remarks             TEXT,
    entered_by_staff_id INTEGER REFERENCES staff(id) ON DELETE SET NULL,
    entered_at          TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE (exam_schedule_id, student_id)
);
CREATE INDEX IF NOT EXISTS ix_exam_result_student ON exam_result(student_id);
