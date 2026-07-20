mod common;

use common::{date, fixture};
use school_erp::repositories::payroll::NewStructureItem;
use school_erp::ServiceError;

async fn seed_components(app: &school_erp::services::AppServices) -> (i64, i64) {
    let basic = app.repos.salary_components.list().await.unwrap()
        .into_iter().find(|c| c.name == "Basic").unwrap();
    let pf = app.repos.salary_components.list().await.unwrap()
        .into_iter().find(|c| c.name == "Provident Fund").unwrap();
    (basic.id, pf.id)
}

#[tokio::test]
async fn payslip_lifecycle_posts_ledger_on_pay() {
    let fx = fixture().await;
    let (basic, pf) = seed_components(&fx.app).await;

    fx.app.payroll.set_salary(fx.teacher_id, date(2025, 6, 1), vec![
        NewStructureItem { component_id: basic, amount_cents: 100_000 },
        NewStructureItem { component_id: pf,    amount_cents:  10_000 },
    ]).await.unwrap();

    let payslip = fx.app.payroll.generate_payslip(fx.teacher_id, 6, 2025).await.unwrap();
    assert_eq!(payslip.gross_cents, 100_000);
    assert_eq!(payslip.deduction_cents, 10_000);
    assert_eq!(payslip.net_cents, 90_000);
    assert_eq!(payslip.status, "draft");

    // Can't pay from draft
    let err = fx.app.payroll.pay(payslip.id, date(2025, 6, 30), true).await.unwrap_err();
    assert!(matches!(err, ServiceError::Conflict(_)));

    fx.app.payroll.approve(payslip.id).await.unwrap();
    fx.app.payroll.pay(payslip.id, date(2025, 6, 30), true).await.unwrap();

    // Ledger balanced with net amount
    let tb = fx.app.repos.ledger.trial_balance(date(2025, 6, 30)).await.unwrap();
    let (dr, cr): (i64, i64) = tb.iter().map(|(_,d,c)| (*d, *c))
        .fold((0, 0), |(a,b), (d,c)| (a+d, b+c));
    assert_eq!(dr, cr);
    assert_eq!(dr, 90_000);
}

#[tokio::test]
async fn cannot_generate_payslip_without_salary_structure() {
    let fx = fixture().await;
    let err = fx.app.payroll.generate_payslip(fx.teacher_id, 6, 2025).await.unwrap_err();
    // Repo rejects with validation → wrapped as Repo error at the service edge
    assert!(matches!(err, ServiceError::Repo(_)));
}
