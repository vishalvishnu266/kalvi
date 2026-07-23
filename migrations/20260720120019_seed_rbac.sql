-- =====================================================================
-- 019 SEED: RBAC permission catalogue + baseline role bindings
-- ---------------------------------------------------------------------
-- This migration adds two things:
--
--   1. A canonical list of permission codes (one per screen/action).
--      Codes follow `"<module>.<action>"` (e.g. `students.view`,
--      `fees.collect`, `fees.pay`). Keep this list in lock-step with
--      the `perm::` catalogue in `src/services/auth/permissions.rs`.
--
--   2. Baseline role → permission bindings for the standard roles
--      seeded by migration 017 (admin, principal, teacher, accountant,
--      librarian, student, guardian).
--
-- Everything below uses INSERT OR IGNORE so the migration is idempotent
-- and safe to re-apply. Tenant admins can freely add/remove bindings
-- through a "Roles & permissions" screen without conflicting with this
-- baseline.
-- =====================================================================
PRAGMA foreign_keys = ON;

-- ---------- 1. Permission catalogue -----------------------------------
INSERT OR IGNORE INTO permission (code) VALUES
    -- ~ students
    ('students.view'),
    ('students.view_own'),      -- guardian-scoped: only their children
    ('students.edit'),
    ('students.admit'),
    -- ~ staff
    ('staff.view'),
    ('staff.edit'),
    ('staff.hire'),
    -- ~ academic
    ('academic.view'),
    ('academic.manage'),
    -- ~ attendance
    ('attendance.view'),
    ('attendance.view_own'),
    ('attendance.mark'),
    -- ~ timetable
    ('timetable.view'),
    ('timetable.manage'),
    -- ~ fees
    ('fees.view'),
    ('fees.view_own'),
    ('fees.collect'),
    ('fees.pay'),
    -- ~ examinations
    ('examinations.view'),
    ('examinations.view_own'),
    ('examinations.manage'),
    ('examinations.enter_marks'),
    -- ~ payroll
    ('payroll.view'),
    ('payroll.view_own'),
    ('payroll.run'),
    -- ~ guardians
    ('guardians.view'),
    ('guardians.manage'),
    -- ~ communication
    ('communication.view'),
    ('communication.broadcast'),
    -- ~ library
    ('library.view'),
    ('library.manage'),
    -- ~ transport
    ('transport.view'),
    ('transport.manage'),
    -- ~ hostel
    ('hostel.view'),
    ('hostel.manage'),
    -- ~ inventory
    ('inventory.view'),
    ('inventory.manage'),
    -- ~ health
    ('health.view'),
    ('health.manage'),
    -- ~ discipline
    ('discipline.view'),
    ('discipline.manage'),
    -- ~ documents
    ('documents.view'),
    ('documents.manage'),
    -- ~ audit
    ('audit.view'),
    -- ~ settings / admin
    ('settings.view'),
    ('settings.manage');


-- ---------- 2. Role bindings ------------------------------------------
-- Admin: everything. Special "grant-all" join.
INSERT OR IGNORE INTO role_permission (role_id, permission_id)
SELECT r.id, p.id
  FROM role r CROSS JOIN permission p
 WHERE r.name = 'admin';

-- Principal: everything read + most manage, except tenant `settings.manage`.
INSERT OR IGNORE INTO role_permission (role_id, permission_id)
SELECT r.id, p.id
  FROM role r, permission p
 WHERE r.name = 'principal'
   AND p.code IN (
        'students.view','students.edit','students.admit',
        'staff.view','staff.edit','staff.hire',
        'academic.view','academic.manage',
        'attendance.view','attendance.mark',
        'timetable.view','timetable.manage',
        'fees.view','fees.collect',
        'examinations.view','examinations.manage','examinations.enter_marks',
        'payroll.view',
        'guardians.view','guardians.manage',
        'communication.view','communication.broadcast',
        'library.view','transport.view','hostel.view',
        'inventory.view','health.view','discipline.view','discipline.manage',
        'documents.view','documents.manage',
        'audit.view','settings.view');

-- Teacher: read most academic data, mark attendance, enter marks.
INSERT OR IGNORE INTO role_permission (role_id, permission_id)
SELECT r.id, p.id
  FROM role r, permission p
 WHERE r.name = 'teacher'
   AND p.code IN (
        'students.view',
        'academic.view',
        'attendance.view','attendance.mark',
        'timetable.view',
        'examinations.view','examinations.enter_marks',
        'guardians.view',
        'communication.view',
        'library.view',
        'documents.view');

-- Accountant: full fees + payroll + read students/staff.
INSERT OR IGNORE INTO role_permission (role_id, permission_id)
SELECT r.id, p.id
  FROM role r, permission p
 WHERE r.name = 'accountant'
   AND p.code IN (
        'students.view','staff.view',
        'fees.view','fees.collect',
        'payroll.view','payroll.run',
        'documents.view','audit.view');

-- Librarian: library only + read students.
INSERT OR IGNORE INTO role_permission (role_id, permission_id)
SELECT r.id, p.id
  FROM role r, permission p
 WHERE r.name = 'librarian'
   AND p.code IN (
        'students.view',
        'library.view','library.manage');

-- Student: self-service portal — own attendance / fees / marks + academic read.
INSERT OR IGNORE INTO role_permission (role_id, permission_id)
SELECT r.id, p.id
  FROM role r, permission p
 WHERE r.name = 'student'
   AND p.code IN (
        'students.view_own',
        'attendance.view_own',
        'timetable.view',
        'fees.view_own','fees.pay',
        'examinations.view_own',
        'academic.view',
        'library.view',
        'communication.view',
        'documents.view');

-- Guardian (parent): only their child(ren)'s read views + fees.pay.
INSERT OR IGNORE INTO role_permission (role_id, permission_id)
SELECT r.id, p.id
  FROM role r, permission p
 WHERE r.name = 'guardian'
   AND p.code IN (
        'students.view_own',
        'attendance.view_own',
        'fees.view_own','fees.pay',
        'examinations.view_own',
        'timetable.view',
        'communication.view',
        'documents.view');
