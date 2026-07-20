-- =====================================================================
-- 017 SEED: minimal reference data (idempotent via INSERT OR IGNORE)
-- =====================================================================
PRAGMA foreign_keys = ON;

-- Single-tenant school row
INSERT OR IGNORE INTO school (id, name) VALUES (1, 'My School');

-- Grades K-12
INSERT OR IGNORE INTO grade (name, level) VALUES
    ('Pre-K', 0),
    ('Kindergarten', 1),
    ('Grade 1', 2),
    ('Grade 2', 3),
    ('Grade 3', 4),
    ('Grade 4', 5),
    ('Grade 5', 6),
    ('Grade 6', 7),
    ('Grade 7', 8),
    ('Grade 8', 9),
    ('Grade 9', 10),
    ('Grade 10', 11),
    ('Grade 11', 12),
    ('Grade 12', 13);

-- Default sections
INSERT OR IGNORE INTO section (name) VALUES ('A'),('B'),('C'),('D');

-- Standard roles
INSERT OR IGNORE INTO role (name) VALUES
    ('admin'),('principal'),('teacher'),('accountant'),
    ('librarian'),('student'),('guardian');

-- Fee categories
INSERT OR IGNORE INTO fee_category (name) VALUES
    ('Tuition'),('Admission'),('Exam'),('Transport'),
    ('Hostel'),('Library'),('Lab'),('Uniform'),('Miscellaneous');

-- Salary components
INSERT OR IGNORE INTO salary_component (name, kind) VALUES
    ('Basic','earning'),
    ('HRA','earning'),
    ('DA','earning'),
    ('Transport Allowance','earning'),
    ('Provident Fund','deduction'),
    ('Tax','deduction'),
    ('Loan Deduction','deduction');

-- Ledger accounts (minimal chart of accounts)
INSERT OR IGNORE INTO ledger_account (code, name, type) VALUES
    ('1000','Cash','asset'),
    ('1010','Bank','asset'),
    ('1200','Accounts Receivable','asset'),
    ('2000','Accounts Payable','liability'),
    ('4000','Tuition Income','income'),
    ('4100','Transport Income','income'),
    ('4200','Hostel Income','income'),
    ('5000','Salary Expense','expense'),
    ('5100','Utilities Expense','expense'),
    ('5200','Supplies Expense','expense');
