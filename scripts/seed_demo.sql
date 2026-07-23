-- =============================================================================
-- scripts/seed_demo.sql
-- -----------------------------------------------------------------------------
-- Offline / bulk seed for a demo tenant DB. Nothing in the app depends on this
-- file — the application ships with no built-in mock data. This exists purely
-- as a convenience for populating dev / demo screens with realistic rows.
--
-- Run this AFTER the tenant DB has been provisioned (i.e. after all migrations
-- have run against `<tenant>.db`, either by the server on first request or by
-- creating the tenant via POST /admin/api/tenants).
--
-- Usage:
--     sqlite3 data/tenants/demo.db < scripts/seed_demo.sql
--
-- Everything below uses INSERT OR IGNORE, so the script is idempotent — you
-- can run it repeatedly without producing duplicates.
-- =============================================================================
PRAGMA foreign_keys = ON;

BEGIN;

-- ---------------------------------------------------------------------------
-- Departments (used by the staff FK).
-- ---------------------------------------------------------------------------
INSERT OR IGNORE INTO department (name) VALUES
    ('Administration'),
    ('Mathematics'),
    ('Sciences'),
    ('Languages'),
    ('Library'),
    ('Finance'),
    ('Physical Education'),
    ('IT');

-- ---------------------------------------------------------------------------
-- Staff — 8 rows spanning several departments and roles.
-- ---------------------------------------------------------------------------
INSERT OR IGNORE INTO staff (
    employee_no, department_id, first_name, last_name,
    phone, email, designation, employment_type,
    date_of_joining, status
) VALUES
    ('EMP-DEMO-001',
        (SELECT id FROM department WHERE name = 'Administration'),
        'Meera', 'Iyer', '+91-98100-00001', 'meera.iyer@demo.example',
        'Principal',        'full_time', '2020-04-01', 'active'),
    ('EMP-DEMO-002',
        (SELECT id FROM department WHERE name = 'Mathematics'),
        'Rohit', 'Verma', '+91-98100-00002', 'rohit.verma@demo.example',
        'Math Teacher',     'full_time', '2021-06-15', 'active'),
    ('EMP-DEMO-003',
        (SELECT id FROM department WHERE name = 'Sciences'),
        'Sana', 'Ali', '+91-98100-00003', 'sana.ali@demo.example',
        'Science Teacher',  'full_time', '2022-04-01', 'active'),
    ('EMP-DEMO-004',
        (SELECT id FROM department WHERE name = 'Library'),
        'David', 'Thomas', '+91-98100-00004', 'david.thomas@demo.example',
        'Librarian',        'part_time', '2023-08-01', 'active'),
    ('EMP-DEMO-005',
        (SELECT id FROM department WHERE name = 'Finance'),
        'Neha', 'Kapoor', '+91-98100-00005', 'neha.kapoor@demo.example',
        'Accountant',       'full_time', '2023-01-10', 'active'),
    ('EMP-DEMO-006',
        (SELECT id FROM department WHERE name = 'Physical Education'),
        'Arjun', 'Nair', '+91-98100-00006', 'arjun.nair@demo.example',
        'PE Teacher',       'full_time', '2022-04-01', 'active'),
    ('EMP-DEMO-007',
        (SELECT id FROM department WHERE name = 'Languages'),
        'Priya', 'Menon', '+91-98100-00007', 'priya.menon@demo.example',
        'English Teacher',  'full_time', '2021-04-01', 'active'),
    ('EMP-DEMO-008',
        (SELECT id FROM department WHERE name = 'IT'),
        'Amit', 'Gupta', '+91-98100-00008', 'amit.gupta@demo.example',
        'IT Administrator', 'full_time', '2020-09-15', 'active');

-- ---------------------------------------------------------------------------
-- Students — 10 rows across grades, both genders, with basic address info.
-- ---------------------------------------------------------------------------
INSERT OR IGNORE INTO student (
    admission_no, first_name, last_name,
    date_of_birth, gender, nationality,
    admission_date, status,
    city, state, country
) VALUES
    ('ADM-DEMO-0001', 'Aarav',  'Sharma',    '2012-03-15', 'male',   'Indian', '2024-04-01', 'active',   'Mumbai', 'MH', 'India'),
    ('ADM-DEMO-0002', 'Diya',   'Patel',     '2010-07-08', 'female', 'Indian', '2024-04-01', 'active',   'Mumbai', 'MH', 'India'),
    ('ADM-DEMO-0003', 'Kabir',  'Khan',      '2014-01-22', 'male',   'Indian', '2024-04-01', 'active',   'Mumbai', 'MH', 'India'),
    ('ADM-DEMO-0004', 'Ananya', 'Rao',       '2008-11-30', 'female', 'Indian', '2024-04-01', 'inactive', 'Pune',   'MH', 'India'),
    ('ADM-DEMO-0005', 'Vihaan', 'Mehta',     '2011-05-19', 'male',   'Indian', '2024-04-01', 'active',   'Mumbai', 'MH', 'India'),
    ('ADM-DEMO-0006', 'Isha',   'Bhatt',     '2013-09-04', 'female', 'Indian', '2024-04-01', 'active',   'Mumbai', 'MH', 'India'),
    ('ADM-DEMO-0007', 'Rohan',  'Desai',     '2009-02-14', 'male',   'Indian', '2024-04-01', 'active',   'Mumbai', 'MH', 'India'),
    ('ADM-DEMO-0008', 'Meera',  'Krishna',   '2012-12-25', 'female', 'Indian', '2024-04-01', 'active',   'Mumbai', 'MH', 'India'),
    ('ADM-DEMO-0009', 'Yash',   'Joshi',     '2010-08-17', 'male',   'Indian', '2024-04-01', 'active',   'Mumbai', 'MH', 'India'),
    ('ADM-DEMO-0010', 'Sara',   'Fernandes', '2011-06-06', 'female', 'Indian', '2024-04-01', 'active',   'Mumbai', 'MH', 'India');

COMMIT;

-- Sanity summary.
SELECT 'staff'   AS kind, COUNT(*) AS n FROM staff
UNION ALL
SELECT 'student' AS kind, COUNT(*) AS n FROM student;
