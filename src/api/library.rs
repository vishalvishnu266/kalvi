use axum::{extract::{Path, Query}, http::StatusCode, Json};
use serde::Deserialize;

use crate::http::{ServiceHttpError, TenantScope};
use crate::repositories::library::{Book, BookIssue, NewBook};
use crate::services::perm;

#[derive(Deserialize)] pub struct Page { #[serde(default = "d50")] limit: i64, #[serde(default)] offset: i64 }
fn d50() -> i64 { 50 }

pub async fn list_books(scope: TenantScope, Query(p): Query<Page>)
    -> Result<Json<Vec<Book>>, ServiceHttpError>
{
    scope.ctx.require(perm::LIBRARY_VIEW)?;
    Ok(Json(scope.services.repos.books.list(p.limit, p.offset).await?))
}

pub async fn create_book(scope: TenantScope, Json(b): Json<NewBook>)
    -> Result<Json<Book>, ServiceHttpError>
{
    scope.ctx.require(perm::LIBRARY_MANAGE)?;
    Ok(Json(scope.services.repos.books.create(&b).await?))
}

#[derive(Deserialize)] pub struct SearchQ { q: String, #[serde(default = "d50")] limit: i64 }

pub async fn search(scope: TenantScope, Query(q): Query<SearchQ>)
    -> Result<Json<Vec<Book>>, ServiceHttpError>
{
    scope.ctx.require(perm::LIBRARY_VIEW)?;
    Ok(Json(scope.services.repos.books.search(&q.q, q.limit).await?))
}

pub async fn get_book(scope: TenantScope, Path((_t, id)): Path<(String, i64)>)
    -> Result<Json<Book>, ServiceHttpError>
{
    scope.ctx.require(perm::LIBRARY_VIEW)?;
    Ok(Json(scope.services.repos.books.get(id).await?))
}

#[derive(Deserialize)] pub struct Adjust { delta: i64 }

pub async fn adjust(scope: TenantScope, Path((_t, id)): Path<(String, i64)>, Json(b): Json<Adjust>)
    -> Result<StatusCode, ServiceHttpError>
{
    scope.ctx.require(perm::LIBRARY_MANAGE)?;
    scope.services.repos.books.adjust_copies(id, b.delta).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize)] pub struct IssueStudent { book_id: i64, student_id: i64, on: chrono::NaiveDate }

pub async fn issue_student(scope: TenantScope, Json(b): Json<IssueStudent>)
    -> Result<Json<BookIssue>, ServiceHttpError>
{ Ok(Json(scope.services.library.issue_to_student(&scope.ctx, b.book_id, b.student_id, b.on).await?)) }

#[derive(Deserialize)] pub struct IssueStaff { book_id: i64, staff_id: i64, on: chrono::NaiveDate }

pub async fn issue_staff(scope: TenantScope, Json(b): Json<IssueStaff>)
    -> Result<Json<BookIssue>, ServiceHttpError>
{ Ok(Json(scope.services.library.issue_to_staff(&scope.ctx, b.book_id, b.staff_id, b.on).await?)) }

#[derive(Deserialize)] pub struct ReturnBody { returned_on: chrono::NaiveDate }

pub async fn return_book(
    scope: TenantScope, Path((_t, id)): Path<(String, i64)>, Json(b): Json<ReturnBody>,
) -> Result<Json<serde_json::Value>, ServiceHttpError> {
    let fine = scope.services.library.return_book(&scope.ctx, id, b.returned_on).await?;
    Ok(Json(serde_json::json!({ "fine_cents": fine })))
}

#[derive(Deserialize)] pub struct Today { today: chrono::NaiveDate }

pub async fn overdue(scope: TenantScope, Query(q): Query<Today>)
    -> Result<Json<Vec<BookIssue>>, ServiceHttpError>
{ Ok(Json(scope.services.library.overdue(&scope.ctx, q.today).await?)) }
