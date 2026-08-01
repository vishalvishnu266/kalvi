//! `<ui-form-banner>` typed builder — the **form-level error surface**.
//!
//! Renders as a bordered coloured banner inside a Form (or standalone above
//! any block of fields) to communicate a message that isn't tied to a
//! specific field — a cross-field business rule, a server-side rejection,
//! a "we're working on it" info notice, or a "saved" success confirmation.
//!
//! ## Where each error type lives
//!
//! | Error type | Surface | Component |
//! |---|---|---|
//! | Field-level (e.g. "Email is required") | Under the specific field | `input().error(...)` |
//! | **Form-level** (spans fields, business rule) | **Banner at top of form** | **`form_banner()`**  ← this file |
//! | Page-level (server error, session expiring) | Toast (transient) or Alert (persistent) | `toast()` / `alert()` |
//!
//! ## Usage
//!
//! ```ignore
//! form().action("/students").method("post")
//!     .banner(form_banner()
//!         .tone(Tone::Danger)
//!         .message("Guardian email must be different from the student's email."))
//!     .add(input().label("Student email").name("s_email"))
//!     .add(input().label("Guardian email").name("g_email"))
//!     .save_cancel("Save");
//! ```
//!
//! Or as an "N field errors" summary at the top of a re-rendered form
//! (great for accessibility — screen readers navigate to each error):
//!
//! ```ignore
//! form_banner().tone(Tone::Danger)
//!     .title("Please fix 2 errors")
//!     .errors_summary(vec![
//!         ("name",  "Full name is required"),
//!         ("email", "Guardian email is invalid"),
//!     ]);
//! ```

use crate::core::{escape_html, wrap, Attr, Component};
use crate::components::badge::Tone;

pub struct FormBanner {
    tone: Tone,
    title: Option<String>,
    message: Option<String>,
    /// Optional error-summary list: (field_name, message) pairs. Each pair
    /// becomes a link that scrolls to and focuses the field with that name.
    errors: Vec<(String, String)>,
    /// Multi-section content — used for "errors AND warnings in one banner".
    /// When non-empty, this is rendered INSTEAD of the single title/message/
    /// errors payload above. Each section carries its own tone (so a danger
    /// section can sit next to a warning section) and its own optional
    /// field-summary list.
    ///
    /// Empty by default → backward compatible with the single-tone API.
    sections: Vec<BannerSection>,
    dismissible: bool,
}

/// One section inside a multi-section [`FormBanner`]. Use when you need to
/// show errors AND warnings AND info in a single top-of-form surface — e.g.
/// "2 errors + 1 warning" after a failed submit that has both hard failures
/// and soft business-rule cautions.
///
/// Constructed via [`banner_section`] or directly with fields exposed.
#[derive(Clone)]
pub struct BannerSection {
    pub tone: Tone,
    pub title: Option<String>,
    pub message: Option<String>,
    pub items: Vec<(String, String)>,  // (field_name, message)
}

/// Build one section for a multi-section [`FormBanner`].
///
/// ```ignore
/// form_banner()
///     .add_section(banner_section(Tone::Danger)
///         .title("Please fix 2 errors")
///         .errors_summary(vec![
///             ("email", "Email is invalid").into(),
///             ("phone", "Phone is required").into(),
///         ]))
///     .add_section(banner_section(Tone::Warning)
///         .title("1 warning")
///         .errors_summary(vec![
///             ("date", "Due date is a public holiday").into(),
///         ]))
/// ```
pub fn banner_section(tone: Tone) -> BannerSection {
    BannerSection { tone, title: None, message: None, items: Vec::new() }
}

impl BannerSection {
    pub fn title(mut self, s: impl Into<String>)   -> Self { self.title   = Some(s.into()); self }
    pub fn message(mut self, s: impl Into<String>) -> Self { self.message = Some(s.into()); self }
    pub fn errors_summary<I: IntoIterator<Item = FieldError>>(mut self, items: I) -> Self {
        self.items.extend(items.into_iter().map(|fe| (fe.field, fe.message)));
        self
    }
}

pub fn form_banner() -> FormBanner {
    FormBanner {
        tone: Tone::Info,
        title: None, message: None,
        errors: Vec::new(), sections: Vec::new(),
        dismissible: false,
    }
}

impl FormBanner {
    /// Semantic tone: `Info`, `Success`, `Warning`, `Danger`. Reuses the
    /// same `Tone` enum as badges/alerts for consistency.
    pub fn tone(mut self, t: Tone) -> Self { self.tone = t; self }
    pub fn title(mut self, s: impl Into<String>)   -> Self { self.title   = Some(s.into()); self }
    pub fn message(mut self, s: impl Into<String>) -> Self { self.message = Some(s.into()); self }

    /// Add an error-summary list. Each `(field_name, message)` becomes a
    /// clickable link that jumps to the field with that `name` attribute
    /// (via `document.getElementsByName(...)[0]`).
    pub fn errors_summary<I: IntoIterator<Item = (String, String)>>(mut self, items: I) -> Self {
        self.errors.extend(items); self
    }

    /// Show a close button. On click the banner is removed from the DOM.
    pub fn dismissible(mut self) -> Self { self.dismissible = true; self }

    /// Append a section. When at least one section is added, the top-level
    /// `.title()` / `.message()` / `.errors_summary()` payload is IGNORED
    /// and the banner renders as a stack of tinted sections instead. Use
    /// this to combine errors AND warnings in a single banner without
    /// creating two separate ones.
    ///
    /// The outer banner still has an overall `.tone()` — use it to set the
    /// dominant colour (usually `Danger` if any section is a danger). The
    /// underlying `<ui-form-banner>` also computes the highest severity of
    /// the sections and sets it as `aria-live` / `role` on the container.
    pub fn add_section(mut self, s: BannerSection) -> Self {
        self.sections.push(s); self
    }

    /// Convenience — add many sections in one call. See [`add_section`].
    pub fn sections<I: IntoIterator<Item = BannerSection>>(mut self, iter: I) -> Self {
        self.sections.extend(iter); self
    }
}

/// One-line helper for server handlers that already have a
/// `HashMap<&str, String>` (or similar) of field-name → error-message pairs.
///
/// Returns `Some(form_banner…)` when there ARE errors, `None` when the map
/// is empty. Pair it with [`crate::components::form::Form::maybe_banner`]:
///
/// ```ignore
/// form().action("/students").method("post")
///     .maybe_banner(errors_summary_banner(&errors))   // ← one line
///     .add(input().label("Full name").name("name")
///          .maybe_error(errors.get("name").cloned()))
///     .add(input().label("Email").name("email")
///          .maybe_error(errors.get("email").cloned()))
///     .save_cancel("Save")
/// ```
///
/// The banner tone is always `Danger` and the title auto-pluralises
/// ("Please fix N error" / "Please fix N errors"). The list is passed as
/// the errors_summary, so each item becomes a click-to-focus link.
pub fn errors_summary_banner<I>(errors: I) -> Option<FormBanner>
where
    I: IntoIterator,
    I::Item: Into<FieldError>,
{
    let items: Vec<(String, String)> = errors
        .into_iter()
        .map(Into::into)
        .map(|fe| (fe.field, fe.message))
        .collect();

    if items.is_empty() { return None; }

    let n = items.len();
    let title = if n == 1 { "Please fix 1 error".to_string() }
                else      { format!("Please fix {n} errors") };

    Some(form_banner()
        .tone(crate::components::badge::Tone::Danger)
        .title(title)
        .errors_summary(items))
}

/// Small adapter type so `errors_summary_banner` accepts many shapes:
/// `(&str, &str)`, `(String, String)`, references to same, iterators of
/// tuples, hashmap iter, etc.
#[derive(Clone)]
pub struct FieldError { pub field: String, pub message: String }

impl<A: Into<String>, B: Into<String>> From<(A, B)> for FieldError {
    fn from((f, m): (A, B)) -> Self {
        FieldError { field: f.into(), message: m.into() }
    }
}

/// Two-section helper for the very common "N errors + M warnings" case
/// after a form submit. Auto-pluralises titles, hides empty sections,
/// returns `None` if BOTH lists are empty.
///
/// ```ignore
/// form().action("/students").method("post")
///     .maybe_banner(errors_and_warnings_banner(&errors, &warnings))
///     .add(...)
///     .save_cancel("Save")
/// ```
pub fn errors_and_warnings_banner<E, W>(
    errors: E, warnings: W,
) -> Option<FormBanner>
where
    E: IntoIterator, E::Item: Into<FieldError>,
    W: IntoIterator, W::Item: Into<FieldError>,
{
    let errs: Vec<FieldError>  = errors.into_iter().map(Into::into).collect();
    let warns: Vec<FieldError> = warnings.into_iter().map(Into::into).collect();
    if errs.is_empty() && warns.is_empty() { return None; }

    let mut b = form_banner()
        // Outer tone = worst severity present. Danger > Warning.
        .tone(if !errs.is_empty() { Tone::Danger } else { Tone::Warning });

    if !errs.is_empty() {
        let n = errs.len();
        let title = if n == 1 { "Please fix 1 error".to_string() }
                    else      { format!("Please fix {n} errors") };
        b = b.add_section(banner_section(Tone::Danger)
                .title(title)
                .errors_summary(errs));
    }
    if !warns.is_empty() {
        let n = warns.len();
        let title = if n == 1 { "1 warning".to_string() }
                    else      { format!("{n} warnings") };
        b = b.add_section(banner_section(Tone::Warning)
                .title(title)
                .errors_summary(warns));
    }
    Some(b)
}

fn tone_str(t: Tone) -> &'static str {
    match t {
        Tone::Neutral => "neutral",
        Tone::Brand   => "brand",
        Tone::Success => "success",
        Tone::Warning => "warning",
        Tone::Danger  => "danger",
        Tone::Info    => "info",
    }
}

impl Component for FormBanner {
    fn render(&self) -> String {
        let mut attrs = vec![Attr::kv("tone", tone_str(self.tone))];
        if let Some(ref v) = self.title   { attrs.push(Attr::kv("title",   v.as_str())); }
        if let Some(ref v) = self.message { attrs.push(Attr::kv("message", v.as_str())); }
        if self.dismissible { attrs.push(Attr::flag("dismissible")); }
        if !self.sections.is_empty() { attrs.push(Attr::flag("multi-section")); }

        let mut body = String::new();

        // ── Multi-section mode ──
        if !self.sections.is_empty() {
            for s in &self.sections {
                body.push_str(&format!(
                    r#"<div slot="section" data-tone="{tone}">"#,
                    tone = tone_str(s.tone),
                ));
                if let Some(ref t) = s.title {
                    body.push_str(&format!(r#"<p class="sec-title">{}</p>"#, escape_html(t)));
                }
                if let Some(ref m) = s.message {
                    body.push_str(&format!(r#"<p class="sec-msg">{}</p>"#, escape_html(m)));
                }
                if !s.items.is_empty() {
                    body.push_str("<ul>");
                    for (field, msg) in &s.items {
                        body.push_str(&format!(
                            r##"<li><a data-field="{}" href="#">{}</a></li>"##,
                            escape_html(field), escape_html(msg),
                        ));
                    }
                    body.push_str("</ul>");
                }
                body.push_str("</div>");
            }
        } else if !self.errors.is_empty() {
            // ── Single-section mode (backward compat) ──
            body.push_str(r#"<ul slot="errors">"#);
            for (field, msg) in &self.errors {
                body.push_str(&format!(
                    r##"<li><a data-field="{}" href="#">{}</a></li>"##,
                    escape_html(field), escape_html(msg),
                ));
            }
            body.push_str("</ul>");
        }
        wrap("ui-form-banner", &attrs, &body)
    }
}
