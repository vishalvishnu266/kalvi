-- =====================================================================
-- 008 FEES + FINANCE (money stored as INTEGER cents)
-- =====================================================================
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS fee_category (
    id      INTEGER PRIMARY KEY,
    name    TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS fee_structure (
    id                INTEGER PRIMARY KEY,
    academic_year_id  INTEGER NOT NULL REFERENCES academic_year(id) ON DELETE CASCADE,
    grade_id          INTEGER NOT NULL REFERENCES grade(id) ON DELETE RESTRICT,
    name              TEXT NOT NULL,
    UNIQUE (academic_year_id, grade_id, name)
);

CREATE TABLE IF NOT EXISTS fee_structure_item (
    id                INTEGER PRIMARY KEY,
    fee_structure_id  INTEGER NOT NULL REFERENCES fee_structure(id) ON DELETE CASCADE,
    fee_category_id   INTEGER NOT NULL REFERENCES fee_category(id) ON DELETE RESTRICT,
    amount_cents      INTEGER NOT NULL CHECK (amount_cents >= 0),
    frequency         TEXT NOT NULL
                      CHECK (frequency IN ('one_time','monthly','quarterly','termly','annually')),
    due_day           INTEGER
);

CREATE TABLE IF NOT EXISTS fee_invoice (
    id                 INTEGER PRIMARY KEY,
    invoice_no         TEXT NOT NULL UNIQUE,
    student_id         INTEGER NOT NULL REFERENCES student(id) ON DELETE RESTRICT,
    academic_year_id   INTEGER NOT NULL REFERENCES academic_year(id) ON DELETE RESTRICT,
    issue_date         TEXT NOT NULL,
    due_date           TEXT NOT NULL,
    subtotal_cents     INTEGER NOT NULL DEFAULT 0 CHECK (subtotal_cents >= 0),
    discount_cents     INTEGER NOT NULL DEFAULT 0 CHECK (discount_cents >= 0),
    tax_cents          INTEGER NOT NULL DEFAULT 0 CHECK (tax_cents >= 0),
    total_cents        INTEGER NOT NULL DEFAULT 0 CHECK (total_cents >= 0),
    paid_cents         INTEGER NOT NULL DEFAULT 0 CHECK (paid_cents >= 0),
    status             TEXT NOT NULL DEFAULT 'unpaid'
                       CHECK (status IN ('unpaid','partial','paid','overdue','cancelled','refunded')),
    notes              TEXT,
    created_at         TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX IF NOT EXISTS ix_invoice_student ON fee_invoice(student_id, status);
CREATE INDEX IF NOT EXISTS ix_invoice_due     ON fee_invoice(due_date, status);

CREATE TABLE IF NOT EXISTS fee_invoice_line (
    id                 INTEGER PRIMARY KEY,
    invoice_id         INTEGER NOT NULL REFERENCES fee_invoice(id) ON DELETE CASCADE,
    fee_category_id    INTEGER NOT NULL REFERENCES fee_category(id) ON DELETE RESTRICT,
    description        TEXT NOT NULL,
    amount_cents       INTEGER NOT NULL CHECK (amount_cents >= 0),
    discount_cents     INTEGER NOT NULL DEFAULT 0 CHECK (discount_cents >= 0)
);

CREATE TABLE IF NOT EXISTS fee_payment (
    id                   INTEGER PRIMARY KEY,
    receipt_no           TEXT NOT NULL UNIQUE,
    invoice_id           INTEGER NOT NULL REFERENCES fee_invoice(id) ON DELETE RESTRICT,
    paid_on              TEXT NOT NULL,
    amount_cents         INTEGER NOT NULL CHECK (amount_cents > 0),
    method               TEXT NOT NULL
                         CHECK (method IN ('cash','card','bank_transfer','cheque','upi','online','other')),
    reference            TEXT,
    received_by_staff_id INTEGER REFERENCES staff(id) ON DELETE SET NULL,
    created_at           TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX IF NOT EXISTS ix_payment_invoice ON fee_payment(invoice_id);

CREATE TABLE IF NOT EXISTS fee_discount (
    id             INTEGER PRIMARY KEY,
    student_id     INTEGER NOT NULL REFERENCES student(id) ON DELETE CASCADE,
    name           TEXT NOT NULL,
    percent        REAL CHECK (percent IS NULL OR (percent >= 0 AND percent <= 100)),
    flat_cents     INTEGER CHECK (flat_cents IS NULL OR flat_cents >= 0),
    valid_from     TEXT,
    valid_to       TEXT,
    CHECK (percent IS NOT NULL OR flat_cents IS NOT NULL)
);

CREATE TABLE IF NOT EXISTS ledger_account (
    id     INTEGER PRIMARY KEY,
    code   TEXT NOT NULL UNIQUE,
    name   TEXT NOT NULL,
    type   TEXT NOT NULL CHECK (type IN ('asset','liability','income','expense','equity'))
);

CREATE TABLE IF NOT EXISTS ledger_entry (
    id            INTEGER PRIMARY KEY,
    entry_date    TEXT NOT NULL,
    account_id    INTEGER NOT NULL REFERENCES ledger_account(id) ON DELETE RESTRICT,
    debit_cents   INTEGER NOT NULL DEFAULT 0 CHECK (debit_cents >= 0),
    credit_cents  INTEGER NOT NULL DEFAULT 0 CHECK (credit_cents >= 0),
    ref_type      TEXT,
    ref_id        INTEGER,
    memo          TEXT,
    CHECK (debit_cents = 0 OR credit_cents = 0)
);
CREATE INDEX IF NOT EXISTS ix_ledger_account_date ON ledger_entry(account_id, entry_date);
CREATE INDEX IF NOT EXISTS ix_ledger_ref ON ledger_entry(ref_type, ref_id);
