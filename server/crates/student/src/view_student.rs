// Feature: View Student Profile
// This file contains EVERYTHING needed for viewing a single student:
// - Models/DTOs
// - Database queries
// - Business logic
// - HTTP handlers (HTML + JSON)
// - Templates (Maud)
// - Routes

use axum::{
    extract::{Extension, Path},
    response::{Html, IntoResponse, Json},
    routing::get,
    Router,
};
use maud::{html, Markup, DOCTYPE};
use sqlx::SqlitePool;

use super::shared::{db, Student};
use super::AppState;

// ============================================================================
// Database Layer
// ============================================================================

async fn fetch_student(pool: &SqlitePool, id: i64) -> Result<Option<Student>, sqlx::Error> {
    // Reuse shared query
    db::get_student_by_id(pool, id).await
}

// ============================================================================
// Business Logic Layer
// ============================================================================

pub async fn get_student(pool: &SqlitePool, id: i64) -> Result<Option<Student>, sqlx::Error> {
    // For now, simple pass-through
    // In the future, add business rules, validation, etc.
    fetch_student(pool, id).await
}

// ============================================================================
// Templates (Maud HTML)
// ============================================================================
// Keep templates in the same file for AI-friendly context
// AI can see request → logic → template in one place

fn render_student_profile(student: &Student) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { "Student Profile - " (student.name) }
                
                // Bootstrap CSS
                link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/css/bootstrap.min.css" 
                     rel="stylesheet" 
                     integrity="sha384-T3c6CoIi6uLrA9TneNEoa7RxnatzjcDSCmG1MXxSR1GAsXEV/Dwwykc2MPK8M2HN" 
                     crossorigin="anonymous";
            }
            body {
                div.container.mt-5 {
                    div.row {
                        div.col-md-8.offset-md-2 {
                            div.card {
                                div.card-header.bg-primary.text-white {
                                    h2.mb-0 { "Student Profile" }
                                }
                                div.card-body {
                                    div.row.mb-3 {
                                        div.col-sm-3 {
                                            strong { "Student ID:" }
                                        }
                                        div.col-sm-9 {
                                            (student.id)
                                        }
                                    }
                                    div.row.mb-3 {
                                        div.col-sm-3 {
                                            strong { "Name:" }
                                        }
                                        div.col-sm-9 {
                                            (student.name)
                                        }
                                    }
                                }
                                div.card-footer {
                                    a.btn.btn-secondary href="/students" { "← Back to Students" }
                                    a.btn.btn-primary.ms-2 href={ "/student/" (student.id) "/edit" } { "Edit Student" }
                                }
                            }
                        }
                    }
                }
                
                // Bootstrap JS (optional, for interactive components)
                script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/js/bootstrap.bundle.min.js" 
                       integrity="sha384-C6RzsynM9kWDrMNeT87bh95OGNyZPhcTNXj1NW7RuBCsyN/o0jlpcV8Qyq46cDfL" 
                       crossorigin="anonymous" {}
            }
        }
    }
}

fn render_not_found() -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { "Student Not Found" }
                
                // Bootstrap CSS
                link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/css/bootstrap.min.css" 
                     rel="stylesheet" 
                     integrity="sha384-T3c6CoIi6uLrA9TneNEoa7RxnatzjcDSCmG1MXxSR1GAsXEV/Dwwykc2MPK8M2HN" 
                     crossorigin="anonymous";
            }
            body {
                div.container.mt-5 {
                    div.row {
                        div.col-md-8.offset-md-2 {
                            div.alert.alert-warning role="alert" {
                                h4.alert-heading { "Student Not Found" }
                                p { "The requested student does not exist in the database." }
                                hr;
                                a.btn.btn-primary href="/students" { "← Back to Students" }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ============================================================================
// HTTP Handlers
// ============================================================================

/// HTML handler - Returns rendered HTML page
async fn view_student_html_handler(
    Extension(pool): Extension<SqlitePool>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    match get_student(&pool, id).await {
        Ok(Some(student)) => Html(render_student_profile(&student).into_string()),
        Ok(None) => Html(render_not_found().into_string()),
        Err(_) => Html("<h1>Error loading student</h1>".to_string()),
    }
}

/// JSON API handler - Returns JSON for API consumers
async fn view_student_json_handler(
    Extension(pool): Extension<SqlitePool>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    match get_student(&pool, id).await {
        Ok(Some(student)) => Ok(Json(student)),
        Ok(None) => Err(axum::http::StatusCode::NOT_FOUND),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// ============================================================================
// Routes
// ============================================================================

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/student/:id", get(view_student_html_handler))
        .route("/api/student/:id", get(view_student_json_handler))
}
