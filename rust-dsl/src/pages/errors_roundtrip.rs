//! `/dsl/errors/roundtrip` — **end-to-end server-render error flow**.
//!
//! Two things live behind one URL:
//!
//!  * `GET  /dsl/errors/roundtrip` → renders the empty form.
//!  * `POST /dsl/errors/roundtrip` → validates the submission and re-renders
//!    the same form with errors annotated. Returns HTTP **422 Unprocessable
//!    Entity**; the shell's document-level submit interceptor swaps the
//!    target region — no per-page JS, no client-side validation library.
//!
//! Successful submits redirect to the same URL with `?ok=1` in the query
//! string; the page then shows a green success banner so you can see the
//! happy path without leaving the demo.
//!
//! This is the **reference implementation** of the pattern documented in
//! `GUIDE.md` §8.6.
//!
//! ## Design note — validation lives in the handler, not the page
//!
//! The DSL crate `lit-ui` is intentionally dependency-free, so `validator`
//! isn't imported here. The Rust struct + `#[derive(Validate)]` + the
//! `banner_from_errors`/`validate_and_render` adapters all live in the
//! main `school_erp` crate (see `src/validation.rs` and
//! `src/web/dsl.rs::errors_roundtrip_post`). The page builder just accepts
//! an `Option<String>` per field — same shape as any other DSL page.
//!
//! Legacy `Input` + `validate` are kept here for now as the fallback for
//! the GET path; new endpoints should skip them and only pass the
//! per-field errors down.

use crate::prelude::*;

/// The submitted (or empty) form values. All fields are `Option` so the
/// GET handler can call `build(&Input::default(), ..)` without special-casing.
#[derive(Default)]
pub struct Input {
    pub name:  Option<String>,
    pub email: Option<String>,
    pub g_email: Option<String>,
    pub due:   Option<String>,
    pub consent: bool,
}

/// Result of validation. Empty maps = success.
#[derive(Default)]
pub struct Validation {
    /// Hard errors — user MUST fix these. Field name → message.
    pub errors:   Vec<(String, String)>,
    /// Soft warnings — user can proceed, but should be aware. Field name → message.
    pub warnings: Vec<(String, String)>,
}

impl Validation {
    pub fn is_ok(&self) -> bool { self.errors.is_empty() }
    pub fn has_any(&self) -> bool { !self.errors.is_empty() || !self.warnings.is_empty() }

    /// Look up an error by field name. Handy for `.maybe_error(...)`.
    pub fn error_of(&self, field: &str) -> Option<String> {
        self.errors.iter().find_map(|(k, v)| if k == field { Some(v.clone()) } else { None })
    }
}

/// Deterministic validation of an [`Input`]. Uses a handful of realistic
/// rules so you can experiment: empty fields fail, malformed emails fail,
/// matching student+guardian emails is a hard error (business rule), and
/// due dates in the past OR on Aug 15 (mock "holiday") produce warnings.
pub fn validate(input: &Input) -> Validation {
    let mut v = Validation::default();

    let name = input.name.as_deref().unwrap_or("").trim();
    if name.is_empty() {
        v.errors.push(("name".into(), "Full name is required".into()));
    } else if name.len() < 2 {
        v.errors.push(("name".into(), "Full name must be at least 2 characters".into()));
    }

    let email = input.email.as_deref().unwrap_or("").trim();
    if email.is_empty() {
        v.errors.push(("email".into(), "Student email is required".into()));
    } else if !looks_like_email(email) {
        v.errors.push(("email".into(), "Student email must be a valid email address".into()));
    }

    let g_email = input.g_email.as_deref().unwrap_or("").trim();
    if g_email.is_empty() {
        v.errors.push(("g_email".into(), "Guardian email is required".into()));
    } else if !looks_like_email(g_email) {
        v.errors.push(("g_email".into(), "Guardian email must be a valid email address".into()));
    } else if !email.is_empty() && g_email.eq_ignore_ascii_case(email) {
        // Cross-field business rule → attach to guardian field AND surface
        // in the banner so both surfaces highlight it.
        v.errors.push(("g_email".into(),
            "Guardian email must be different from the student's email".into()));
    }

    let due = input.due.as_deref().unwrap_or("").trim();
    if due.is_empty() {
        v.errors.push(("due".into(), "Please pick a due date".into()));
    } else {
        // "Past" is a hard error, "holiday" is a soft warning.
        if due < "2026-08-01" {
            v.errors.push(("due".into(), "Due date cannot be in the past".into()));
        }
        if due == "2026-08-15" {
            v.warnings.push(("due".into(),
                "Aug 15 is a public holiday — the invoice email may be delayed".into()));
        }
    }

    if !input.consent {
        v.errors.push(("consent".into(),
            "You must consent to the school's data policy".into()));
    }

    v
}

fn looks_like_email(s: &str) -> bool {
    // Deliberately minimal — full RFC 5322 is not the point of this demo.
    // Rules: exactly one '@', at least one char before it, and a '.'
    // somewhere in the domain part with at least two chars on either side
    // of that dot ("a@b.c" is the shortest string we accept). Written with
    // `saturating_sub` so short inputs (e.g. "a@") never panic in debug
    // builds via `usize` underflow.
    let bytes = s.as_bytes();
    let Some(i) = bytes.iter().position(|&c| c == b'@') else { return false };
    if i == 0 || i >= bytes.len().saturating_sub(3) { return false; }
    // Reject a second '@' — a valid address has exactly one.
    if bytes[i + 1..].contains(&b'@') { return false; }
    // Require a dot in the domain, and at least one char on each side of it.
    let domain = &s[i + 1..];
    let Some(dot) = domain.find('.') else { return false };
    dot > 0 && dot < domain.len() - 1
}

/// Build the page. `saved` = show the green "just saved" banner (used on
/// the ?ok=1 redirect). Otherwise `input` + `validation` control the field
/// values and the error/warning surfaces.
pub fn build(input: &Input, validation: &Validation, saved: bool) -> Page {
    // ── Assemble the top-of-form banner ──
    // Priority:
    //   1. saved=true                 → success banner
    //   2. any errors OR warnings     → multi-section banner via helper
    //   3. otherwise                  → no banner
    let top_banner: Option<FormBanner> = if saved {
        Some(form_banner().tone(Tone::Success).dismissible()
            .title("Student saved")
            .message("Your submission passed validation and would have been persisted in a real handler."))
    } else {
        errors_and_warnings_banner(
            validation.errors.iter().cloned().map(FieldError::from),
            validation.warnings.iter().cloned().map(FieldError::from),
        )
    };

    // ── Compose the form ──
    let f = form().action("/dsl/errors/roundtrip").method("post")
        .maybe_banner(top_banner)
        .add(input_field("name",    "Full name",       input.name.as_deref(),    validation.error_of("name")))
        .add(input_field("email",   "Student email",   input.email.as_deref(),   validation.error_of("email"))
             .kind(InputType::Email))
        .add(input_field("g_email", "Guardian email",  input.g_email.as_deref(), validation.error_of("g_email"))
             .kind(InputType::Email))
        .add(datepicker().label("Due date").name("due")
             .value(input.due.as_deref().unwrap_or(""))
             .maybe_error(validation.error_of("due").as_deref()))
        .add(checkbox("I consent to the school's data policy")
             .name("consent").value("on")
             .maybe_error(validation.error_of("consent").as_deref())
             .set_checked_if(input.consent))
        .save_cancel("Save student");

    // ── Compose the page ──
    let mut body = page_shell()
        .add(toolbar()
            .add(breadcrumb()
                .item(Crumb::link("DSL", "/dsl"))
                .item(Crumb::link("Error UX", "/dsl/errors"))
                .item(Crumb::current("Server round-trip")))
            .add(spacer())
            .add(button().label("Handbook").variant(Variant::Secondary).icon(Icons::INFO))
            .add(button().label("Combinations").variant(Variant::Secondary).icon(Icons::GRID)));

    body = body.add(card().add(Node::raw(
        "<p style=\"margin:0 0 8px\">This is a <strong>real server round-trip</strong>. \
         Fill the form and click Save — the browser POSTs to \
         <code>/dsl/errors/roundtrip</code>, Axum validates the input, and \
         returns HTTP <strong>422</strong> with a re-rendered version of \
         this page (errors annotated). The shell's submit interceptor swaps the target region \
         so you see the same URL with errors inline — no per-page JS, no XHR handler.</p>\
         <p style=\"margin:0;color:var(--color-text-muted);font-size:var(--fs-sm)\">\
         Try: leave fields empty → hard errors under each. \
         Enter the SAME email for student &amp; guardian → cross-field error + banner. \
         Pick due date <strong>2026-08-15</strong> → warning section (still saves). \
         Pick anything before <strong>2026-08-01</strong> → hard error on the date. \
         Fill everything correctly → green success banner via <code>?ok=1</code>.</p>",
    )));

    body = body.add(card().add(f));

    body = body.add(card().add(Node::raw(
        "<h3 style=\"margin:0 0 8px\">The handler</h3>\
         <pre style=\"margin:0;padding:12px 14px;background:var(--color-surface);\
                     border:1px solid var(--color-border);border-radius:8px;\
                     font-family:ui-monospace,SFMono-Regular,Menlo,monospace;\
                     font-size:12px;line-height:1.55;overflow:auto\"><code>\
// GET  /dsl/errors/roundtrip\
\npub async fn get() -&gt; Html&lt;String&gt; {\
\n    let ok = /* ?ok=1 present */;\
\n    let page = errors_roundtrip::build(\
\n        &amp;Input::default(),\
\n        &amp;Validation::default(),\
\n        ok,\
\n    );\
\n    Html(page.render())\
\n}\
\n\
\n// POST /dsl/errors/roundtrip\
\npub async fn post(Form(body): Form&lt;PostBody&gt;) -&gt; Response {\
\n    let input = Input::from(body);\
\n    let v     = errors_roundtrip::validate(&amp;input);\
\n    if v.is_ok() {\
\n        // Would persist here; then redirect (shell follows → GET 200).\
\n        return Redirect::to(\"/dsl/errors/roundtrip?ok=1\").into_response();\
\n    }\
\n    // 422 + fresh HTML. The shell swaps the target region and shows the errors.\
\n    let page = errors_roundtrip::build(&amp;input, &amp;v, false);\
\n    (StatusCode::UNPROCESSABLE_ENTITY, Html(page.render())).into_response()\
\n}\
\n</code></pre>",
    )));

    page_of("Server round-trip · error UX demo", body)
}

// ── Small helper that unifies the input-with-value+error pattern ──
fn input_field<'a>(
    name: &'a str,
    label: &'a str,
    value: Option<&'a str>,
    err: Option<String>,
) -> Input_ {
    let mut i = input().label(label).name(name).required();
    if let Some(v) = value { i = i.value(v); }
    if let Some(m) = err   { i = i.error(m); }
    i
}

// Re-export the DSL Input builder type under a local alias so it doesn't
// collide with our own `Input` struct.
use crate::components::input::Input as Input_;

// ── Local extension trait so `checkbox().set_checked_if(bool)` reads cleanly ──
pub trait CheckboxExt { fn set_checked_if(self, cond: bool) -> Self; }
impl CheckboxExt for crate::components::checkbox::Checkbox {
    fn set_checked_if(self, cond: bool) -> Self {
        if cond { self.checked() } else { self }
    }
}
