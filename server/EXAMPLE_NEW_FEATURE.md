# Example: Adding a New Feature

This guide shows how to add "List Students" feature to demonstrate the AI-friendly pattern.

## Create: `crates/student/src/list_students.rs`

```rust
// Feature: List All Students
// Everything needed for the student list page in ONE file

use axum::{
    extract::{Extension, Query},
    response::{Html, IntoResponse, Json},
    routing::get,
    Router,
};
use maud::{html, Markup, DOCTYPE};
use serde::Deserialize;
use sqlx::SqlitePool;

use super::shared::{db, Student};
use super::AppState;

// ============================================================================
// REQUEST DTOs
// ============================================================================

#[derive(Deserialize)]
pub struct ListQuery {
    #[serde(default)]
    search: String,
    #[serde(default)]
    page: usize,
}

// ============================================================================
// DATABASE LAYER
// ============================================================================

async fn fetch_students(
    pool: &SqlitePool,
    search: &str,
    page: usize,
) -> Result<Vec<Student>, sqlx::Error> {
    let limit = 20;
    let offset = (page * limit) as i64;
    
    let search_pattern = format!("%{}%", search);
    
    sqlx::query_as::<_, Student>(
        "SELECT id, name FROM students 
         WHERE name LIKE ? 
         ORDER BY name 
         LIMIT ? OFFSET ?"
    )
    .bind(search_pattern)
    .bind(limit as i64)
    .bind(offset)
    .fetch_all(pool)
    .await
}

async fn count_students(
    pool: &SqlitePool,
    search: &str,
) -> Result<i64, sqlx::Error> {
    let search_pattern = format!("%{}%", search);
    
    let (count,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM students WHERE name LIKE ?"
    )
    .bind(search_pattern)
    .fetch_one(pool)
    .await?;
    
    Ok(count)
}

// ============================================================================
// BUSINESS LOGIC LAYER
// ============================================================================

pub struct StudentListResult {
    pub students: Vec<Student>,
    pub total: i64,
    pub page: usize,
    pub total_pages: usize,
}

pub async fn list_students(
    pool: &SqlitePool,
    search: &str,
    page: usize,
) -> Result<StudentListResult, sqlx::Error> {
    let students = fetch_students(pool, search, page).await?;
    let total = count_students(pool, search).await?;
    let total_pages = ((total as f64) / 20.0).ceil() as usize;
    
    Ok(StudentListResult {
        students,
        total,
        page,
        total_pages,
    })
}

// ============================================================================
// TEMPLATES (Maud HTML)
// ============================================================================

fn render_student_list(result: &StudentListResult, search: &str) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                title { "Student List" }
                style {
                    r#"
                    body {
                        font-family: Arial, sans-serif;
                        max-width: 1200px;
                        margin: 20px auto;
                        padding: 20px;
                    }
                    .search-box {
                        margin-bottom: 20px;
                    }
                    .search-box input {
                        padding: 8px;
                        width: 300px;
                        border: 1px solid #ddd;
                        border-radius: 4px;
                    }
                    .search-box button {
                        padding: 8px 16px;
                        background: #007bff;
                        color: white;
                        border: none;
                        border-radius: 4px;
                        cursor: pointer;
                    }
                    table {
                        width: 100%;
                        border-collapse: collapse;
                        margin-top: 20px;
                    }
                    th, td {
                        padding: 12px;
                        text-align: left;
                        border-bottom: 1px solid #ddd;
                    }
                    th {
                        background-color: #f8f9fa;
                        font-weight: bold;
                    }
                    tr:hover {
                        background-color: #f8f9fa;
                    }
                    .pagination {
                        margin-top: 20px;
                        display: flex;
                        gap: 10px;
                    }
                    .pagination a {
                        padding: 8px 12px;
                        border: 1px solid #ddd;
                        border-radius: 4px;
                        text-decoration: none;
                        color: #007bff;
                    }
                    .pagination a.active {
                        background: #007bff;
                        color: white;
                    }
                    "#
                }
            }
            body {
                h1 { "Student Management" }
                
                // Search form
                form.search-box method="get" action="/students" {
                    input type="text" name="search" placeholder="Search students..." value=(search);
                    button type="submit" { "Search" }
                }
                
                // Results summary
                p {
                    "Showing " 
                    strong { (result.students.len()) } 
                    " of " 
                    strong { (result.total) } 
                    " students"
                }
                
                // Student table
                table {
                    thead {
                        tr {
                            th { "ID" }
                            th { "Name" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        @for student in &result.students {
                            tr {
                                td { (student.id) }
                                td { (student.name) }
                                td {
                                    a href={ "/student/" (student.id) } { "View" }
                                    " | "
                                    a href={ "/student/" (student.id) "/edit" } { "Edit" }
                                }
                            }
                        }
                        @if result.students.is_empty() {
                            tr {
                                td colspan="3" style="text-align: center; padding: 40px;" {
                                    "No students found"
                                }
                            }
                        }
                    }
                }
                
                // Pagination
                @if result.total_pages > 1 {
                    div.pagination {
                        @for p in 0..result.total_pages {
                            a 
                                href={ "/students?search=" (search) "&page=" (p) }
                                class=@if p == result.page { "active" } else { "" }
                            {
                                (p + 1)
                            }
                        }
                    }
                }
            }
        }
    }
}

fn render_error() -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                title { "Error" }
            }
            body {
                h1 { "Error" }
                p { "Failed to load students. Please try again." }
                a href="/students" { "← Back to Students" }
            }
        }
    }
}

// ============================================================================
// HTTP HANDLERS
// ============================================================================

/// HTML handler - Returns rendered list page
async fn list_students_html_handler(
    Extension(pool): Extension<SqlitePool>,
    Query(query): Query<ListQuery>,
) -> impl IntoResponse {
    match list_students(&pool, &query.search, query.page).await {
        Ok(result) => Html(render_student_list(&result, &query.search).into_string()),
        Err(_) => Html(render_error().into_string()),
    }
}

/// JSON API handler - Returns student list as JSON
async fn list_students_json_handler(
    Extension(pool): Extension<SqlitePool>,
    Query(query): Query<ListQuery>,
) -> impl IntoResponse {
    match list_students(&pool, &query.search, query.page).await {
        Ok(result) => Ok(Json(result.students)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// ============================================================================
// ROUTES
// ============================================================================

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/students", get(list_students_html_handler))
        .route("/api/students", get(list_students_json_handler))
}
```

## Update: `crates/student/src/lib.rs`

Add the new module and route:

```rust
mod shared;
mod view_student;
mod list_students;  // <- ADD THIS

use axum::Router;
use std::sync::Arc;
use ::shared::TenantDatabaseManager;

#[derive(Clone)]
pub struct AppState {
    pub db_manager: Arc<TenantDatabaseManager>,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .merge(view_student::routes())
        .merge(list_students::routes())  // <- ADD THIS
}

pub mod middleware {
    // ... existing middleware code ...
}
```

## That's It! 🎉

You now have:
- ✅ Search functionality
- ✅ Pagination
- ✅ HTML and JSON endpoints
- ✅ All code in one logical file
- ✅ Easy for AI to understand and modify

## Testing

```bash
cargo run -p server

# Then visit:
# http://localhost:3000/students
# http://localhost:3000/students?search=vishal
# http://localhost:3000/students?page=1
# http://localhost:3000/api/students (JSON)
```

## Key Takeaways

1. **One file** contains complete feature
2. **Templates inline** for better AI context
3. **Clear sections** with comment headers
4. **Both HTML and JSON** handlers in same file
5. **Easy to extend** - AI knows exactly where to add code

This is the pattern to follow for ALL features!
