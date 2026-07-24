use std::sync::Arc;

use chrono::NaiveDate;

use crate::repositories::Repositories;
use crate::repositories::fees::JournalLine;
use crate::repositories::payroll::{NewStructureItem, Payslip};
use crate::services::{ledger_codes, RequestCtx, ServiceError, ServiceResult};
use crate::services::perm;

#[derive(Clone)]
pub struct PayrollService {
    repos: Arc<Repositories>,
}

impl PayrollService {
    pub fn new(repos: Arc<Repositories>) -> Self { Self { repos } }

pub async fn set_salary(
        &self, ctx: &RequestCtx, staff_id: i64, effective_from: NaiveDate,
        items: Vec<NewStructureItem>,
    ) -> ServiceResult<i64> {
        ctx.require(perm::PAYROLL_RUN)?;
        if items.is_empty() {
            return Err(ServiceError::validation("salary structure needs items"));
        }
        let s = self.repos.salary_structures.create(staff_id, effective_from, &items).await?;
        Ok(s.id)
    }

pub async fn generate_payslip(
        &self, ctx: &RequestCtx, staff_id: i64, month: i64, year: i64,
    ) -> ServiceResult<Payslip> {
        ctx.require(perm::PAYROLL_RUN)?;
        Ok(self.repos.payslips.generate(staff_id, month, year).await?)
    }

    pub async fn approve(&self, ctx: &RequestCtx, payslip_id: i64) -> ServiceResult<()> {
        ctx.require(perm::PAYROLL_RUN)?;
        let p = self.repos.payslips.get(payslip_id).await?;
        if p.status != "draft" {
            return Err(ServiceError::conflict("only draft payslips can be approved"));
        }
        self.repos.payslips.approve(payslip_id).await?;
        Ok(())
    }

pub async fn pay(
        &self, ctx: &RequestCtx, payslip_id: i64, paid_on: NaiveDate, from_bank: bool,
    ) -> ServiceResult<()> {
        ctx.require(perm::PAYROLL_RUN)?;
        let p = self.repos.payslips.get(payslip_id).await?;
        if p.status != "approved" {
            return Err(ServiceError::conflict("payslip must be approved before paying"));
        }
        self.repos.payslips.mark_paid(payslip_id, paid_on).await?;

        let accounts = self.repos.ledger.accounts().await?;
        let expense  = accounts.iter().find(|a| a.code == ledger_codes::SALARY_EXPENSE);
        let cash_or_bank = accounts.iter()
            .find(|a| a.code == if from_bank { ledger_codes::BANK } else { ledger_codes::CASH });

        if let (Some(exp), Some(cr)) = (expense, cash_or_bank) {
            let lines = vec![
                JournalLine {
                    account_id: exp.id, debit_cents: p.net_cents, credit_cents: 0,
                    memo: Some(format!("Payslip {} for staff {}", p.id, p.staff_id)),
                },
                JournalLine {
                    account_id: cr.id, debit_cents: 0, credit_cents: p.net_cents,
                    memo: Some(format!("Payslip {}", p.id)),
                },
            ];
            self.repos.ledger.post_journal(paid_on, Some("payslip"), Some(p.id), &lines).await?;
        }
        Ok(())
    }

pub async fn generate_month(&self, ctx: &RequestCtx, month: i64, year: i64) -> ServiceResult<u64> {
        ctx.require(perm::PAYROLL_RUN)?;
        let staff = self.repos.staff.list_active().await?;
        let mut count = 0_u64;
        for s in staff {

            if self.repos.salary_structures.current_for_staff(s.id).await?.is_some() {
                match self.repos.payslips.generate(s.id, month, year).await {
                    Ok(_) => count += 1,
                    Err(crate::error::RepoError::Sqlx(sqlx::Error::Database(db)))
                        if db.message().contains("UNIQUE") => {

                    }
                    Err(e) => return Err(e.into()),
                }
            }
        }
        Ok(count)
    }
}
