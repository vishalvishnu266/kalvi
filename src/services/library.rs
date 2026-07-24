use std::sync::Arc;

use chrono::NaiveDate;

use crate::repositories::Repositories;
use crate::repositories::library::{BookIssue, IssueBook};
use crate::services::{ServiceError, ServiceResult};

pub const DEFAULT_LOAN_DAYS: i64 = 14;
pub const DEFAULT_FINE_PER_DAY_CENTS: i64 = 500;

#[derive(Clone)]
pub struct LibraryService {
    repos: Arc<Repositories>,
    loan_days: i64,
    fine_per_day_cents: i64,
}

impl LibraryService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self {
            repos,
            loan_days: DEFAULT_LOAN_DAYS,
            fine_per_day_cents: DEFAULT_FINE_PER_DAY_CENTS,
        }
    }

    pub fn with_policy(mut self, loan_days: i64, fine_per_day_cents: i64) -> Self {
        self.loan_days = loan_days;
        self.fine_per_day_cents = fine_per_day_cents;
        self
    }

pub async fn issue_to_student(
        &self, book_id: i64, student_id: i64, on: NaiveDate,
    ) -> ServiceResult<BookIssue> {
        let due = on + chrono::Duration::days(self.loan_days);
        Ok(self.repos.book_issues.issue(&IssueBook {
            book_id, student_id: Some(student_id), staff_id: None,
            issued_on: on, due_on: due,
        }).await?)
    }

    pub async fn issue_to_staff(
        &self, book_id: i64, staff_id: i64, on: NaiveDate,
    ) -> ServiceResult<BookIssue> {
        let due = on + chrono::Duration::days(self.loan_days);
        Ok(self.repos.book_issues.issue(&IssueBook {
            book_id, student_id: None, staff_id: Some(staff_id),
            issued_on: on, due_on: due,
        }).await?)
    }

pub async fn return_book(&self, issue_id: i64, returned_on: NaiveDate) -> ServiceResult<i64> {
        let issue = self.repos.book_issues.get(issue_id).await?;
        if issue.returned_on.is_some() {
            return Err(ServiceError::conflict("book already returned"));
        }
        let days_late = (returned_on - issue.due_on).num_days().max(0);
        let fine = days_late * self.fine_per_day_cents;
        self.repos.book_issues.return_book(issue_id, returned_on, fine).await?;
        Ok(fine)
    }

    pub async fn overdue(&self, today: NaiveDate) -> ServiceResult<Vec<BookIssue>> {
        Ok(self.repos.book_issues.overdue(today).await?)
    }
}
