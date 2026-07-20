-- =====================================================================
-- 009 PAYROLL
-- =====================================================================
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS salary_component (
    id     INTEGER PRIMARY KEY,
    name   TEXT NOT NULL UNIQUE,
    kind   TEXT NOT NULL CHECK (kind IN ('earning','deduction'))
);

CREATE TABLE IF NOT EXISTS salary_structure (
    id             INTEGER PRIMARY KEY,
    staff_id       INTEGER NOT NULL REFERENCES staff(id) ON DELETE CASCADE,
    effective_from TEXT NOT NULL,
    effective_to   TEXT,
    CHECK (effective_to IS NULL OR effective_from <= effective_to)
);
CREATE INDEX IF NOT EXISTS ix_salary_struct_staff ON salary_structure(staff_id, effective_from);

CREATE TABLE IF NOT EXISTS salary_structure_item (
    id                   INTEGER PRIMARY KEY,
    salary_structure_id  INTEGER NOT NULL REFERENCES salary_structure(id) ON DELETE CASCADE,
    component_id         INTEGER NOT NULL REFERENCES salary_component(id) ON DELETE RESTRICT,
    amount_cents         INTEGER NOT NULL CHECK (amount_cents >= 0),
    UNIQUE (salary_structure_id, component_id)
);

CREATE TABLE IF NOT EXISTS payslip (
    id              INTEGER PRIMARY KEY,
    staff_id        INTEGER NOT NULL REFERENCES staff(id) ON DELETE RESTRICT,
    period_month    INTEGER NOT NULL CHECK (period_month BETWEEN 1 AND 12),
    period_year     INTEGER NOT NULL CHECK (period_year >= 2000),
    gross_cents     INTEGER NOT NULL CHECK (gross_cents >= 0),
    deduction_cents INTEGER NOT NULL CHECK (deduction_cents >= 0),
    net_cents       INTEGER NOT NULL CHECK (net_cents >= 0),
    paid_on         TEXT,
    status          TEXT NOT NULL DEFAULT 'draft'
                    CHECK (status IN ('draft','approved','paid','cancelled')),
    created_at      TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE (staff_id, period_year, period_month)
);

CREATE TABLE IF NOT EXISTS payslip_line (
    id             INTEGER PRIMARY KEY,
    payslip_id     INTEGER NOT NULL REFERENCES payslip(id) ON DELETE CASCADE,
    component_id   INTEGER NOT NULL REFERENCES salary_component(id) ON DELETE RESTRICT,
    amount_cents   INTEGER NOT NULL,
    UNIQUE (payslip_id, component_id)
);
