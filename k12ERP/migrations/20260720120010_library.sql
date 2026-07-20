-- =====================================================================
-- 010 LIBRARY
-- =====================================================================
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS book (
    id            INTEGER PRIMARY KEY,
    isbn          TEXT,
    title         TEXT NOT NULL,
    author        TEXT,
    publisher     TEXT,
    category      TEXT,
    total_copies  INTEGER NOT NULL DEFAULT 1 CHECK (total_copies >= 0),
    available     INTEGER NOT NULL DEFAULT 1 CHECK (available >= 0),
    CHECK (available <= total_copies)
);
CREATE INDEX IF NOT EXISTS ix_book_title ON book(title);
CREATE INDEX IF NOT EXISTS ix_book_isbn  ON book(isbn);

CREATE TABLE IF NOT EXISTS book_issue (
    id           INTEGER PRIMARY KEY,
    book_id      INTEGER NOT NULL REFERENCES book(id) ON DELETE RESTRICT,
    student_id   INTEGER REFERENCES student(id) ON DELETE SET NULL,
    staff_id     INTEGER REFERENCES staff(id) ON DELETE SET NULL,
    issued_on    TEXT NOT NULL,
    due_on       TEXT NOT NULL,
    returned_on  TEXT,
    fine_cents   INTEGER NOT NULL DEFAULT 0 CHECK (fine_cents >= 0),
    CHECK (student_id IS NOT NULL OR staff_id IS NOT NULL),
    CHECK (issued_on <= due_on)
);
CREATE INDEX IF NOT EXISTS ix_book_issue_open
    ON book_issue(book_id) WHERE returned_on IS NULL;
