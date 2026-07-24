use std::sync::Arc;

use chrono::NaiveDate;

use crate::repositories::Repositories;
use crate::repositories::fees::{
    FeeInvoice, FeePayment, JournalLine, NewInvoice, NewInvoiceLine, NewPayment,
};
use crate::services::{ledger_codes, ServiceError, ServiceResult};

#[derive(Clone)]
pub struct FeeService {
    repos: Arc<Repositories>,
}

impl FeeService {
    pub fn new(repos: Arc<Repositories>) -> Self { Self { repos } }

pub async fn generate_invoice_for_student(
        &self,
        student_id: i64,
        fee_structure_id: i64,
        invoice_no: String,
        issue_date: NaiveDate,
        due_date: NaiveDate,
        tax_cents: i64,
    ) -> ServiceResult<FeeInvoice> {
        let structure = self.repos.fee_structures.get(fee_structure_id).await?;
        let items = self.repos.fee_structures.items(fee_structure_id).await?;
        if items.is_empty() {
            return Err(ServiceError::validation("fee structure has no items"));
        }

let discounts = self.repos.discounts.for_student(student_id).await?;
        let today = issue_date;
        let mut percent_off = 0.0_f64;
        let mut flat_off_cents = 0_i64;
        for d in discounts {
            let ok_from = d.valid_from.map(|f| f <= today).unwrap_or(true);
            let ok_to   = d.valid_to.map(|t| today <= t).unwrap_or(true);
            if ok_from && ok_to {
                if let Some(p) = d.percent    { percent_off += p; }
                if let Some(f) = d.flat_cents { flat_off_cents += f; }
            }
        }
        if percent_off > 100.0 { percent_off = 100.0; }

let lines: Vec<NewInvoiceLine> = items.into_iter().map(|it| {
            let pct_cents = ((it.amount_cents as f64) * percent_off / 100.0).round() as i64;
            NewInvoiceLine {
                fee_category_id: it.fee_category_id,
                description: format!("Fee item ({})", it.frequency),
                amount_cents: it.amount_cents,
                discount_cents: pct_cents,
            }
        }).collect();

        let mut invoice = NewInvoice {
            invoice_no,
            student_id,
            academic_year_id: structure.academic_year_id,
            issue_date,
            due_date,
            tax_cents,
            notes: None,
            lines,
        };

if flat_off_cents > 0 {
            if let Some(l) = invoice.lines.first_mut() {
                l.discount_cents = l.discount_cents.saturating_add(flat_off_cents);
            }
        }

        Ok(self.repos.invoices.create(&invoice).await?)
    }

pub async fn record_payment(&self, p: NewPayment) -> ServiceResult<FeePayment> {
        let payment = self.repos.payments.record(&p).await?;

let accounts = self.repos.ledger.accounts().await?;
        let cash = accounts.iter().find(|a| a.code == ledger_codes::CASH);
        let bank = accounts.iter().find(|a| a.code == ledger_codes::BANK);
        let recv = accounts.iter().find(|a| a.code == ledger_codes::RECEIVABLE);

        let debit_account = match payment.method.as_str() {
            "cash" => cash,
            _      => bank.or(cash),
        };

        if let (Some(dr), Some(cr)) = (debit_account, recv) {
            let lines = vec![
                JournalLine {
                    account_id: dr.id, debit_cents: payment.amount_cents, credit_cents: 0,
                    memo: Some(format!("Payment receipt {}", payment.receipt_no)),
                },
                JournalLine {
                    account_id: cr.id, debit_cents: 0, credit_cents: payment.amount_cents,
                    memo: Some(format!("Invoice {}", payment.invoice_id)),
                },
            ];
            self.repos.ledger.post_journal(
                payment.paid_on, Some("payment"), Some(payment.id), &lines,
            ).await?;
        }
        Ok(payment)
    }

pub async fn outstanding(&self, student_id: i64) -> ServiceResult<i64> {
        Ok(self.repos.invoices.outstanding(student_id).await?)
    }

pub async fn aging(&self, today: NaiveDate) -> ServiceResult<(i64,i64,i64,i64)> {
        let overdue = self.repos.invoices.overdue(today).await?;
        let (mut b0, mut b30, mut b60, mut b90) = (0, 0, 0, 0);
        for inv in overdue {
            let days = (today - inv.due_date).num_days();
            let bal  = inv.total_cents - inv.paid_cents;
            match days {
                d if d <= 30 => b0  += bal,
                d if d <= 60 => b30 += bal,
                d if d <= 90 => b60 += bal,
                _            => b90 += bal,
            }
        }
        Ok((b0, b30, b60, b90))
    }

    pub async fn cancel_invoice(&self, id: i64) -> ServiceResult<()> {
        let inv = self.repos.invoices.get(id).await?;
        if inv.paid_cents > 0 {
            return Err(ServiceError::conflict("cannot cancel an invoice with payments"));
        }
        self.repos.invoices.cancel(id).await?;
        Ok(())
    }
}
