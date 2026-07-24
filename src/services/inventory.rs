use std::sync::Arc;

use chrono::NaiveDate;

use crate::repositories::Repositories;
use crate::repositories::fees::JournalLine;
use crate::services::{ledger_codes, ServiceError, ServiceResult};

#[derive(Clone)]
pub struct InventoryService {
    repos: Arc<Repositories>,
}

impl InventoryService {
    pub fn new(repos: Arc<Repositories>) -> Self { Self { repos } }

pub async fn set_po_status(&self, id: i64, target: &str) -> ServiceResult<()> {
        let po = self.repos.purchase_orders.get(id).await?;
        let ok = matches!(
            (po.status.as_str(), target),
            ("draft", "ordered") | ("draft", "cancelled") |
            ("ordered", "received") | ("ordered", "cancelled")
        );
        if !ok {
            return Err(ServiceError::conflict(format!(
                "invalid PO transition {} -> {}", po.status, target
            )));
        }
        if target == "received" {
            return self.receive_po(id, po.order_date).await;
        }
        self.repos.purchase_orders.set_status(id, target).await?;
        Ok(())
    }

pub async fn receive_po(&self, id: i64, on: NaiveDate) -> ServiceResult<()> {
        let po = self.repos.purchase_orders.get(id).await?;
        if po.status == "received" {
            return Err(ServiceError::conflict("already received"));
        }

self.repos.purchase_orders.receive(id).await?;

        let accounts = self.repos.ledger.accounts().await?;
        let expense = accounts.iter().find(|a| a.code == ledger_codes::SUPPLIES_EXPENSE);
        let payable = accounts.iter().find(|a| a.code == ledger_codes::PAYABLE);
        if let (Some(exp), Some(ap)) = (expense, payable) {
            let lines = vec![
                JournalLine {
                    account_id: exp.id, debit_cents: po.total_cents, credit_cents: 0,
                    memo: Some(format!("PO {} receipt", po.po_no)),
                },
                JournalLine {
                    account_id: ap.id, debit_cents: 0, credit_cents: po.total_cents,
                    memo: Some(format!("Vendor {}", po.vendor_id)),
                },
            ];
            self.repos.ledger.post_journal(on, Some("purchase_order"), Some(po.id), &lines).await?;
        }
        Ok(())
    }

pub async fn low_stock_alert_ids(&self) -> ServiceResult<Vec<i64>> {
        Ok(self.repos.items.below_reorder().await?
            .into_iter().map(|i| i.id).collect())
    }
}
