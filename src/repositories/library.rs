//! Library: books and issues.

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

use crate::error::{RepoError, RepoResult};

// ---------- Book ----------

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Book {
    pub id: i64,
    pub isbn: Option<String>,
    pub title: String,
    pub author: Option<String>,
    pub publisher: Option<String>,
    pub category: Option<String>,
    pub total_copies: i64,
    pub available: i64,
}

#[derive(Debug, Clone)]
pub struct NewBook {
    pub isbn: Option<String>,
    pub title: String,
    pub author: Option<String>,
    pub publisher: Option<String>,
    pub category: Option<String>,
    pub total_copies: i64,
}

#[derive(Clone)]
pub struct BookRepo { pool: SqlitePool }

impl BookRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn create(&self, b: &NewBook) -> RepoResult<Book> {
        if b.total_copies < 0 {
            return Err(RepoError::validation("total_copies must be >= 0"));
        }
        let id = sqlx::query_scalar::<_, i64>(
            r#"INSERT INTO book (isbn, title, author, publisher, category, total_copies, available)
               VALUES (?, ?, ?, ?, ?, ?, ?) RETURNING id"#,
        )
        .bind(&b.isbn).bind(&b.title).bind(&b.author).bind(&b.publisher).bind(&b.category)
        .bind(b.total_copies).bind(b.total_copies)
        .fetch_one(&self.pool).await?;
        self.get(id).await
    }

    pub async fn get(&self, id: i64) -> RepoResult<Book> {
        sqlx::query_as::<_, Book>("SELECT * FROM book WHERE id = ?")
            .bind(id).fetch_optional(&self.pool).await?
            .ok_or(RepoError::NotFound)
    }

    pub async fn search(&self, q: &str, limit: i64) -> RepoResult<Vec<Book>> {
        let like = format!("%{}%", q);
        Ok(sqlx::query_as::<_, Book>(
            r#"SELECT * FROM book
               WHERE title LIKE ? OR author LIKE ? OR isbn LIKE ?
               ORDER BY title LIMIT ?"#,
        ).bind(&like).bind(&like).bind(&like).bind(limit)
        .fetch_all(&self.pool).await?)
    }

    pub async fn list(&self, limit: i64, offset: i64) -> RepoResult<Vec<Book>> {
        Ok(sqlx::query_as::<_, Book>(
            "SELECT * FROM book ORDER BY title LIMIT ? OFFSET ?",
        ).bind(limit).bind(offset).fetch_all(&self.pool).await?)
    }

    pub async fn adjust_copies(&self, id: i64, delta: i64) -> RepoResult<()> {
        sqlx::query(
            r#"UPDATE book
                 SET total_copies = total_copies + ?,
                     available    = available    + ?
               WHERE id = ?"#,
        ).bind(delta).bind(delta).bind(id).execute(&self.pool).await?;
        Ok(())
    }
}

// ---------- Book issue ----------

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct BookIssue {
    pub id: i64,
    pub book_id: i64,
    pub student_id: Option<i64>,
    pub staff_id: Option<i64>,
    pub issued_on: NaiveDate,
    pub due_on: NaiveDate,
    pub returned_on: Option<NaiveDate>,
    pub fine_cents: i64,
}

#[derive(Debug, Clone)]
pub struct IssueBook {
    pub book_id: i64,
    pub student_id: Option<i64>,
    pub staff_id: Option<i64>,
    pub issued_on: NaiveDate,
    pub due_on: NaiveDate,
}

#[derive(Clone)]
pub struct BookIssueRepo { pool: SqlitePool }

impl BookIssueRepo {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    /// Issue a copy of a book, decrementing `book.available`.
    pub async fn issue(&self, i: &IssueBook) -> RepoResult<BookIssue> {
        if i.student_id.is_none() && i.staff_id.is_none() {
            return Err(RepoError::validation("student_id or staff_id required"));
        }
        if i.issued_on > i.due_on {
            return Err(RepoError::validation("due_on must be >= issued_on"));
        }

        let mut tx = self.pool.begin().await?;

        let available: i64 = sqlx::query_scalar("SELECT available FROM book WHERE id = ?")
            .bind(i.book_id).fetch_optional(&mut *tx).await?
            .ok_or(RepoError::NotFound)?;
        if available <= 0 {
            return Err(RepoError::conflict("no copies available"));
        }

        sqlx::query("UPDATE book SET available = available - 1 WHERE id = ?")
            .bind(i.book_id).execute(&mut *tx).await?;

        let id = sqlx::query_scalar::<_, i64>(
            r#"INSERT INTO book_issue
                 (book_id, student_id, staff_id, issued_on, due_on)
               VALUES (?, ?, ?, ?, ?) RETURNING id"#,
        )
        .bind(i.book_id).bind(i.student_id).bind(i.staff_id).bind(i.issued_on).bind(i.due_on)
        .fetch_one(&mut *tx).await?;

        tx.commit().await?;
        self.get(id).await
    }

    /// Return an issued book. Optionally apply a fine (cents).
    pub async fn return_book(
        &self, issue_id: i64, returned_on: NaiveDate, fine_cents: i64,
    ) -> RepoResult<()> {
        if fine_cents < 0 {
            return Err(RepoError::validation("fine_cents must be >= 0"));
        }
        let mut tx = self.pool.begin().await?;

        let (book_id, already_returned): (i64, Option<NaiveDate>) = sqlx::query_as(
            "SELECT book_id, returned_on FROM book_issue WHERE id = ?",
        ).bind(issue_id).fetch_optional(&mut *tx).await?
         .ok_or(RepoError::NotFound)?;

        if already_returned.is_some() {
            return Err(RepoError::conflict("book already returned"));
        }

        sqlx::query(
            "UPDATE book_issue SET returned_on = ?, fine_cents = ? WHERE id = ?",
        ).bind(returned_on).bind(fine_cents).bind(issue_id).execute(&mut *tx).await?;

        sqlx::query("UPDATE book SET available = available + 1 WHERE id = ?")
            .bind(book_id).execute(&mut *tx).await?;

        tx.commit().await?;
        Ok(())
    }

    pub async fn get(&self, id: i64) -> RepoResult<BookIssue> {
        sqlx::query_as::<_, BookIssue>("SELECT * FROM book_issue WHERE id = ?")
            .bind(id).fetch_optional(&self.pool).await?
            .ok_or(RepoError::NotFound)
    }

    pub async fn open_issues_for_student(&self, student_id: i64) -> RepoResult<Vec<BookIssue>> {
        Ok(sqlx::query_as::<_, BookIssue>(
            "SELECT * FROM book_issue WHERE student_id = ? AND returned_on IS NULL",
        ).bind(student_id).fetch_all(&self.pool).await?)
    }

    pub async fn overdue(&self, today: NaiveDate) -> RepoResult<Vec<BookIssue>> {
        Ok(sqlx::query_as::<_, BookIssue>(
            "SELECT * FROM book_issue WHERE returned_on IS NULL AND due_on < ? ORDER BY due_on",
        ).bind(today).fetch_all(&self.pool).await?)
    }
}
