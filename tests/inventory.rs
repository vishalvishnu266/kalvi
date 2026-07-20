mod common;

use common::{date, fixture};
use school_erp::repositories::inventory::{
    Item, NewPurchaseOrder, POLine, Vendor,
};
use school_erp::ServiceError;

#[tokio::test]
async fn po_lifecycle_receives_stock_and_posts_journal() {
    let fx = fixture().await;

    let vendor = fx.app.repos.vendors.create(&Vendor {
        id: 0, name: "ACME".into(), contact: None,
        phone: None, email: None, address: None,
    }).await.unwrap();

    let item = fx.app.repos.items.create(&Item {
        id: 0, name: "Whiteboard Marker".into(),
        sku: Some("SKU-1".into()), unit: Some("pcs".into()),
        stock_qty: 0, reorder_level: Some(5), unit_cost_cents: Some(200),
    }).await.unwrap();
    assert_eq!(item.stock_qty, 0);

    let po = fx.app.repos.purchase_orders.create(&NewPurchaseOrder {
        po_no: "PO-1".into(),
        vendor_id: vendor.id,
        order_date: date(2025, 6, 1),
        expected_date: Some(date(2025, 6, 10)),
        notes: None,
        lines: vec![POLine {
            item_id: item.id, quantity: 20, unit_cost_cents: 200,
        }],
    }).await.unwrap();
    assert_eq!(po.total_cents, 4_000);
    assert_eq!(po.status, "draft");

    // draft → ordered → received
    fx.app.inventory.set_po_status(po.id, "ordered").await.unwrap();
    fx.app.inventory.set_po_status(po.id, "received").await.unwrap();

    let after = fx.app.repos.items.get(item.id).await.unwrap();
    assert_eq!(after.stock_qty, 20);

    let tb = fx.app.repos.ledger.trial_balance(date(2025, 6, 30)).await.unwrap();
    let (dr, cr): (i64, i64) = tb.iter().map(|(_,d,c)| (*d, *c))
        .fold((0, 0), |(a,b),(d,c)| (a+d, b+c));
    assert_eq!(dr, cr);
    assert_eq!(dr, 4_000);
}

#[tokio::test]
async fn invalid_po_transition_is_rejected() {
    let fx = fixture().await;

    let vendor = fx.app.repos.vendors.create(&Vendor {
        id: 0, name: "V".into(), contact: None, phone: None, email: None, address: None,
    }).await.unwrap();
    let item = fx.app.repos.items.create(&Item {
        id: 0, name: "X".into(), sku: None, unit: None,
        stock_qty: 0, reorder_level: None, unit_cost_cents: None,
    }).await.unwrap();
    let po = fx.app.repos.purchase_orders.create(&NewPurchaseOrder {
        po_no: "PO-B".into(), vendor_id: vendor.id,
        order_date: date(2025, 6, 1), expected_date: None, notes: None,
        lines: vec![POLine { item_id: item.id, quantity: 1, unit_cost_cents: 100 }],
    }).await.unwrap();

    // draft -> received is not allowed
    let err = fx.app.inventory.set_po_status(po.id, "received").await.unwrap_err();
    assert!(matches!(err, ServiceError::Conflict(_)));
}
