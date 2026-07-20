-- =====================================================================
-- 004 CLASS SECTION + ENROLLMENT (depends on grade/section/room + staff)
-- =====================================================================
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS class_section (
    id                  INTEGER PRIMARY KEY,
    academic_year_id    INTEGER NOT NULL REFERENCES academic_year(id) ON DELETE RESTRICT,
    grade_id            INTEGER NOT NULL REFERENCES grade(id) ON DELETE RESTRICT,
    section_id          INTEGER NOT NULL REFERENCES section(id) ON DELETE RESTRICT,
    class_teacher_id    INTEGER REFERENCES staff(id) ON DELETE SET NULL,
    room_id             INTEGER REFERENCES room(id) ON DELETE SET NULL,
    capacity            INTEGER,
    UNIQUE (academic_year_id, grade_id, section_id)
);
CREATE INDEX IF NOT EXISTS ix_class_section_year ON class_section(academic_year_id);

CREATE TABLE IF NOT EXISTS class_subject (
    id                  INTEGER PRIMARY KEY,
    class_section_id    INTEGER NOT NULL REFERENCES class_section(id) ON DELETE CASCADE,
    subject_id          INTEGER NOT NULL REFERENCES subject(id) ON DELETE RESTRICT,
    teacher_id          INTEGER REFERENCES staff(id) ON DELETE SET NULL,
    UNIQUE (class_section_id, subject_id)
);
CREATE INDEX IF NOT EXISTS ix_class_subject_teacher ON class_subject(teacher_id);

CREATE TABLE IF NOT EXISTS enrollment (
    id                 INTEGER PRIMARY KEY,
    student_id         INTEGER NOT NULL REFERENCES student(id) ON DELETE CASCADE,
    class_section_id   INTEGER NOT NULL REFERENCES class_section(id) ON DELETE RESTRICT,
    academic_year_id   INTEGER NOT NULL REFERENCES academic_year(id) ON DELETE RESTRICT,
    roll_no            INTEGER,
    enrolled_on        TEXT NOT NULL DEFAULT (date('now')),
    left_on            TEXT,
    result             TEXT CHECK (result IN ('promoted','retained','failed','transferred','pending')),
    UNIQUE (student_id, academic_year_id)
);
CREATE INDEX IF NOT EXISTS ix_enrollment_class ON enrollment(class_section_id);
CREATE UNIQUE INDEX IF NOT EXISTS ux_enrollment_roll
    ON enrollment(class_section_id, roll_no) WHERE roll_no IS NOT NULL;
