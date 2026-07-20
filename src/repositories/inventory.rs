//! Inventory: vendors, items, stock movements, purchase orders.

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

use crate::error::{RepoError, RepoResult};

// ---------- Vendor ----------

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Vendor {
    pub id: i64,
    pub name: String,
    pub contact: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
}

#[derive(Clone)]
pub struct VendorRepo { pool: SqlitePool }

impl VendorRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn create(&self, v: &Vendor) -> RepoResult<Vendor> {
        let id = sqlx::query_scalar::<_, i64>(
            r#"INSERT INTO vendor (name, contact, phone, email, address)
               VALUES (?, ?, ?, ?, ?) RETURNING id"#,
        )
        .bind(&v.name).bind(&v.contact).bind(&v.phone).bind(&v.email).bind(&v.address)
        .fetch_one(&self.pool).await?;
        self.get(id).await
    }

    pub async fn get(&self, id: i64) -> RepoResult<Vendor> {
        sqlx::query_as::<_, Vendor>("SELECT * FROM vendor WHERE id = ?")
            .bind(id).fetch_optional(&self.pool).await?
            .ok_or(RepoError::NotFound)
    }

    pub async fn list(&self) -> RepoResult<Vec<Vendor>> {
        Ok(sqlx::query_as::<_, Vendor>("SELECT * FROM vendor ORDER BY name")
            .fetch_all(&self.pool).await?)
    }
}

// ---------- Item ----------

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Item {
    pub id: i64,
    pub name: String,
    pub sku: Option<String>,
    pub unit: Option<String>,
    pub stock_qty: i64,
    pub reorder_level: Option<i64>,
    pub unit_cost_cents: Option<i64>,
}

#[derive(Clone)]
pub struct ItemRepo { pool: SqlitePool }

impl ItemRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn create(&self, i: &Item) -> RepoResult<Item> {
        let id = sqlx::query_scalar::<_, i64>(
            r#"INSERT INTO item (name, sku, unit, stock_qty, reorder_level, unit_cost_cents)
               VALUES (?, ?, ?, ?, ?, ?) RETURNING id"#,
        )
        .bind(&i.name).bind(&i.sku).bind(&i.unit)
        .bind(i.stock_qty).bind(i.reorder_level).bind(i.unit_cost_cents)
        .fetch_one(&self.pool).await?;
        self.get(id).await
    }

    pub async fn get(&self, id: i64) -> RepoResult<Item> {
        sqlx::query_as::<_, Item>("SELECT * FROM item WHERE id = ?")
            .bind(id).fetch_optional(&self.pool).await?
            .ok_or(RepoError::NotFound)
    }

    pub async fn list(&self, limit: i64, offset: i64) -> RepoResult<Vec<Item>> {
        Ok(sqlx::query_as::<_, Item>(
            "SELECT * FROM item ORDER BY name LIMIT ? OFFSET ?",
        ).bind(limit).bind(offset).fetch_all(&self.pool).await?)
    }

    pub async fn below_reorder(&self) -> RepoResult<Vec<Item>> {
        Ok(sqlx::query_as::<_, Item>(
            "SELECT * FROM item WHERE reorder_level IS NOT NULL AND stock_qty <= reorder_level",
        ).fetch_all(&self.pool).await?)
    }
}

// ---------- Stock movements ----------

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct StockMovement {
    pub id: i64,
    pub item_id: i64,
    pub movement: String,   // in | out | adjust
    pub quantity: i64,
    pub reason: Option<String>,
    pub ref_type: Option<String>,
    pub ref_id: Option<i64>,
    pub moved_on: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewMovement {
    pub item_id: i64,
    pub movement: String,
    pub quantity: i64,
    pub reason: Option<String>,
    pub ref_type: Option<String>,
    pub ref_id: Option<i64>,
}

#[derive(Clone)]
pub struct StockMovementRepo { pool: SqlitePool }

impl StockMovementRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    /// Record a movement and adjust `item.stock_qty` atomically.
    pub async fn record(&self, m: &NewMovement) -> RepoResult<StockMovement> {
        if m.quantity == 0 {
            return Err(RepoError::validation("quantity cannot be 0"));
        }
        let delta: i64 = match m.movement.as_str() {
            "in" => m.quantity.abs(),
            "out" => -m.quantity.abs(),
            "adjust" => m.quantity,
            _ => return Err(RepoError::validation("movement must be in|out|adjust")),
        };

        let mut tx = self.pool.begin().await?;

        let current: i64 = sqlx::query_scalar("SELECT stock_qty FROM item WHERE id = ?")
            .bind(m.item_id).fetch_optional(&mut *tx).await?
            .ok_or(RepoError::NotFound)?;
        let new_qty = current.checked_add(delta)
            .ok_or_else(|| RepoError::validation("stock overflow"))?;
        if new_qty < 0 {
            return Err(RepoError::validation("insufficient stock"));
        }

        sqlx::query("UPDATE item SET stock_qty = ? WHERE id = ?")
            .bind(new_qty).bind(m.item_id).execute(&mut *tx).await?;

        let id = sqlx::query_scalar::<_, i64>(
            r#"INSERT INTO stock_movement
                 (item_id, movement, quantity, reason, ref_type, ref_id)
               VALUES (?, ?, ?, ?, ?, ?) RETURNING id"#,
        )
        .bind(m.item_id).bind(&m.movement).bind(m.quantity)
        .bind(&m.reason).bind(&m.ref_type).bind(m.ref_id)
        .fetch_one(&mut *tx).await?;

        tx.commit().await?;

        sqlx::query_as::<_, StockMovement>("SELECT * FROM stock_movement WHERE id = ?")
            .bind(id).fetch_one(&self.pool).await.map_err(Into::into)
    }

    pub async fn history(&self, item_id: i64, limit: i64) -> RepoResult<Vec<StockMovement>> {
        Ok(sqlx::query_as::<_, StockMovement>(
            "SELECT * FROM stock_movement WHERE item_id = ? ORDER BY moved_on DESC LIMIT ?",
        ).bind(item_id).bind(limit).fetch_all(&self.pool).await?)
    }
}

// ---------- Purchase Order ----------

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PurchaseOrder {
    pub id: i64,
    pub po_no: String,
    pub vendor_id: i64,
    pub order_date: chrono::NaiveDate,
    pub expected_date: Option<chrono::NaiveDate>,
    pub status: String,
    pub total_cents: i64,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewPurchaseOrder {
    pub po_no: String,
    pub vendor_id: i64,
    pub order_date: chrono::NaiveDate,
    pub expected_date: Option<chrono::NaiveDate>,
    pub notes: Option<String>,
    pub lines: Vec<POLine>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct POLine {
    pub item_id: i64,
    pub quantity: i64,
    pub unit_cost_cents: i64,
}

#[derive(Clone)]
pub struct PurchaseOrderRepo { pool: SqlitePool }

impl PurchaseOrderRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn create(&self, p: &NewPurchaseOrder) -> RepoResult<PurchaseOrder> {
        if p.lines.is_empty() { return Err(RepoError::validation("PO needs lines")); }
        let total: i64 = p.lines.iter().map(|l| l.quantity * l.unit_cost_cents).sum();

        let mut tx = self.pool.begin().await?;

        let id = sqlx::query_scalar::<_, i64>(
            r#"INSERT INTO purchase_order
                 (po_no, vendor_id, order_date, expected_date, status, total_cents, notes)
               VALUES (?, ?, ?, ?, 'draft', ?, ?) RETURNING id"#,
        )
        .bind(&p.po_no).bind(p.vendor_id).bind(p.order_date).bind(p.expected_date)
        .bind(total).bind(&p.notes)
        .fetch_one(&mut *tx).await?;

        for l in &p.lines {
            sqlx::query(
                r#"INSERT INTO purchase_order_line
                     (po_id, item_id, quantity, unit_cost_cents)
                   VALUES (?, ?, ?, ?)"#,
            )
            .bind(id).bind(l.item_id).bind(l.quantity).bind(l.unit_cost_cents)
            .execute(&mut *tx).await?;
        }

        tx.commit().await?;
        self.get(id).await
    }

    pub async fn get(&self, id: i64) -> RepoResult<PurchaseOrder> {
        sqlx::query_as::<_, PurchaseOrder>("SELECT * FROM purchase_order WHERE id = ?")
            .bind(id).fetch_optional(&self.pool).await?
            .ok_or(RepoError::NotFound)
    }

    pub async fn set_status(&self, id: i64, status: &str) -> RepoResult<()> {
        if !matches!(status, "draft"|"ordered"|"received"|"cancelled") {
            return Err(RepoError::validation("invalid status"));
        }
        sqlx::query("UPDATE purchase_order SET status = ? WHERE id = ?")
            .bind(status).bind(id).execute(&self.pool).await?;
        Ok(())
    }

    /// When a PO is received, increment stock for each line via stock_movement.
    pub async fn receive(&self, id: i64) -> RepoResult<()> {
        let mut tx = self.pool.begin().await?;

        let lines: Vec<(i64, i64)> = sqlx::query_as(
            "SELECT item_id, quantity FROM purchase_order_line WHERE po_id = ?",
        ).bind(id).fetch_all(&mut *tx).await?;

        for (item_id, qty) in lines {
            sqlx::query("UPDATE item SET stock_qty = stock_qty + ? WHERE id = ?")
                .bind(qty).bind(item_id).execute(&mut *tx).await?;
            sqlx::query(
                r#"INSERT INTO stock_movement (item_id, movement, quantity, reason, ref_type, ref_id)
                   VALUES (?, 'in', ?, 'PO receipt', 'purchase_order', ?)"#,
            )
            .bind(item_id).bind(qty).bind(id).execute(&mut *tx).await?;
        }

        sqlx::query("UPDATE purchase_order SET status = 'received' WHERE id = ?")
            .bind(id).execute(&mut *tx).await?;
        tx.commit().await?;
        Ok(())
    }
}
