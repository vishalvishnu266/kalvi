//! Thin adapter between the [`validator`] crate and the `lit-ui` DSL's
//! `FormBanner` + per-field `.error()` machinery.
//!
//! Three pieces live here:
//!
//!   1. **[`FieldErrorLookup`]** — extension trait so any Rust page builder
//!      can call `errors.field_error("name")` to get `Option<String>` for
//!      `.maybe_error(...)`.
//!
//!   2. **[`banner_from_errors`]** — turns a [`validator::ValidationErrors`]
//!      into `Option<FormBanner>` with:
//!        * the standard "Please fix N errors" title (auto-pluralised),
//!        * one linked summary item per field error,
//!        * the struct-level `.errors()` (cross-field rules from
//!          `#[validate(schema=...)]`) rendered as the banner's message.
//!
//!   3. **[`validate_and_render`]** — the one-line Axum helper for the
//!      standard "validate → 422 with re-rendered form" round-trip. See the
//!      module docs and `GUIDE.md` §8.6 for the pattern.
//!
//! ## The full endpoint pattern
//!
//! ```ignore
//! use crate::validation::*;
//! use validator::Validate;
//!
//! #[derive(Deserialize, Validate)]
//! pub struct NewStudent {
//!     #[validate(length(min = 2, message = "Full name must be at least 2 characters"))]
//!     pub name: String,
//!     #[validate(email(message = "Guardian email must be valid"))]
//!     pub email: String,
//! }
//!
//! pub async fn create(Form(input): Form<NewStudent>) -> Response {
//!     // Phase 1 — pure/sync validation via attributes.
//!     if let Some(resp) = validate_and_render(&input, |i, e| {
//!         students::add_form(i, e)   // page builder — receives &ValidationErrors
//!     }) { return resp; }
//!
//!     // Phase 2 — async / DB business rules (optional).
//!     // let mut biz = ValidationErrors::new();
//!     // if db.email_exists(&input.email).await { … }
//!     // if !biz.is_empty() { … return 422 … }
//!
//!     save(&input).await;
//!     Redirect::to("/students?ok=1").into_response()
//! }
//! ```
//!
//! The page builder never touches validator internals — it only sees a
//! `&ValidationErrors` (empty on GET, populated on 422 re-render) and calls
//! `errors.field_error("name")` and `banner_from_errors(errors)`. That
//! keeps the DSL side (`lit-ui`) 100% dependency-free.

use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use lit_ui::core::Component;
use lit_ui::prelude::{
    banner_section, errors_and_warnings_banner, form_banner,
    FieldError, FormBanner, Tone,
};
use validator::{Validate, ValidationErrors};

// ---------------------------------------------------------------------------
// 1. Field-level lookup — used by input().maybe_error(errors.field_error("x"))
// ---------------------------------------------------------------------------

/// Extension trait — resolve the FIRST validation message for a given field
/// name, ignoring internal codes. Returns `None` when the field is clean.
///
/// Preferred over `errors.field_errors().get(name)` at every call site.
pub trait FieldErrorLookup {
    /// The message for the first error on this field, if any.
    fn field_error(&self, name: &str) -> Option<String>;

    /// Do we have ANY errors (field-level OR struct-level)?
    fn has_any(&self) -> bool;
}

impl FieldErrorLookup for ValidationErrors {
    fn field_error(&self, name: &str) -> Option<String> {
        self.field_errors()
            .get(name)
            .and_then(|v| v.first())
            .and_then(|e| e.message.as_ref().map(|c| c.to_string()))
    }

    fn has_any(&self) -> bool {
        !self.field_errors().is_empty() || !self.errors().is_empty()
    }
}

// ---------------------------------------------------------------------------
// 2. ValidationErrors → FormBanner
// ---------------------------------------------------------------------------

/// Turn a `ValidationErrors` into a top-of-form summary banner.
///
///   * Field-level errors  → one linked item per field (scroll+focus on click).
///   * Struct-level errors (`#[validate(schema = "…")]`) → rendered as the
///     banner's `message` because they aren't tied to a single field.
///   * Returns `None` when both lists are empty → callers use
///     `.maybe_banner(banner_from_errors(errors))`.
pub fn banner_from_errors(errors: &ValidationErrors) -> Option<FormBanner> {
    if !errors.has_any() { return None; }

    // Collect field errors, one entry per (field, first-message).
    let field_items: Vec<FieldError> = errors.field_errors().iter()
        .filter_map(|(name, list)| {
            let msg = list.first()?.message.as_ref()?.to_string();
            Some(FieldError { field: name.to_string(), message: msg })
        })
        .collect();

    // Struct-level (cross-field / schema) rules — surfaced as banner message
    // because they aren't tied to any single field.
    let struct_messages: Vec<String> = errors.errors().iter()
        .filter_map(|(_code, kind)| match kind {
            validator::ValidationErrorsKind::Field(list) => {
                // These are also field errors — already handled above.
                let _ = list; None
            }
            validator::ValidationErrorsKind::Struct(nested) => {
                // Nested — flatten via recursion (rare in practice).
                Some(banner_message_from_nested(nested))
            }
            validator::ValidationErrorsKind::List(_) => None,
        })
        .collect();

    // If the ONLY thing we have is struct-level messages, render a plain
    // single-section danger banner with those as the message.
    if field_items.is_empty() && !struct_messages.is_empty() {
        return Some(form_banner()
            .tone(Tone::Danger)
            .title("Please fix the errors below")
            .message(struct_messages.join(" · ")));
    }

    // Common path — multi-section banner: field errors + optional
    // schema-level warning-tone section for cross-field rules.
    let base = errors_and_warnings_banner(
        field_items,
        // No separate warnings list from validator by default; empty vec.
        Vec::<(&str, &str)>::new(),
    );
    match (base, struct_messages.is_empty()) {
        (Some(b), true)  => Some(b),
        (Some(b), false) => Some(b.add_section(
            banner_section(Tone::Danger)
                .message(struct_messages.join(" · ")))),
        (None,    false) => Some(form_banner().tone(Tone::Danger)
            .message(struct_messages.join(" · "))),
        (None,    true)  => None,
    }
}

fn banner_message_from_nested(errors: &ValidationErrors) -> String {
    // Recursive walk — flattens nested struct errors into "field: msg" lines.
    let mut out = Vec::new();
    for (name, list) in errors.field_errors() {
        if let Some(msg) = list.first().and_then(|e| e.message.as_ref()) {
            out.push(format!("{name}: {msg}"));
        }
    }
    out.join(" · ")
}

// ---------------------------------------------------------------------------
// 3. Axum helper — validate + render + 422, in one line
// ---------------------------------------------------------------------------

/// The one-line validation entry point for Axum handlers.
///
/// * `input` — the deserialised `Form<...>` body (or JSON).
/// * `render` — a page-builder closure `(input, &ValidationErrors) -> Page`.
///   Called ONLY when validation fails; the errors will already contain
///   everything from the `#[derive(Validate)]` attributes.
///
/// Returns:
/// * `Some(Response)` — an HTTP 422 with the re-rendered form. The handler
///   should just `return` it.
/// * `None` — validation passed; the handler continues to persist +
///   redirect on success.
///
/// See the module docs for the full endpoint pattern.
pub fn validate_and_render<T, F, P>(input: &T, render: F) -> Option<Response>
where
    T: Validate,
    F: FnOnce(&T, &ValidationErrors) -> P,
    P: Component,
{
    match input.validate() {
        Ok(()) => None,
        Err(errors) => {
            let page = render(input, &errors);
            Some((StatusCode::UNPROCESSABLE_ENTITY, Html(page.render())).into_response())
        }
    }
}

/// Extended variant that ALSO takes a set of already-computed async /
/// business-rule errors (e.g. "email is already registered", checked
/// against the database) and merges them with the attribute-derived ones
/// before rendering. Use this for phase-2 validation.
pub fn validate_and_render_with<T, F, P>(
    input: &T,
    extra: ValidationErrors,
    render: F,
) -> Option<Response>
where
    T: Validate,
    F: FnOnce(&T, &ValidationErrors) -> P,
    P: Component,
{
    let mut errors = match input.validate() {
        Ok(()) => ValidationErrors::new(),
        Err(e) => e,
    };
    // Merge every field error from `extra` into `errors`.
    for (field, list) in extra.field_errors() {
        for e in list {
            errors.add(field, e.clone());
        }
    }
    if !errors.has_any() { return None; }
    let page = render(input, &errors);
    Some((StatusCode::UNPROCESSABLE_ENTITY, Html(page.render())).into_response())
}
