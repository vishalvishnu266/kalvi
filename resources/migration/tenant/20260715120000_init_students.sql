-- ============================================================
-- SchoolDesk ERP - Students module schema
-- Tenant DB (SQLite)
-- ============================================================

CREATE TABLE IF NOT EXISTS students (
    id                TEXT PRIMARY KEY,          -- UUID
    admission_no      TEXT NOT NULL UNIQUE,      -- e.g. STU-2026-1042
    first_name        TEXT NOT NULL,
    last_name         TEXT NOT NULL,
    email             TEXT,
    phone             TEXT,
    date_of_birth     DATE,
    gender            TEXT NOT NULL CHECK (gender IN ('Male','Female','Other')),
    blood_group       TEXT,

    -- Academic
    class_name        TEXT NOT NULL,             -- e.g. "10"
    section           TEXT NOT NULL,             -- e.g. "A"
    roll_no           TEXT NOT NULL,
    admission_date    DATE NOT NULL DEFAULT (DATE('now')),

    -- Guardian
    guardian_name     TEXT NOT NULL,
    guardian_phone    TEXT NOT NULL,
    guardian_email    TEXT,
    guardian_relation TEXT,                      -- Father / Mother / Guardian

    -- Address
    address_line      TEXT,
    city              TEXT,
    state             TEXT,
    postal_code       TEXT,

    -- Status
    status            TEXT NOT NULL DEFAULT 'Active'
                          CHECK (status IN ('Active','Inactive','Pending','Graduated','Suspended')),

    -- Audit
    created_at        DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at        DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_students_class     ON students(class_name, section);
CREATE INDEX IF NOT EXISTS idx_students_status    ON students(status);
CREATE INDEX IF NOT EXISTS idx_students_last_name ON students(last_name);

-- ------------------------------------------------------------
-- Seed data
-- ------------------------------------------------------------
INSERT INTO students (
    id, admission_no, first_name, last_name, email, phone, date_of_birth, gender,
    blood_group, class_name, section, roll_no, admission_date,
    guardian_name, guardian_phone, guardian_email, guardian_relation,
    address_line, city, state, postal_code, status
) VALUES
('11111111-1111-1111-1111-111111111111','STU-2024-1042','Aarav','Patel','aarav.p@school.edu','+91 90000 11111','2009-04-12','Male',
 'B+','10','A','1042','2024-06-01',
 'Rajesh Patel','+91 98765 43210','rajesh.p@example.com','Father',
 '221B Baker Street','Mumbai','MH','400001','Active'),

('22222222-2222-2222-2222-222222222222','STU-2024-0938','Sanya','Mehta','sanya.m@school.edu','+91 90000 22222','2010-11-03','Female',
 'A+','9','B','938','2024-06-02',
 'Suresh Mehta','+91 87654 32109','suresh.m@example.com','Father',
 '15 MG Road','Pune','MH','411001','Active'),

('33333333-3333-3333-3333-333333333333','STU-2024-1201','Rohan','Kumar','rohan.k@school.edu','+91 90000 33333','2007-08-25','Male',
 'O+','12','A','1201','2024-06-05',
 'Amit Kumar','+91 76543 21098','amit.k@example.com','Father',
 '5 Nehru Nagar','Delhi','DL','110001','Pending'),

('44444444-4444-4444-4444-444444444444','STU-2023-0812','Priya','Sharma','priya.s@school.edu','+91 90000 44444','2011-02-19','Female',
 'AB+','8','C','812','2023-06-10',
 'Deepak Sharma','+91 65432 10987','deepak.s@example.com','Father',
 '9 Ring Road','Jaipur','RJ','302001','Active'),

('55555555-5555-5555-5555-555555555555','STU-2023-1105','Vikram','Rao','vikram.r@school.edu','+91 90000 55555','2008-05-30','Male',
 'B-','11','B','1105','2023-06-12',
 'Narayan Rao','+91 54321 09876','narayan.r@example.com','Father',
 '77 Brigade Road','Bengaluru','KA','560001','Inactive'),

('66666666-6666-6666-6666-666666666666','STU-2024-0723','Neha','Gupta','neha.g@school.edu','+91 90000 66666','2010-09-14','Female',
 'O-','9','A','723','2024-06-08',
 'Vinod Gupta','+91 43210 98765','vinod.g@example.com','Father',
 '12 Park Street','Kolkata','WB','700001','Active');
