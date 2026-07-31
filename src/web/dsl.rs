//! `/dsl/*` HTML routes rendered by the `lit-ui` Rust DSL.
//!
//! Each handler is a one-liner: it calls a `lit_ui::pages::*::build(...)`
//! function with **mock data** (no services / DB), renders the resulting
//! [`lit_ui::components::page::Page`] into a `String`, and returns it as
//! `text/html`.
//!
//! Wired in `crate::http::routes::build_router` under:
//!
//! * `GET /dsl`            → index of demo pages
//! * `GET /dsl/students`   → students list + add-form
//! * `GET /dsl/fees`       → invoices + create-form
//! * `GET /dsl/attendance` → daily register + notes drawer
//! * `GET /dsl/dashboard`  → KPIs + admissions kanban + activity timeline
//!
//! The pages link to `/lit-components/...` for their CSS and JS, so the
//! router also needs to serve that folder — see [`crate::web::assets`]'s
//! `serve_lit_components` handler.

use axum::response::Html;
use lit_ui::core::Component;
use lit_ui::pages::{attendance, dashboard, fees, students};

/// Simple index page linking to each DSL demo — pure HTML string, no DSL.
pub async fn index() -> Html<&'static str> {
    Html(
        r##"<!doctype html>
<html>
  <head>
    <meta charset="utf-8">
    <title>DSL demos · school_erp</title>
    <link rel="stylesheet" href="/lit-components/assets/tokens.css">
    <link rel="stylesheet" href="/lit-components/assets/global.css">
    <style>
      body { font-family: var(--font-sans); background: var(--color-bg); color: var(--color-text); }
      main { max-width: 640px; margin: 48px auto; padding: 0 24px; }
      h1 { font-size: 1.75rem; margin: 0 0 8px; letter-spacing: -.02em; }
      p  { color: var(--color-text-muted); margin: 0 0 32px; }
      ul { list-style: none; padding: 0; margin: 0; display: grid; gap: 12px; }
      li a {
        display: flex; align-items: center; gap: 12px;
        padding: 16px 20px;
        background: var(--color-surface);
        border: 1px solid var(--color-border);
        border-radius: 12px;
        color: var(--color-text); text-decoration: none;
        transition: border-color .15s;
      }
      li a:hover { border-color: var(--color-primary); }
      li strong { font-weight: 600; }
      li small { color: var(--color-text-muted); font-size: .8rem; }
    </style>
  </head>
  <body>
    <main>
      <h1>lit-ui · Rust DSL demos</h1>
      <p>Every page below is rendered from typed Rust builders that emit the Lit web components. Mock data — no database calls.</p>
      <ul>
        <li><a href="/dsl/dashboard"><div><strong>Dashboard</strong><br><small>KPIs, admissions kanban, activity timeline</small></div></a></li>
        <li><a href="/dsl/students"><div><strong>Students</strong><br><small>Sortable/filterable table + add-student form</small></div></a></li>
        <li><a href="/dsl/fees"><div><strong>Fees</strong><br><small>Invoice list with statuses + create form</small></div></a></li>
        <li><a href="/dsl/attendance"><div><strong>Attendance</strong><br><small>Daily register with P/L/A segmented control per student</small></div></a></li>
      </ul>
    </main>
  </body>
</html>
"##,
    )
}

pub async fn students_page() -> Html<String> {
    let rows = students::mock_students();
    Html(students::build(&rows).render())
}

pub async fn fees_page() -> Html<String> {
    let rows = fees::mock_invoices();
    Html(fees::build(&rows).render())
}

pub async fn attendance_page() -> Html<String> {
    let rows = attendance::mock_class();
    Html(attendance::build("Grade 5-B", "2026-08-14", &rows).render())
}

pub async fn dashboard_page() -> Html<String> {
    Html(dashboard::build().render())
}
