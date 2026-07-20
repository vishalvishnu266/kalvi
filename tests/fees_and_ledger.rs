mod common;

use common::{date, fixture};
use school_erp::repositories::fees::{FeeDiscount, NewPayment, NewStructureItem};
use school_erp::ServiceError;

#[tokio::test]
async fn invoice_from_structure_applies_percent_discount() {
    let fx = fixture().await;

    // Tuition = 10 000 cents (₹100), monthly
    let structure = fx.app.repos.fee_structures
        .create(fx.year_id, fx.grade_id, "Standard").await.unwrap();
    let tuition = fx.app.repos.fee_categories.list().await.unwrap()
        .into_iter().find(|c| c.name == "Tuition").unwrap();
    fx.app.repos.fee_structures.add_item(structure.id, &NewStructureItem {
        fee_category_id: tuition.id,
        amount_cents: 10_000,
        frequency: "monthly".into(),
        due_day: Some(5),
    }).await.unwrap();

    // 10% discount active for the student on issue date.
    fx.app.repos.discounts.grant(&FeeDiscount {
        id: 0,
        student_id: fx.student_id,
        name: "Sibling".into(),
        percent: Some(10.0),
        flat_cents: None,
        valid_from: Some(date(2025, 6, 1)),
        valid_to: Some(date(2026, 3, 31)),
    }).await.unwrap();

    let invoice = fx.app.fees.generate_invoice_for_student(
        fx.student_id, structure.id,
        "INV-0001".into(), date(2025, 6, 1), date(2025, 6, 15), 0,
    ).await.unwrap();

    assert_eq!(invoice.subtotal_cents, 10_000);
    assert_eq!(invoice.discount_cents, 1_000);
    assert_eq!(invoice.total_cents,    9_000);
    assert_eq!(invoice.status, "unpaid");
}

#[tokio::test]
async fn payment_updates_invoice_and_posts_journal() {
    let fx = fixture().await;

    // Minimal fee structure.
    let structure = fx.app.repos.fee_structures
        .create(fx.year_id, fx.grade_id, "Standard").await.unwrap();
    let tuition = fx.app.repos.fee_categories.list().await.unwrap()
        .into_iter().find(|c| c.name == "Tuition").unwrap();
    fx.app.repos.fee_structures.add_item(structure.id, &NewStructureItem {
        fee_category_id: tuition.id, amount_cents: 5_000,
        frequency: "one_time".into(), due_day: None,
    }).await.unwrap();

    let inv = fx.app.fees.generate_invoice_for_student(
        fx.student_id, structure.id,
        "INV-0002".into(), date(2025, 6, 1), date(2025, 6, 30), 0,
    ).await.unwrap();
    assert_eq!(inv.total_cents, 5_000);

    // Partial payment
    fx.app.fees.record_payment(NewPayment {
        receipt_no: "RCPT-1".into(),
        invoice_id: inv.id,
        paid_on: date(2025, 6, 5),
        amount_cents: 2_000,
        method: "cash".into(),
        reference: None,
        received_by_staff_id: Some(fx.teacher_id),
    }).await.unwrap();
    let inv2 = fx.app.repos.invoices.get(inv.id).await.unwrap();
    assert_eq!(inv2.paid_cents, 2_000);
    assert_eq!(inv2.status, "partial");

    // Full payment closes it out
    fx.app.fees.record_payment(NewPayment {
        receipt_no: "RCPT-2".into(),
        invoice_id: inv.id,
        paid_on: date(2025, 6, 6),
        amount_cents: 3_000,
        method: "cash".into(),
        reference: None,
        received_by_staff_id: None,
    }).await.unwrap();
    let inv3 = fx.app.repos.invoices.get(inv.id).await.unwrap();
    assert_eq!(inv3.paid_cents, 5_000);
    assert_eq!(inv3.status, "paid");

    // Ledger is balanced (debits == credits).
    let tb = fx.app.repos.ledger.trial_balance(date(2025, 6, 30)).await.unwrap();
    let (dr, cr): (i64, i64) = tb.iter().map(|(_,d,c)| (*d,*c))
        .fold((0,0), |(a,b), (d,c)| (a+d, b+c));
    assert_eq!(dr, cr);
    assert_eq!(dr, 5_000);
}

#[tokio::test]
async fn overpayment_is_rejected() {
    let fx = fixture().await;

    let structure = fx.app.repos.fee_structures
        .create(fx.year_id, fx.grade_id, "S").await.unwrap();
    let cat = fx.app.repos.fee_categories.list().await.unwrap().remove(0);
    fx.app.repos.fee_structures.add_item(structure.id, &NewStructureItem {
        fee_category_id: cat.id, amount_cents: 1_000,
        frequency: "one_time".into(), due_day: None,
    }).await.unwrap();
    let inv = fx.app.fees.generate_invoice_for_student(
        fx.student_id, structure.id, "INV-9".into(),
        date(2025, 6, 1), date(2025, 6, 30), 0,
    ).await.unwrap();

    let err = fx.app.fees.record_payment(NewPayment {
        receipt_no: "R".into(), invoice_id: inv.id,
        paid_on: date(2025, 6, 5), amount_cents: 5_000,
        method: "cash".into(), reference: None,
        received_by_staff_id: None,
    }).await.unwrap_err();
    // Repo raises validation → surfaces as ServiceError::Repo(Validation)
    assert!(matches!(err, ServiceError::Repo(_)));
}

#[tokio::test]
async fn cancel_invoice_only_if_unpaid() {
    let fx = fixture().await;

    let structure = fx.app.repos.fee_structures
        .create(fx.year_id, fx.grade_id, "S").await.unwrap();
    let cat = fx.app.repos.fee_categories.list().await.unwrap().remove(0);
    fx.app.repos.fee_structures.add_item(structure.id, &NewStructureItem {
        fee_category_id: cat.id, amount_cents: 500,
        frequency: "one_time".into(), due_day: None,
    }).await.unwrap();
    let inv = fx.app.fees.generate_invoice_for_student(
        fx.student_id, structure.id, "INV-C".into(),
        date(2025, 6, 1), date(2025, 6, 30), 0,
    ).await.unwrap();

    fx.app.fees.cancel_invoice(inv.id).await.unwrap();
    let cancelled = fx.app.repos.invoices.get(inv.id).await.unwrap();
    assert_eq!(cancelled.status, "cancelled");
}
