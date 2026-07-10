// Feature: Edit Student with Hotwire Turbo
// This demonstrates:
// 1. Turbo Frames for inline editing
// 2. Turbo Streams for dynamic updates
// 3. Progressive enhancement (works without JavaScript)

use axum::{
    Form, Router,
    extract::{Extension, Path},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
};
use maud::{DOCTYPE, Markup, PreEscaped, html};
use serde::Deserialize;
use sqlx::SqlitePool;

use super::AppState;
use super::shared::{Student, db};

// ============================================================================
// Models & DTOs
// ============================================================================

#[derive(Deserialize, Debug)]
pub struct UpdateStudentForm {
    name: String,
}

// ============================================================================
// Database Layer
// ============================================================================

async fn update_student_name(
    pool: &SqlitePool,
    id: i64,
    name: String,
) -> Result<Student, sqlx::Error> {
    sqlx::query_as::<_, Student>("UPDATE students SET name = ? WHERE id = ? RETURNING id, name")
        .bind(&name)
        .bind(id)
        .fetch_one(pool)
        .await
}

// ============================================================================
// Templates (Maud HTML)
// ============================================================================

/// Full page with Turbo Frame demonstration
fn render_demo_page(student: &Student) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { "Hotwire Turbo Demo - " (student.name) }

                // Bootstrap CSS
                link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/css/bootstrap.min.css"
                     rel="stylesheet"
                     integrity="sha384-T3c6CoIi6uLrA9TneNEoa7RxnatzjcDSCmG1MXxSR1GAsXEV/Dwwykc2MPK8M2HN"
                     crossorigin="anonymous";

                // Hotwire Turbo - This enables all the magic!
                // Using jsdelivr CDN (same as working Node.js example)
                script type="module" {
                    (PreEscaped("import * as Turbo from 'https://cdn.jsdelivr.net/npm/@hotwired/turbo@8.0.4/+esm';"))
                }

                style {
                    (PreEscaped(r#"
                        .turbo-frame-border {
                            border: 2px dashed #0d6efd;
                            padding: 1rem;
                            margin: 1rem 0;
                            border-radius: 0.5rem;
                            background-color: #f8f9fa;
                        }
                        .demo-label {
                            background-color: #0d6efd;
                            color: white;
                            padding: 0.25rem 0.5rem;
                            border-radius: 0.25rem;
                            font-size: 0.875rem;
                            font-weight: bold;
                            margin-bottom: 0.5rem;
                            display: inline-block;
                        }
                        .update-count {
                            background-color: #198754;
                            color: white;
                            padding: 0.5rem 1rem;
                            border-radius: 0.25rem;
                            margin-top: 1rem;
                            display: inline-block;
                        }
                    "#))
                }
            }
            body {
                nav.navbar.navbar-expand-lg.navbar-dark.bg-primary {
                    div.container-fluid {
                        a.navbar-brand href="/" { "🚀 Hotwire Turbo Demo" }
                        div.navbar-text.text-white {
                            "Watch the page update WITHOUT full reload!"
                        }
                    }
                }

                div.container.mt-5 {
                    // Instructions
                    div.alert.alert-info {
                        h4.alert-heading { "🎯 Hotwire Turbo Demo" }
                        p { "This page demonstrates Turbo Frames and Turbo Streams:" }
                        ul {
                            li { strong { "Turbo Frame: " } "Click 'Edit' to replace just the profile section (no page reload!)" }
                            li { strong { "Turbo Stream: " } "Save changes and watch multiple parts update independently" }
                            li { strong { "Progressive Enhancement: " } "Disable JavaScript - it still works with full page loads" }
                        }
                        hr;
                        p.mb-0 {
                            "Open browser DevTools Network tab to see: "
                            strong { "NO full page reloads!" }
                        }
                    }

                    div.row {
                        // Left column: Student Profile (Turbo Frame)
                        div.col-md-6 {
                            div.turbo-frame-border {
                                span.demo-label { "📦 TURBO FRAME #1" }
                                p.small.text-muted { "This frame updates independently when you click Edit/Save" }

                                // This is the Turbo Frame that will be replaced
                                (render_student_profile_frame(student))
                            }
                        }

                        // Right column: Update counter (updates via Turbo Stream)
                        div.col-md-6 {
                            div.turbo-frame-border {
                                span.demo-label { "🔄 TURBO STREAM TARGET" }
                                p.small.text-muted { "This updates via Turbo Stream when you save" }

                                div id="update-counter" {
                                    div.update-count {
                                        "✅ Ready to update"
                                    }
                                }
                            }

                            div.turbo-frame-border.mt-4 {
                                span.demo-label { "📊 ANOTHER STREAM TARGET" }
                                p.small.text-muted { "This also updates independently" }

                                div id="last-modified" {
                                    div.alert.alert-secondary {
                                        "⏰ Not modified yet"
                                    }
                                }
                            }
                        }
                    }

                    // How it works explanation
                    div.mt-5 {
                        div.card {
                            div.card-header.bg-success.text-white {
                                h5.mb-0 { "💡 How This Works" }
                            }
                            div.card-body {
                                h6 { "1. Turbo Frames (Partial Updates)" }
                                p.small {
                                    "The "
                                    code { "turbo-frame" }
                                    " tag tells Turbo to only replace that section. When you click Edit, "
                                    "the server returns just the edit form wrapped in the same frame ID."
                                }

                                h6.mt-3 { "2. Turbo Streams (Multiple Updates)" }
                                p.small {
                                    "When you save, the server returns a "
                                    code { "turbo-stream" }
                                    " response that updates multiple parts of the page at once!"
                                }

                                h6.mt-3 { "3. No JavaScript Required (Progressive Enhancement)" }
                                p.small {
                                    "If Turbo isn't loaded, forms submit normally with full page reloads. "
                                    "The app works either way!"
                                }
                            }
                        }
                    }
                }

                // Bootstrap JS
                script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/js/bootstrap.bundle.min.js"
                       integrity="sha384-C6RzsynM9kWDrMNeT87bh95OGNyZPhcTNXj1NW7RuBCsyN/o0jlpcV8Qyq46cDfL"
                       crossorigin="anonymous" {}
            }
        }
    }
}

/// Turbo Frame: Display mode (read-only)
fn render_student_profile_frame(student: &Student) -> Markup {
    html! {
        // This turbo-frame wraps the student profile
        // When you click "Edit", this entire frame gets replaced
        turbo-frame id="student_profile" {
            div.card {
                div.card-header.bg-primary.text-white {
                    h3.mb-0 { "Student Profile" }
                }
                div.card-body {
                    div.row.mb-3 {
                        div.col-sm-4 {
                            strong { "Student ID:" }
                        }
                        div.col-sm-8 {
                            (student.id)
                        }
                    }
                    div.row.mb-3 {
                        div.col-sm-4 {
                            strong { "Name:" }
                        }
                        div.col-sm-8 {
                            span.fs-4 { (student.name) }
                        }
                    }
                }
                div.card-footer {
                    // This link triggers a Turbo Frame navigation
                    // Only the student_profile frame will be replaced!
                    a.btn.btn-warning href={"/turbo-demo/student/" (student.id) "/edit"} {
                        "✏️ Edit (Watch Frame Update!)"
                    }
                    a.btn.btn-secondary.ms-2 href="/students" { "Back to List" }
                }
            }
        }
    }
}

/// Turbo Frame: Edit mode (form)
fn render_student_edit_frame(student: &Student) -> Markup {
    html! {
        // Same turbo-frame ID! This replaces the profile above
        turbo-frame id="student_profile" {
            div.card {
                div.card-header.bg-warning {
                    h3.mb-0 { "✏️ Edit Student" }
                }
                div.card-body {
                    // Form submits to update endpoint
                    form method="post" action={"/turbo-demo/student/" (student.id)} {
                        div.mb-3 {
                            label.form-label for="name" { "Student Name:" }
                            input.form-control #name type="text" name="name"
                                  value=(student.name) required;
                        }

                        div.alert.alert-info {
                            strong { "💡 Pro tip: " }
                            "Change the name and click Save. Watch how ONLY the relevant parts update!"
                        }

                        div.d-flex.gap-2 {
                            button.btn.btn-success type="submit" {
                                "💾 Save (Triggers Turbo Stream!)"
                            }
                            // Cancel button navigates back within the frame
                            a.btn.btn-secondary href={"/turbo-demo/student/" (student.id)} {
                                "Cancel"
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Turbo Stream response - updates multiple targets at once!
fn render_turbo_stream_update(student: &Student, update_number: i64) -> Markup {
    html! {
        // Action: replace the student_profile frame with updated view
        turbo-stream action="replace" target="student_profile" {
            template {
                (render_student_profile_frame(student))
            }
        }

        // Action: update the counter
        turbo-stream action="update" target="update-counter" {
            template {
                div.update-count {
                    "✅ Updated #" (update_number) " - Name: " strong { (student.name) }
                }
            }
        }

        // Action: update the last modified time
        turbo-stream action="update" target="last-modified" {
            template {
                div.alert.alert-success {
                    "⏰ Last modified: Just now!"
                    br;
                    "Student: " strong { (student.name) }
                }
            }
        }
    }
}

// ============================================================================
// HTTP Handlers
// ============================================================================

/// Show the demo page (initial load)
async fn show_demo_page(
    Extension(pool): Extension<SqlitePool>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    match db::get_student_by_id(&pool, id).await {
        Ok(Some(student)) => Html(render_demo_page(&student).into_string()),
        Ok(None) => Html("<h1>Student not found</h1>".to_string()),
        Err(_) => Html("<h1>Database error</h1>".to_string()),
    }
}

/// Show just the student profile frame (for Turbo Frame navigation)
async fn show_student_frame(
    Extension(pool): Extension<SqlitePool>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    match db::get_student_by_id(&pool, id).await {
        Ok(Some(student)) => Html(render_student_profile_frame(&student).into_string()),
        Ok(None) => Html("<h1>Student not found</h1>".to_string()),
        Err(_) => Html("<h1>Database error</h1>".to_string()),
    }
}

/// Show edit form (Turbo Frame)
async fn edit_student_frame(
    Extension(pool): Extension<SqlitePool>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    match db::get_student_by_id(&pool, id).await {
        Ok(Some(student)) => Html(render_student_edit_frame(&student).into_string()),
        Ok(None) => Html("<h1>Student not found</h1>".to_string()),
        Err(_) => Html("<h1>Database error</h1>".to_string()),
    }
}

/// Update student - returns Turbo Stream!
async fn update_student(
    Extension(pool): Extension<SqlitePool>,
    Path(id): Path<i64>,
    Form(form): Form<UpdateStudentForm>,
) -> Response {
    match update_student_name(&pool, id, form.name).await {
        Ok(student) => {
            // Simulate update counter (in real app, this would come from database)
            use std::time::{SystemTime, UNIX_EPOCH};
            let update_number = (SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                % 1000) as i64;

            // Return Turbo Stream response
            // This updates multiple parts of the page!
            (
                StatusCode::OK,
                [("Content-Type", "text/vnd.turbo-stream.html")],
                Html(render_turbo_stream_update(&student, update_number).into_string()),
            )
                .into_response()
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Html("<h1>Failed to update student</h1>".to_string()),
        )
            .into_response(),
    }
}

// ============================================================================
// Routes
// ============================================================================

pub fn routes() -> Router<AppState> {
    Router::new()
        // Demo page
        .route("/turbo-demo/student/{id}", get(show_demo_page))
        // Turbo Frame endpoints
        .route("/turbo-demo/student/{id}/frame", get(show_student_frame))
        .route("/turbo-demo/student/{id}/edit", get(edit_student_frame))
        // Update endpoint (returns Turbo Stream)
        .route("/turbo-demo/student/{id}", post(update_student))
}
