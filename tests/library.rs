mod common;

use common::{date, fixture};
use school_erp::repositories::library::NewBook;
use school_erp::ServiceError;

#[tokio::test]
async fn issue_and_return_updates_available_and_computes_fine() {
    let fx = fixture().await;

    let book = fx.app.repos.books.create(&NewBook {
        isbn: Some("978-1-2345".into()),
        title: "The Rust Book".into(),
        author: Some("Klabnik".into()),
        publisher: None, category: Some("Programming".into()),
        total_copies: 2,
    }).await.unwrap();
    assert_eq!(book.available, 2);

    let issue = fx.app.library
        .issue_to_student(book.id, fx.student_id, date(2025, 6, 1))
        .await.unwrap();
    assert_eq!(issue.due_on, date(2025, 6, 15));

    let refreshed = fx.app.repos.books.get(book.id).await.unwrap();
    assert_eq!(refreshed.available, 1);

    // Returned 5 days late → fine = 5 * 500 cents.
    let fine = fx.app.library.return_book(issue.id, date(2025, 6, 20)).await.unwrap();
    assert_eq!(fine, 2_500);

    let after = fx.app.repos.books.get(book.id).await.unwrap();
    assert_eq!(after.available, 2);
}

#[tokio::test]
async fn cannot_issue_when_no_copies_available() {
    let fx = fixture().await;

    let book = fx.app.repos.books.create(&NewBook {
        isbn: None, title: "Only One".into(), author: None,
        publisher: None, category: None, total_copies: 1,
    }).await.unwrap();

    fx.app.library.issue_to_student(book.id, fx.student_id, date(2025, 6, 1))
        .await.unwrap();

    // Second student
    let other = fx.app.repos.students.create(&school_erp::repositories::students::NewStudent {
        admission_no: "ADM-002".into(),
        user_id: None,
        first_name: "E".into(), middle_name: None, last_name: "F".into(),
        date_of_birth: date(2015, 1, 1),
        gender: None, blood_group: None, nationality: None, religion: None,
        photo_path: None, admission_date: date(2025, 6, 1),
        address_line1: None, address_line2: None,
        city: None, state: None, postal_code: None, country: None,
    }).await.unwrap();

    let err = fx.app.library.issue_to_student(book.id, other.id, date(2025, 6, 2))
        .await.unwrap_err();
    assert!(matches!(err, ServiceError::Repo(_)));
}
