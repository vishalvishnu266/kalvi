-- =====================================================================
-- 013 INVENTORY / ASSETS
-- =====================================================================
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS vendor (
    id       INTEGER PRIMARY KEY,
    name     TEXT NOT NULL UNIQUE,
    contact  TEXT,
    phone    TEXT,
    email    TEXT,
    address  TEXT
);

CREATE TABLE IF NOT EXISTS item (
    id            INTEGER PRIMARY KEY,
    name          TEXT NOT NULL,
    sku           TEXT UNIQUE,
    unit          TEXT,
    stock_qty     INTEGER NOT NULL DEFAULT 0 CHECK (stock_qty >= 0),
    reorder_level INTEGER CHECK (reorder_level IS NULL OR reorder_level >= 0),
    unit_cost_cents INTEGER CHECK (unit_cost_cents IS NULL OR unit_cost_cents >= 0)
);

CREATE TABLE IF NOT EXISTS stock_movement (
    id         INTEGER PRIMARY KEY,
    item_id    INTEGER NOT NULL REFERENCES item(id) ON DELETE RESTRICT,
    movement   TEXT NOT NULL CHECK (movement IN ('in','out','adjust')),
    quantity   INTEGER NOT NULL,
    reason     TEXT,
    ref_type   TEXT,
    ref_id     INTEGER,
    moved_on   TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX IF NOT EXISTS ix_stock_item_date ON stock_movement(item_id, moved_on);

CREATE TABLE IF NOT EXISTS purchase_order (
    id             INTEGER PRIMARY KEY,
    po_no          TEXT NOT NULL UNIQUE,
    vendor_id      INTEGER NOT NULL REFERENCES vendor(id) ON DELETE RESTRICT,
    order_date     TEXT NOT NULL,
    expected_date  TEXT,
    status         TEXT NOT NULL DEFAULT 'draft'
                   CHECK (status IN ('draft','ordered','received','cancelled')),
    total_cents    INTEGER NOT NULL DEFAULT 0 CHECK (total_cents >= 0),
    notes          TEXT
);

CREATE TABLE IF NOT EXISTS purchase_order_line (
    id              INTEGER PRIMARY KEY,
    po_id           INTEGER NOT NULL REFERENCES purchase_order(id) ON DELETE CASCADE,
    item_id         INTEGER NOT NULL REFERENCES item(id) ON DELETE RESTRICT,
    quantity        INTEGER NOT NULL CHECK (quantity > 0),
    unit_cost_cents INTEGER NOT NULL CHECK (unit_cost_cents >= 0)
);
