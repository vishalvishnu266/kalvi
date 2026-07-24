use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

use crate::error::{RepoError, RepoResult};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct SalaryComponent {
    pub id: i64,
    pub name: String,
    pub kind: String,
}

#[derive(Clone)]
pub struct SalaryComponentRepo { pool: SqlitePool }

impl SalaryComponentRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn create(&self, name: &str, kind: &str) -> RepoResult<SalaryComponent> {
        if !matches!(kind, "earning" | "deduction") {
            return Err(RepoError::validation("kind must be 'earning' or 'deduction'"));
        }
        let id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO salary_component (name, kind) VALUES (?, ?) RETURNING id",
        ).bind(name).bind(kind).fetch_one(&self.pool).await?;
        Ok(SalaryComponent { id, name: name.into(), kind: kind.into() })
    }

    pub async fn list(&self) -> RepoResult<Vec<SalaryComponent>> {
        Ok(sqlx::query_as::<_, SalaryComponent>(
            "SELECT * FROM salary_component ORDER BY kind, name",
        ).fetch_all(&self.pool).await?)
    }
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct SalaryStructure {
    pub id: i64,
    pub staff_id: i64,
    pub effective_from: NaiveDate,
    pub effective_to: Option<NaiveDate>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct SalaryStructureItem {
    pub id: i64,
    pub salary_structure_id: i64,
    pub component_id: i64,
    pub amount_cents: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewStructureItem {
    pub component_id: i64,
    pub amount_cents: i64,
}

#[derive(Clone)]
pub struct SalaryStructureRepo { pool: SqlitePool }

impl SalaryStructureRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn create(
        &self, staff_id: i64, effective_from: NaiveDate,
        items: &[NewStructureItem],
    ) -> RepoResult<SalaryStructure> {
        let mut tx = self.pool.begin().await?;

sqlx::query(
            r#"UPDATE salary_structure
                 SET effective_to = date(?, '-1 day')
               WHERE staff_id = ? AND effective_to IS NULL"#,
        ).bind(effective_from).bind(staff_id).execute(&mut *tx).await?;

        let id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO salary_structure (staff_id, effective_from) VALUES (?, ?) RETURNING id",
        ).bind(staff_id).bind(effective_from).fetch_one(&mut *tx).await?;

        for it in items {
            sqlx::query(
                r#"INSERT INTO salary_structure_item
                     (salary_structure_id, component_id, amount_cents)
                   VALUES (?, ?, ?)"#,
            )
            .bind(id).bind(it.component_id).bind(it.amount_cents)
            .execute(&mut *tx).await?;
        }

        tx.commit().await?;
        Ok(sqlx::query_as::<_, SalaryStructure>("SELECT * FROM salary_structure WHERE id = ?")
            .bind(id).fetch_one(&self.pool).await?)
    }

    pub async fn current_for_staff(&self, staff_id: i64) -> RepoResult<Option<SalaryStructure>> {
        Ok(sqlx::query_as::<_, SalaryStructure>(
            r#"SELECT * FROM salary_structure
               WHERE staff_id = ? AND effective_to IS NULL
               ORDER BY effective_from DESC LIMIT 1"#,
        ).bind(staff_id).fetch_optional(&self.pool).await?)
    }

    pub async fn items(&self, structure_id: i64) -> RepoResult<Vec<SalaryStructureItem>> {
        Ok(sqlx::query_as::<_, SalaryStructureItem>(
            "SELECT * FROM salary_structure_item WHERE salary_structure_id = ?",
        ).bind(structure_id).fetch_all(&self.pool).await?)
    }
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Payslip {
    pub id: i64,
    pub staff_id: i64,
    pub period_month: i64,
    pub period_year: i64,
    pub gross_cents: i64,
    pub deduction_cents: i64,
    pub net_cents: i64,
    pub paid_on: Option<NaiveDate>,
    pub status: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct PayslipLine {
    pub component_id: i64,
    pub amount_cents: i64,
    pub kind: String,
}

#[derive(Clone)]
pub struct PayslipRepo { pool: SqlitePool }

impl PayslipRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

pub async fn generate(
        &self, staff_id: i64, month: i64, year: i64,
    ) -> RepoResult<Payslip> {
        if !(1..=12).contains(&month) {
            return Err(RepoError::validation("month must be 1..12"));
        }
        let mut tx = self.pool.begin().await?;

let rows: Vec<(i64, i64, String)> = sqlx::query_as(
            r#"SELECT ssi.component_id, ssi.amount_cents, sc.kind
                 FROM salary_structure ss
                 JOIN salary_structure_item ssi ON ssi.salary_structure_id = ss.id
                 JOIN salary_component sc       ON sc.id = ssi.component_id
                WHERE ss.staff_id = ? AND ss.effective_to IS NULL
                ORDER BY ssi.component_id"#,
        ).bind(staff_id).fetch_all(&mut *tx).await?;

        if rows.is_empty() {
            return Err(RepoError::validation("no active salary structure for staff"));
        }

        let mut gross = 0_i64;
        let mut deductions = 0_i64;
        for (_cid, amt, kind) in &rows {
            match kind.as_str() {
                "earning"   => gross += amt,
                "deduction" => deductions += amt,
                _ => {}
            }
        }
        let net = gross - deductions;

        let id = sqlx::query_scalar::<_, i64>(
            r#"INSERT INTO payslip
                 (staff_id, period_month, period_year, gross_cents, deduction_cents, net_cents, status)
               VALUES (?, ?, ?, ?, ?, ?, 'draft') RETURNING id"#,
        )
        .bind(staff_id).bind(month).bind(year).bind(gross).bind(deductions).bind(net)
        .fetch_one(&mut *tx).await?;

        for (cid, amt, _kind) in &rows {
            sqlx::query(
                "INSERT INTO payslip_line (payslip_id, component_id, amount_cents) VALUES (?, ?, ?)",
            )
            .bind(id).bind(cid).bind(amt)
            .execute(&mut *tx).await?;
        }

        tx.commit().await?;
        self.get(id).await
    }

    pub async fn get(&self, id: i64) -> RepoResult<Payslip> {
        sqlx::query_as::<_, Payslip>("SELECT * FROM payslip WHERE id = ?")
            .bind(id).fetch_optional(&self.pool).await?
            .ok_or(RepoError::NotFound)
    }

    pub async fn approve(&self, id: i64) -> RepoResult<()> {
        sqlx::query("UPDATE payslip SET status = 'approved' WHERE id = ? AND status = 'draft'")
            .bind(id).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn mark_paid(&self, id: i64, paid_on: NaiveDate) -> RepoResult<()> {
        sqlx::query(
            "UPDATE payslip SET status = 'paid', paid_on = ? WHERE id = ? AND status = 'approved'",
        )
        .bind(paid_on).bind(id).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn for_staff(&self, staff_id: i64, year: i64) -> RepoResult<Vec<Payslip>> {
        Ok(sqlx::query_as::<_, Payslip>(
            r#"SELECT * FROM payslip
               WHERE staff_id = ? AND period_year = ?
               ORDER BY period_month"#,
        ).bind(staff_id).bind(year).fetch_all(&self.pool).await?)
    }
}
