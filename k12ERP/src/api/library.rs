//! `/api/tenant/library/*`

use axum::{
    extract::{Path, Query},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;

use crate::http::{ExtractServices, ServiceHttpError};
use crate::http::middleware::TenantScopeState;
use crate::repositories::library::{Book, BookIssue, NewBook};

pub fn routes() -> Router<TenantScopeState> {
    Router::new()
        .route("/books",              get(list_books).post(create_book))
        .route("/books/search",       get(search))
        .route("/books/{id}",          get(get_book))
        .route("/books/{id}/adjust",   post(adjust))
        .route("/issues/student",     post(issue_student))
        .route("/issues/staff",       post(issue_staff))
        .route("/issues/{id}/return",  post(return_book))
        .route("/issues/overdue",     get(overdue))
}

#[derive(Deserialize)] struct Page { #[serde(default = "d50")] limit: i64, #[serde(default)] offset: i64 }
fn d50() -> i64 { 50 }

async fn list_books(ExtractServices(a): ExtractServices, Query(p): Query<Page>)
    -> Result<Json<Vec<Book>>, ServiceHttpError>
{ Ok(Json(a.repos.books.list(p.limit, p.offset).await.map_err(re)?)) }

async fn create_book(ExtractServices(a): ExtractServices, Json(b): Json<NewBook>)
    -> Result<Json<Book>, ServiceHttpError>
{ Ok(Json(a.repos.books.create(&b).await.map_err(re)?)) }

#[derive(Deserialize)] struct SearchQ { q: String, #[serde(default = "d50")] limit: i64 }

async fn search(ExtractServices(a): ExtractServices, Query(q): Query<SearchQ>)
    -> Result<Json<Vec<Book>>, ServiceHttpError>
{ Ok(Json(a.repos.books.search(&q.q, q.limit).await.map_err(re)?)) }

async fn get_book(ExtractServices(a): ExtractServices, Path(id): Path<i64>)
    -> Result<Json<Book>, ServiceHttpError>
{ Ok(Json(a.repos.books.get(id).await.map_err(re)?)) }

#[derive(Deserialize)] struct Adjust { delta: i64 }

async fn adjust(ExtractServices(a): ExtractServices, Path(id): Path<i64>, Json(b): Json<Adjust>)
    -> Result<axum::http::StatusCode, ServiceHttpError>
{ a.repos.books.adjust_copies(id, b.delta).await.map_err(re)?; Ok(axum::http::StatusCode::NO_CONTENT) }

#[derive(Deserialize)] struct IssueStudent { book_id: i64, student_id: i64, on: chrono::NaiveDate }

async fn issue_student(ExtractServices(a): ExtractServices, Json(b): Json<IssueStudent>)
    -> Result<Json<BookIssue>, ServiceHttpError>
{ Ok(Json(a.library.issue_to_student(b.book_id, b.student_id, b.on).await?)) }

#[derive(Deserialize)] struct IssueStaff { book_id: i64, staff_id: i64, on: chrono::NaiveDate }

async fn issue_staff(ExtractServices(a): ExtractServices, Json(b): Json<IssueStaff>)
    -> Result<Json<BookIssue>, ServiceHttpError>
{ Ok(Json(a.library.issue_to_staff(b.book_id, b.staff_id, b.on).await?)) }

#[derive(Deserialize)] struct ReturnBody { returned_on: chrono::NaiveDate }

async fn return_book(
    ExtractServices(a): ExtractServices, Path(id): Path<i64>, Json(b): Json<ReturnBody>,
) -> Result<Json<serde_json::Value>, ServiceHttpError> {
    let fine = a.library.return_book(id, b.returned_on).await?;
    Ok(Json(serde_json::json!({ "fine_cents": fine })))
}

#[derive(Deserialize)] struct Today { today: chrono::NaiveDate }

async fn overdue(ExtractServices(a): ExtractServices, Query(q): Query<Today>)
    -> Result<Json<Vec<BookIssue>>, ServiceHttpError>
{ Ok(Json(a.library.overdue(q.today).await?)) }

fn re(e: crate::error::RepoError) -> ServiceHttpError { ServiceHttpError(e.into()) }
