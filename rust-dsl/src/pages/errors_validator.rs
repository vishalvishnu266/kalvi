//! `/dsl/errors/validator` — **showcase for the validator-crate pattern**.
//!
//! Renders the same UX as `/dsl/errors/roundtrip` but the handler side uses
//! the `validator` crate + the shared `banner_from_errors` /
//! `validate_and_render` adapters in `src/validation.rs` instead of hand-
//! coded `if` branches.
//!
//! The DSL builder here is deliberately generic — it accepts:
//!
//!   * `values`: an `Option<...>` per field so we can echo them back on 422,
//!   * `errors`: an `Option<String>` per field for `.maybe_error(...)`,
//!   * `banner`: an optional `FormBanner` from `banner_from_errors(...)`.
//!
//! Because the page has zero knowledge of the `validator` crate, `lit-ui`
//! stays dependency-free.

use crate::prelude::*;

/// Field values echoed back after a failed submit. Kept as `Option`s so the
/// GET handler can pass `&Values::default()` without special-casing.
#[derive(Default)]
pub struct Values<'a> {
    pub name:    Option<&'a str>,
    pub email:   Option<&'a str>,
    pub g_email: Option<&'a str>,
    pub age:     Option<&'a str>,
    pub website: Option<&'a str>,
    pub consent: bool,
}

/// Per-field error messages (populated by the handler from `ValidationErrors`).
#[derive(Default)]
pub struct Errors<'a> {
    pub name:    Option<&'a str>,
    pub email:   Option<&'a str>,
    pub g_email: Option<&'a str>,
    pub age:     Option<&'a str>,
    pub website: Option<&'a str>,
    pub consent: Option<&'a str>,
}

pub fn build(
    values: &Values,
    errors: &Errors,
    banner: Option<FormBanner>,
    saved: bool,
) -> Page {
    // ── Assemble the top-of-form banner ──
    //   * saved=true → green success
    //   * banner=Some → whatever the handler passed (usually from banner_from_errors)
    //   * else → nothing
    let top_banner: Option<FormBanner> = if saved {
        Some(form_banner().tone(Tone::Success).dismissible()
            .title("Student saved")
            .message("Your submission passed every validator rule — this would have been persisted in a real handler."))
    } else {
        banner
    };

    let f = form().action("/dsl/errors/validator").method("post")
        .maybe_banner(top_banner)
        .add(input().label("Full name").name("name").required()
             .value(values.name.unwrap_or(""))
             .maybe_error(errors.name))
        .add(input().label("Student email").name("email").required()
             .kind(InputType::Email)
             .value(values.email.unwrap_or(""))
             .icon_leading_svg(r#"<path d="M4 4h16a2 2 0 0 1 2 2v12a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2z"/><polyline points="22 6 12 13 2 6"/>"#)
             .maybe_error(errors.email))
        .add(input().label("Guardian email").name("g_email").required()
             .kind(InputType::Email)
             .value(values.g_email.unwrap_or(""))
             .maybe_error(errors.g_email))
        .add(input().label("Age").name("age").required()
             .kind(InputType::Number)
             .value(values.age.unwrap_or(""))
             .maybe_error(errors.age))
        .add(input().label("Website (optional)").name("website")
             .placeholder("https://example.com")
             .value(values.website.unwrap_or(""))
             .maybe_error(errors.website))
        .add({
            let cb = checkbox("I consent to the school's data policy")
                .name("consent").value("on")
                .maybe_error(errors.consent);
            if values.consent { cb.checked() } else { cb }
        })
        .save_cancel("Save student");

    // ── Compose the page ──
    let mut body = page_shell()
        .add(toolbar()
            .add(breadcrumb()
                .item(Crumb::link("DSL", "/dsl"))
                .item(Crumb::link("Error UX", "/dsl/errors"))
                .item(Crumb::current("Validator crate")))
            .add(spacer())
            .add(button().label("Hand-rolled round-trip").variant(Variant::Secondary).icon(Icons::INFO))
            .add(button().label("Handbook").variant(Variant::Secondary).icon(Icons::GRID)));

    body = body.add(card().add(Node::raw(
        "<p style=\"margin:0 0 8px\">This is the <strong>same UX</strong> as \
         <code>/dsl/errors/roundtrip</code>, but the server-side validation \
         is <strong>declarative</strong> via the <code>validator</code> crate. \
         The handler is 6 lines total — no hand-written <code>if</code> \
         branches, no per-field error message strings duplicated in the \
         handler code.</p>\
         <p style=\"margin:0;color:var(--color-text-muted);font-size:var(--fs-sm)\">\
         Try: leave fields empty (attribute-level rules fire). \
         Same email for student &amp; guardian (schema-level rule fires). \
         Age &lt; 3 or &gt; 120 (range rule fires). \
         Invalid website URL (URL rule fires). \
         Everything valid → green success banner via <code>?ok=1</code>.</p>",
    )));

    body = body.add(card().add(f));

    body = body.add(card().add(Node::raw(
        "<h3 style=\"margin:0 0 8px\">The struct definition (declarative rules)</h3>\
         <pre style=\"margin:0 0 16px;padding:12px 14px;background:var(--color-surface);\
                     border:1px solid var(--color-border);border-radius:8px;\
                     font-family:ui-monospace,SFMono-Regular,Menlo,monospace;\
                     font-size:12px;line-height:1.55;overflow:auto\"><code>\
use validator::Validate;\
\n\
\n#[derive(Deserialize, Validate)]\
\n#[validate(schema(function = \"student_and_guardian_email_differ\"))]\
\npub struct NewStudent {\
\n    #[validate(length(min = 2, message = \"Full name must be at least 2 characters\"))]\
\n    pub name: String,\
\n\
\n    #[validate(email(message = \"Student email must be a valid email\"))]\
\n    pub email: String,\
\n\
\n    #[validate(email(message = \"Guardian email must be a valid email\"))]\
\n    pub g_email: String,\
\n\
\n    #[validate(range(min = 3, max = 120, message = \"Age must be 3\u{2013}120\"))]\
\n    pub age: u8,\
\n\
\n    #[validate(url(message = \"Website must be a valid URL\"))]\
\n    pub website: Option&lt;String&gt;,\
\n\
\n    #[validate(custom(function = \"must_be_true\", message = \"You must consent\"))]\
\n    pub consent: bool,\
\n}\
\n</code></pre>\
\n<h3 style=\"margin:0 0 8px\">The handler (6 lines)</h3>\
\n<pre style=\"margin:0;padding:12px 14px;background:var(--color-surface);\
\n            border:1px solid var(--color-border);border-radius:8px;\
\n            font-family:ui-monospace,SFMono-Regular,Menlo,monospace;\
\n            font-size:12px;line-height:1.55;overflow:auto\"><code>\
pub async fn post(Form(input): Form&lt;NewStudent&gt;) -&gt; Response {\
\n    if let Some(resp) = validate_and_render(&amp;input, |i, e| {\
\n        errors_validator::build(&amp;values_from(i), &amp;errors_from(e), banner_from_errors(e), false)\
\n    }) { return resp; }\
\n    // save(&amp;input).await;\
\n    Redirect::to(\"/dsl/errors/validator?ok=1\").into_response()\
\n}\
\n</code></pre>\
\n<p style=\"margin:12px 0 0;color:var(--color-text-muted);font-size:var(--fs-sm)\">\
Compared to the hand-rolled <code>/dsl/errors/roundtrip</code>: the handler \
went from ~30 lines to 6, and every rule now lives on the struct in one \
place \u{2014} one obvious source of truth, i18n-ready via validator's message \
system, unit-testable independent of the handler.</p>",
    )));

    page_of("Validator crate · error UX", body)
}
