//! Fees and finance: categories, structures, invoices, payments, discounts, ledger.
//!
//! Money is stored and passed as `i64` cents everywhere.

use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

use crate::error::{RepoError, RepoResult};

// =====================================================================
// Categories
// =====================================================================

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct FeeCategory {
    pub id: i64,
    pub name: String,
}

#[derive(Clone)]
pub struct FeeCategoryRepo { pool: SqlitePool }

impl FeeCategoryRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn create(&self, name: &str) -> RepoResult<FeeCategory> {
        let id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO fee_category (name) VALUES (?) RETURNING id",
        ).bind(name).fetch_one(&self.pool).await?;
        Ok(FeeCategory { id, name: name.to_string() })
    }

    pub async fn list(&self) -> RepoResult<Vec<FeeCategory>> {
        Ok(sqlx::query_as::<_, FeeCategory>("SELECT * FROM fee_category ORDER BY name")
            .fetch_all(&self.pool).await?)
    }
}

// =====================================================================
// Fee structures (per grade per year)
// =====================================================================

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct FeeStructure {
    pub id: i64,
    pub academic_year_id: i64,
    pub grade_id: i64,
    pub name: String,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct FeeStructureItem {
    pub id: i64,
    pub fee_structure_id: i64,
    pub fee_category_id: i64,
    pub amount_cents: i64,
    pub frequency: String,   // one_time|monthly|quarterly|termly|annually
    pub due_day: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct NewStructureItem {
    pub fee_category_id: i64,
    pub amount_cents: i64,
    pub frequency: String,
    pub due_day: Option<i64>,
}

#[derive(Clone)]
pub struct FeeStructureRepo { pool: SqlitePool }

impl FeeStructureRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn create(
        &self, academic_year_id: i64, grade_id: i64, name: &str,
    ) -> RepoResult<FeeStructure> {
        let id = sqlx::query_scalar::<_, i64>(
            r#"INSERT INTO fee_structure (academic_year_id, grade_id, name)
               VALUES (?, ?, ?) RETURNING id"#,
        )
        .bind(academic_year_id).bind(grade_id).bind(name)
        .fetch_one(&self.pool).await?;
        self.get(id).await
    }

    pub async fn get(&self, id: i64) -> RepoResult<FeeStructure> {
        sqlx::query_as::<_, FeeStructure>("SELECT * FROM fee_structure WHERE id = ?")
            .bind(id).fetch_optional(&self.pool).await?
            .ok_or(RepoError::NotFound)
    }

    pub async fn list_for_year(&self, year_id: i64) -> RepoResult<Vec<FeeStructure>> {
        Ok(sqlx::query_as::<_, FeeStructure>(
            "SELECT * FROM fee_structure WHERE academic_year_id = ? ORDER BY grade_id, name",
        ).bind(year_id).fetch_all(&self.pool).await?)
    }

    pub async fn add_item(&self, fee_structure_id: i64, item: &NewStructureItem)
        -> RepoResult<FeeStructureItem>
    {
        let id = sqlx::query_scalar::<_, i64>(
            r#"INSERT INTO fee_structure_item
                 (fee_structure_id, fee_category_id, amount_cents, frequency, due_day)
               VALUES (?, ?, ?, ?, ?) RETURNING id"#,
        )
        .bind(fee_structure_id).bind(item.fee_category_id)
        .bind(item.amount_cents).bind(&item.frequency).bind(item.due_day)
        .fetch_one(&self.pool).await?;

        sqlx::query_as::<_, FeeStructureItem>("SELECT * FROM fee_structure_item WHERE id = ?")
            .bind(id).fetch_one(&self.pool).await.map_err(Into::into)
    }

    pub async fn items(&self, fee_structure_id: i64) -> RepoResult<Vec<FeeStructureItem>> {
        Ok(sqlx::query_as::<_, FeeStructureItem>(
            "SELECT * FROM fee_structure_item WHERE fee_structure_id = ?",
        ).bind(fee_structure_id).fetch_all(&self.pool).await?)
    }
}

// =====================================================================
// Invoices + lines
// =====================================================================

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct FeeInvoice {
    pub id: i64,
    pub invoice_no: String,
    pub student_id: i64,
    pub academic_year_id: i64,
    pub issue_date: NaiveDate,
    pub due_date: NaiveDate,
    pub subtotal_cents: i64,
    pub discount_cents: i64,
    pub tax_cents: i64,
    pub total_cents: i64,
    pub paid_cents: i64,
    pub status: String,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct FeeInvoiceLine {
    pub id: i64,
    pub invoice_id: i64,
    pub fee_category_id: i64,
    pub description: String,
    pub amount_cents: i64,
    pub discount_cents: i64,
}

#[derive(Debug, Clone)]
pub struct NewInvoice {
    pub invoice_no: String,
    pub student_id: i64,
    pub academic_year_id: i64,
    pub issue_date: NaiveDate,
    pub due_date: NaiveDate,
    pub tax_cents: i64,
    pub notes: Option<String>,
    pub lines: Vec<NewInvoiceLine>,
}

#[derive(Debug, Clone)]
pub struct NewInvoiceLine {
    pub fee_category_id: i64,
    pub description: String,
    pub amount_cents: i64,
    pub discount_cents: i64,
}

#[derive(Clone)]
pub struct InvoiceRepo { pool: SqlitePool }

impl InvoiceRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    /// Create invoice + lines and compute the totals atomically.
    pub async fn create(&self, inv: &NewInvoice) -> RepoResult<FeeInvoice> {
        if inv.lines.is_empty() {
            return Err(RepoError::validation("invoice must have at least one line"));
        }

        let subtotal: i64 = inv.lines.iter().map(|l| l.amount_cents).sum();
        let discount: i64 = inv.lines.iter().map(|l| l.discount_cents).sum();
        let total    = subtotal - discount + inv.tax_cents;
        if total < 0 { return Err(RepoError::validation("invoice total cannot be negative")); }

        let mut tx = self.pool.begin().await?;

        let id = sqlx::query_scalar::<_, i64>(
            r#"INSERT INTO fee_invoice
                 (invoice_no, student_id, academic_year_id, issue_date, due_date,
                  subtotal_cents, discount_cents, tax_cents, total_cents, paid_cents,
                  status, notes)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 0, 'unpaid', ?) RETURNING id"#,
        )
        .bind(&inv.invoice_no).bind(inv.student_id).bind(inv.academic_year_id)
        .bind(inv.issue_date).bind(inv.due_date)
        .bind(subtotal).bind(discount).bind(inv.tax_cents).bind(total)
        .bind(&inv.notes)
        .fetch_one(&mut *tx).await?;

        for l in &inv.lines {
            sqlx::query(
                r#"INSERT INTO fee_invoice_line
                     (invoice_id, fee_category_id, description, amount_cents, discount_cents)
                   VALUES (?, ?, ?, ?, ?)"#,
            )
            .bind(id).bind(l.fee_category_id).bind(&l.description)
            .bind(l.amount_cents).bind(l.discount_cents)
            .execute(&mut *tx).await?;
        }

        tx.commit().await?;
        self.get(id).await
    }

    pub async fn get(&self, id: i64) -> RepoResult<FeeInvoice> {
        sqlx::query_as::<_, FeeInvoice>("SELECT * FROM fee_invoice WHERE id = ?")
            .bind(id).fetch_optional(&self.pool).await?
            .ok_or(RepoError::NotFound)
    }

    pub async fn find_by_no(&self, no: &str) -> RepoResult<Option<FeeInvoice>> {
        Ok(sqlx::query_as::<_, FeeInvoice>("SELECT * FROM fee_invoice WHERE invoice_no = ?")
            .bind(no).fetch_optional(&self.pool).await?)
    }

    pub async fn lines(&self, invoice_id: i64) -> RepoResult<Vec<FeeInvoiceLine>> {
        Ok(sqlx::query_as::<_, FeeInvoiceLine>(
            "SELECT * FROM fee_invoice_line WHERE invoice_id = ? ORDER BY id",
        ).bind(invoice_id).fetch_all(&self.pool).await?)
    }

    pub async fn for_student(&self, student_id: i64) -> RepoResult<Vec<FeeInvoice>> {
        Ok(sqlx::query_as::<_, FeeInvoice>(
            "SELECT * FROM fee_invoice WHERE student_id = ? ORDER BY issue_date DESC",
        ).bind(student_id).fetch_all(&self.pool).await?)
    }

    pub async fn outstanding(&self, student_id: i64) -> RepoResult<i64> {
        Ok(sqlx::query_scalar::<_, i64>(
            r#"SELECT COALESCE(SUM(total_cents - paid_cents), 0)
                 FROM fee_invoice
                WHERE student_id = ? AND status IN ('unpaid','partial','overdue')"#,
        ).bind(student_id).fetch_one(&self.pool).await?)
    }

    pub async fn overdue(&self, today: NaiveDate) -> RepoResult<Vec<FeeInvoice>> {
        Ok(sqlx::query_as::<_, FeeInvoice>(
            r#"SELECT * FROM fee_invoice
               WHERE due_date < ? AND status IN ('unpaid','partial')
               ORDER BY due_date"#,
        ).bind(today).fetch_all(&self.pool).await?)
    }

    pub async fn cancel(&self, id: i64) -> RepoResult<()> {
        sqlx::query("UPDATE fee_invoice SET status = 'cancelled' WHERE id = ?")
            .bind(id).execute(&self.pool).await?;
        Ok(())
    }
}

// =====================================================================
// Payments
// =====================================================================

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct FeePayment {
    pub id: i64,
    pub receipt_no: String,
    pub invoice_id: i64,
    pub paid_on: NaiveDate,
    pub amount_cents: i64,
    pub method: String,
    pub reference: Option<String>,
    pub received_by_staff_id: Option<i64>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct NewPayment {
    pub receipt_no: String,
    pub invoice_id: i64,
    pub paid_on: NaiveDate,
    pub amount_cents: i64,
    pub method: String,
    pub reference: Option<String>,
    pub received_by_staff_id: Option<i64>,
}

#[derive(Clone)]
pub struct PaymentRepo { pool: SqlitePool }

impl PaymentRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    /// Record a payment and update invoice status/paid_cents atomically.
    pub async fn record(&self, p: &NewPayment) -> RepoResult<FeePayment> {
        if p.amount_cents <= 0 {
            return Err(RepoError::validation("amount_cents must be > 0"));
        }

        let mut tx = self.pool.begin().await?;

        // Lock-like read (SQLite is single-writer via WAL; the transaction is enough).
        let (total, paid): (i64, i64) = sqlx::query_as(
            "SELECT total_cents, paid_cents FROM fee_invoice WHERE id = ?",
        ).bind(p.invoice_id).fetch_optional(&mut *tx).await?
         .ok_or(RepoError::NotFound)?;

        let new_paid = paid.checked_add(p.amount_cents)
            .ok_or_else(|| RepoError::validation("paid amount overflow"))?;
        if new_paid > total {
            return Err(RepoError::validation("payment exceeds invoice balance"));
        }

        let id = sqlx::query_scalar::<_, i64>(
            r#"INSERT INTO fee_payment
                 (receipt_no, invoice_id, paid_on, amount_cents, method, reference, received_by_staff_id)
               VALUES (?, ?, ?, ?, ?, ?, ?) RETURNING id"#,
        )
        .bind(&p.receipt_no).bind(p.invoice_id).bind(p.paid_on)
        .bind(p.amount_cents).bind(&p.method).bind(&p.reference)
        .bind(p.received_by_staff_id)
        .fetch_one(&mut *tx).await?;

        let new_status = if new_paid == total { "paid" } else { "partial" };
        sqlx::query(
            "UPDATE fee_invoice SET paid_cents = ?, status = ? WHERE id = ?",
        )
        .bind(new_paid).bind(new_status).bind(p.invoice_id)
        .execute(&mut *tx).await?;

        tx.commit().await?;

        sqlx::query_as::<_, FeePayment>("SELECT * FROM fee_payment WHERE id = ?")
            .bind(id).fetch_one(&self.pool).await.map_err(Into::into)
    }

    pub async fn for_invoice(&self, invoice_id: i64) -> RepoResult<Vec<FeePayment>> {
        Ok(sqlx::query_as::<_, FeePayment>(
            "SELECT * FROM fee_payment WHERE invoice_id = ? ORDER BY paid_on",
        ).bind(invoice_id).fetch_all(&self.pool).await?)
    }

    pub async fn totals_between(&self, from: NaiveDate, to: NaiveDate) -> RepoResult<i64> {
        Ok(sqlx::query_scalar::<_, i64>(
            "SELECT COALESCE(SUM(amount_cents), 0) FROM fee_payment WHERE paid_on BETWEEN ? AND ?",
        ).bind(from).bind(to).fetch_one(&self.pool).await?)
    }
}

// =====================================================================
// Discounts
// =====================================================================

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct FeeDiscount {
    pub id: i64,
    pub student_id: i64,
    pub name: String,
    pub percent: Option<f64>,
    pub flat_cents: Option<i64>,
    pub valid_from: Option<NaiveDate>,
    pub valid_to: Option<NaiveDate>,
}

#[derive(Clone)]
pub struct DiscountRepo { pool: SqlitePool }

impl DiscountRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn grant(&self, d: &FeeDiscount) -> RepoResult<i64> {
        if d.percent.is_none() && d.flat_cents.is_none() {
            return Err(RepoError::validation("either percent or flat_cents is required"));
        }
        Ok(sqlx::query_scalar::<_, i64>(
            r#"INSERT INTO fee_discount
                 (student_id, name, percent, flat_cents, valid_from, valid_to)
               VALUES (?, ?, ?, ?, ?, ?) RETURNING id"#,
        )
        .bind(d.student_id).bind(&d.name).bind(d.percent).bind(d.flat_cents)
        .bind(d.valid_from).bind(d.valid_to)
        .fetch_one(&self.pool).await?)
    }

    pub async fn for_student(&self, student_id: i64) -> RepoResult<Vec<FeeDiscount>> {
        Ok(sqlx::query_as::<_, FeeDiscount>(
            "SELECT * FROM fee_discount WHERE student_id = ?",
        ).bind(student_id).fetch_all(&self.pool).await?)
    }
}

// =====================================================================
// Ledger (double-entry-ish)
// =====================================================================

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct LedgerAccount {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub r#type: String,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct LedgerEntry {
    pub id: i64,
    pub entry_date: NaiveDate,
    pub account_id: i64,
    pub debit_cents: i64,
    pub credit_cents: i64,
    pub ref_type: Option<String>,
    pub ref_id: Option<i64>,
    pub memo: Option<String>,
}

#[derive(Debug, Clone)]
pub struct JournalLine {
    pub account_id: i64,
    pub debit_cents: i64,
    pub credit_cents: i64,
    pub memo: Option<String>,
}

#[derive(Clone)]
pub struct LedgerRepo { pool: SqlitePool }

impl LedgerRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn accounts(&self) -> RepoResult<Vec<LedgerAccount>> {
        Ok(sqlx::query_as::<_, LedgerAccount>("SELECT * FROM ledger_account ORDER BY code")
            .fetch_all(&self.pool).await?)
    }

    /// Post a balanced journal entry (sum(debits) must equal sum(credits)).
    pub async fn post_journal(
        &self,
        entry_date: NaiveDate,
        ref_type: Option<&str>,
        ref_id: Option<i64>,
        lines: &[JournalLine],
    ) -> RepoResult<()> {
        if lines.is_empty() {
            return Err(RepoError::validation("journal must have lines"));
        }
        let (mut d, mut c) = (0_i64, 0_i64);
        for l in lines {
            if l.debit_cents < 0 || l.credit_cents < 0 {
                return Err(RepoError::validation("amounts must be >= 0"));
            }
            if l.debit_cents > 0 && l.credit_cents > 0 {
                return Err(RepoError::validation("a line cannot be both debit and credit"));
            }
            d = d.checked_add(l.debit_cents).ok_or_else(|| RepoError::validation("overflow"))?;
            c = c.checked_add(l.credit_cents).ok_or_else(|| RepoError::validation("overflow"))?;
        }
        if d != c {
            return Err(RepoError::validation("debits must equal credits"));
        }

        let mut tx = self.pool.begin().await?;
        for l in lines {
            sqlx::query(
                r#"INSERT INTO ledger_entry
                     (entry_date, account_id, debit_cents, credit_cents, ref_type, ref_id, memo)
                   VALUES (?, ?, ?, ?, ?, ?, ?)"#,
            )
            .bind(entry_date).bind(l.account_id)
            .bind(l.debit_cents).bind(l.credit_cents)
            .bind(ref_type).bind(ref_id).bind(&l.memo)
            .execute(&mut *tx).await?;
        }
        tx.commit().await?;
        Ok(())
    }

    /// Trial balance snapshot.
    pub async fn trial_balance(&self, as_of: NaiveDate) -> RepoResult<Vec<(String, i64, i64)>> {
        Ok(sqlx::query_as::<_, (String, i64, i64)>(
            r#"SELECT a.code,
                      COALESCE(SUM(e.debit_cents), 0)  AS debit_cents,
                      COALESCE(SUM(e.credit_cents), 0) AS credit_cents
                 FROM ledger_account a
                 LEFT JOIN ledger_entry e
                   ON e.account_id = a.id AND e.entry_date <= ?
                GROUP BY a.id
                ORDER BY a.code"#,
        ).bind(as_of).fetch_all(&self.pool).await?)
    }

    pub async fn entries_for(&self, account_id: i64, from: NaiveDate, to: NaiveDate)
        -> RepoResult<Vec<LedgerEntry>>
    {
        Ok(sqlx::query_as::<_, LedgerEntry>(
            r#"SELECT * FROM ledger_entry
               WHERE account_id = ? AND entry_date BETWEEN ? AND ?
               ORDER BY entry_date, id"#,
        ).bind(account_id).bind(from).bind(to).fetch_all(&self.pool).await?)
    }
}
