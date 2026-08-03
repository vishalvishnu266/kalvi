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

use std::any::Any;
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
        .add(form_section("testing")
            .tone(SectionTone::Danger)
            .add(input().label("test"))
            .add(input().label("test"))
            .add(input().label("test"))
            .add(input().label("test"))
            .add(input().label("test"))
            .add(input().label("test"))
            .add(input().label("test"))
            .add(input().label("test"))
            .add(input().label("test"))

        )

        .maybe_banner(top_banner)
        .add(card().add(input().kind(InputType::Password)))
        .add(card().add(two_col_with(1,2,input(),input())))
        .add(input().label("Full name").name("name").required()
             .value(values.name.unwrap_or(""))
             .maybe_error(errors.name))
        .add(input().label("Student email").name("email").required()
             .kind(InputType::Email)
             .value(values.email.unwrap_or(""))
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
        .sticky_bottom()
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


    body = body.add(card().add(f).sticky_friendly());


    page_of("Validator crate · error UX", body)
}
