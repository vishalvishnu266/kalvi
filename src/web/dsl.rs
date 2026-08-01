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

use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::Form;
use lit_ui::core::Component;
use lit_ui::pages::{
    attendance, components, dashboard, errors, errors_combos, errors_roundtrip, errors_validator,
    fees, icons, layouts, students,
};
use serde::Deserialize;
use validator::{Validate, ValidationError, ValidationErrors};

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
        <li><a href="/dsl/icons"><div><strong>Icons</strong><br><small>Visual catalogue of every Icons::* constant</small></div></a></li>
        <li><a href="/dsl/layouts"><div><strong>Layout guide</strong><br><small>Every layout primitive & preset with live demos + source</small></div></a></li>
        <li><a href="/dsl/components"><div><strong>Components</strong><br><small>Every UI component with variants + source (buttons, inputs, tables, modals…)</small></div></a></li>
        <li><a href="/dsl/errors"><div><strong>Error UX</strong><br><small>Field / form / page error surfaces — the handbook for validation UI</small></div></a></li>
        <li><a href="/dsl/errors/combos"><div><strong>Error combinations</strong><br><small>Every meaningful combination of banner + field + alert + ack panel with source</small></div></a></li>
        <li><a href="/dsl/errors/roundtrip"><div><strong>Error round-trip (live)</strong><br><small>Real POST → 422 → Turbo swap → errors inline. Hand-rolled validation.</small></div></a></li>
        <li><a href="/dsl/errors/validator"><div><strong>Validator crate (recommended)</strong><br><small>Same UX, declarative rules via #[derive(Validate)]. Copy this for new endpoints.</small></div></a></li>
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

pub async fn icons_page() -> Html<String> {
    Html(icons::build().render())
}

pub async fn layouts_page() -> Html<String> {
    Html(layouts::build().render())
}

pub async fn components_page() -> Html<String> {
    Html(components::build().render())
}

pub async fn errors_page() -> Html<String> {
    Html(errors::build().render())
}

pub async fn errors_combos_page() -> Html<String> {
    Html(errors_combos::build().render())
}

// ────────────────────────────────────────────────────────────────────────────
// Server round-trip demo — GET renders the empty form, POST validates it
// and returns HTTP 422 with a re-rendered version showing errors inline.
// This is the reference implementation of the pattern documented in
// GUIDE.md §8.6.
// ────────────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct RoundtripQuery {
    /// Present after a successful POST → we show the green "saved" banner.
    ok: Option<u8>,
}

#[derive(Deserialize)]
pub struct RoundtripPost {
    pub name:    Option<String>,
    pub email:   Option<String>,
    pub g_email: Option<String>,
    pub due:     Option<String>,
    /// HTML checkboxes send "on" when checked and NOTHING when unchecked,
    /// so we use `Option<String>` and treat any Some as true.
    pub consent: Option<String>,
}

pub async fn errors_roundtrip_get(Query(q): Query<RoundtripQuery>) -> Html<String> {
    let page = errors_roundtrip::build(
        &errors_roundtrip::Input::default(),
        &errors_roundtrip::Validation::default(),
        q.ok.unwrap_or(0) == 1,
    );
    Html(page.render())
}

pub async fn errors_roundtrip_post(Form(body): Form<RoundtripPost>) -> Response {
    let input = errors_roundtrip::Input {
        name:    body.name.filter(|s| !s.trim().is_empty()),
        email:   body.email.filter(|s| !s.trim().is_empty()),
        g_email: body.g_email.filter(|s| !s.trim().is_empty()),
        due:     body.due.filter(|s| !s.trim().is_empty()),
        consent: body.consent.is_some(),
    };
    let v = errors_roundtrip::validate(&input);
    if v.is_ok() {
        // A real handler would persist here. Then redirect so Turbo issues
        // a fresh GET (avoids the "back re-submits POST" problem).
        return Redirect::to("/dsl/errors/roundtrip?ok=1").into_response();
    }
    // 422 + re-rendered HTML. Turbo swaps <body>; user sees errors inline.
    let page = errors_roundtrip::build(&input, &v, false);
    (StatusCode::UNPROCESSABLE_ENTITY, Html(page.render())).into_response()
}

// ────────────────────────────────────────────────────────────────────────────
// Validator-crate demo — SAME UX as errors_roundtrip, but the handler uses
// declarative attribute rules on the request struct instead of hand-coded
// `if` branches. Serves as the reference implementation to copy for new
// endpoints. See src/validation.rs for the shared helpers.
// ────────────────────────────────────────────────────────────────────────────

/// Request struct — every rule declared inline. Compare with the manual
/// `errors_roundtrip::validate` function — this is what disappears.
#[derive(Deserialize, Validate)]
#[validate(schema(function = "check_emails_differ"))]
pub struct NewStudent {
    #[validate(length(min = 2, message = "Full name must be at least 2 characters"))]
    pub name: String,

    #[validate(email(message = "Student email must be a valid email"))]
    pub email: String,

    #[validate(email(message = "Guardian email must be a valid email"))]
    pub g_email: String,

    #[validate(range(min = 3, max = 120, message = "Age must be between 3 and 120"))]
    pub age: u8,

    // Optional — only validated when Some.
    #[validate(url(message = "Website must be a valid URL"))]
    pub website: Option<String>,

    #[validate(custom(function = "must_be_true", message = "You must consent to continue"))]
    pub consent: bool,
}

// Struct-level (schema) validator — sees the whole struct so it can compare
// fields. Errors it emits are surfaced by the banner adapter as the
// banner's `.message()` (they aren't tied to a single field).
fn check_emails_differ(s: &NewStudent) -> Result<(), ValidationError> {
    if !s.email.is_empty() && s.email.eq_ignore_ascii_case(&s.g_email) {
        return Err(ValidationError::new("emails_must_differ")
            .with_message("Guardian email must be different from the student's email".into()));
    }
    Ok(())
}

// Reusable custom validator — "boolean must be true" (e.g. consent, terms).
fn must_be_true(b: &bool) -> Result<(), ValidationError> {
    if *b { Ok(()) } else { Err(ValidationError::new("must_be_true")) }
}

/// Raw form body — separate from `NewStudent` because HTML forms send
/// checkboxes as `Some("on")` / `None` (never `false`), and we may need to
/// echo back partial input on 422.
#[derive(Deserialize, Default)]
pub struct NewStudentForm {
    pub name:    Option<String>,
    pub email:   Option<String>,
    pub g_email: Option<String>,
    pub age:     Option<String>,      // raw so an invalid "abc" doesn't 400
    pub website: Option<String>,
    pub consent: Option<String>,
}

pub async fn errors_validator_get(Query(q): Query<RoundtripQuery>) -> Html<String> {
    let page = errors_validator::build(
        &errors_validator::Values::default(),
        &errors_validator::Errors::default(),
        None,
        q.ok.unwrap_or(0) == 1,
    );
    Html(page.render())
}

pub async fn errors_validator_post(Form(body): Form<NewStudentForm>) -> Response {
    // Parse raw form → strongly-typed struct. If required strings/ints are
    // missing we still build the struct with placeholders so validator can
    // emit its own error messages (better UX than a generic 400).
    let input = NewStudent {
        name:    body.name.clone().unwrap_or_default(),
        email:   body.email.clone().unwrap_or_default(),
        g_email: body.g_email.clone().unwrap_or_default(),
        age:     body.age.as_deref().and_then(|s| s.parse().ok()).unwrap_or(0),
        website: body.website.clone().filter(|s| !s.trim().is_empty()),
        consent: body.consent.is_some(),
    };

    // The whole validation + re-render dance in ONE line via the shared helper.
    if let Some(resp) = crate::validation::validate_and_render(&input, |i, e| {
        render_validator_page(i, &body, e)
    }) {
        return resp;
    }

    // Everything valid — a real handler would persist here.
    Redirect::to("/dsl/errors/validator?ok=1").into_response()
}

// Adapter — turns the strongly-typed struct + raw body + errors into a Page.
fn render_validator_page(
    input: &NewStudent,
    raw: &NewStudentForm,
    errors: &ValidationErrors,
) -> lit_ui::components::page::Page {
    use crate::validation::{banner_from_errors, FieldErrorLookup};

    // Echo back what the user typed (even for fields that failed to parse).
    let values = errors_validator::Values {
        name:    raw.name.as_deref(),
        email:   raw.email.as_deref(),
        g_email: raw.g_email.as_deref(),
        age:     raw.age.as_deref(),
        website: raw.website.as_deref(),
        consent: input.consent,
    };

    // Per-field errors — one shared lookup call per field, no manual glue.
    let name_err    = errors.field_error("name");
    let email_err   = errors.field_error("email");
    let g_email_err = errors.field_error("g_email");
    let age_err     = errors.field_error("age");
    let website_err = errors.field_error("website");
    let consent_err = errors.field_error("consent");

    let field_errors = errors_validator::Errors {
        name:    name_err.as_deref(),
        email:   email_err.as_deref(),
        g_email: g_email_err.as_deref(),
        age:     age_err.as_deref(),
        website: website_err.as_deref(),
        consent: consent_err.as_deref(),
    };

    errors_validator::build(&values, &field_errors, banner_from_errors(errors), false)
}
