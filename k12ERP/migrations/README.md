# Migrations

`sqlx migrate run` will execute these in numeric order. They are intentionally
split by module to keep diffs reviewable and to make it easy to add new
migrations later without touching a single monolithic file.

## Order

| # | File | Contents |
|---|---|---|
| 001 | `..._core.sql` | `school`, `academic_year`, `term` |
| 002 | `..._academic_structure.sql` | `grade`, `section`, `room`, `subject` |
| 003 | `..._auth_and_people.sql` | `user_account`, `role`, `permission`, `staff`, `student`, `guardian`, `student_guardian` |
| 004 | `..._class_section_and_enrollment.sql` | `class_section`, `class_subject`, `enrollment` |
| 005 | `..._attendance_and_leave.sql` | `student_attendance`, `staff_attendance`, `*_leave_request` |
| 006 | `..._timetable.sql` | `period`, `timetable_slot` |
| 007 | `..._examinations.sql` | `grading_scale`, `grade_band`, `exam`, `exam_schedule`, `exam_result` |
| 008 | `..._fees_and_finance.sql` | `fee_category`, `fee_structure*`, `fee_invoice*`, `fee_payment`, `fee_discount`, `ledger_*` |
| 009 | `..._payroll.sql` | `salary_component`, `salary_structure*`, `payslip*` |
| 010 | `..._library.sql` | `book`, `book_issue` |
| 011 | `..._transport.sql` | `vehicle`, `route`, `route_stop`, `student_transport` |
| 012 | `..._hostel.sql` | `hostel`, `hostel_room`, `hostel_allocation` |
| 013 | `..._inventory.sql` | `vendor`, `item`, `stock_movement`, `purchase_order*` |
| 014 | `..._communication.sql` | `announcement`, `message`, `notification` |
| 015 | `..._health_and_discipline.sql` | `health_record`, `vaccination`, `clinic_visit`, `discipline_incident` |
| 016 | `..._documents_and_audit.sql` | `document`, `audit_log` |
| 017 | `..._seed_reference_data.sql` | Idempotent seed data (grades, sections, roles, fee categories, salary components, chart of accounts) |

## Rust / sqlx setup

Enable foreign keys **on every connection** (SQLite defaults them off):

```rust
use sqlx::sqlite::{SqlitePoolOptions, SqliteConnectOptions};
use std::str::FromStr;

let opts = SqliteConnectOptions::from_str("sqlite://data/erp.db?mode=rwc")?
    .create_if_missing(true)
    .foreign_keys(true)
    .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
    .synchronous(sqlx::sqlite::SqliteSynchronous::Normal)
    .busy_timeout(std::time::Duration::from_secs(5));

let pool = SqlitePoolOptions::new()
    .max_connections(8)
    .connect_with(opts)
    .await?;

sqlx::migrate!("./migrations").run(&pool).await?;
```

## Conventions

* **Money** is stored as `INTEGER` cents (`*_cents`). Never use `REAL` for money.
* **Dates/times** are ISO-8601 `TEXT` (`YYYY-MM-DD` and `YYYY-MM-DD HH:MM:SS`), which map cleanly to `chrono::NaiveDate` / `NaiveDateTime`.
* **Booleans** are `INTEGER 0/1` with a `CHECK` constraint.
* **Enums** are `TEXT` with `CHECK (col IN (...))`.
* **Polymorphic FKs** (`document.owner_type/owner_id`, `audit_log.entity/entity_id`, `stock_movement.ref_type/ref_id`, `ledger_entry.ref_type/ref_id`) cannot be enforced by SQLite — enforce in the Rust repository layer.
* All FKs default to `ON DELETE RESTRICT`; `CASCADE` is used only for tightly-owned child rows.
