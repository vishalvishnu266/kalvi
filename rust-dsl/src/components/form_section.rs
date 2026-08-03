//! `<ui-collapse>` typed builder — a **foldable section** for long forms.
//!
//! Big ERP forms (Student admission, Employee onboarding, Fee structure,
//! Purchase order, Invoice) quickly grow to 40–80 fields. Rather than
//! forcing users to scroll through one giant slab, group related fields
//! into collapsible sections:
//!
//! ```ignore
//! use lit_ui::prelude::*;
//!
//! form().action("/students").method("post")
//!     .csrf(&csrf_token)
//!     .add(form_section("Personal details").open()
//!         .add(input().label("First name").name("first"))
//!         .add(input().label("Last name").name("last"))
//!         .add(datepicker().label("Date of birth").name("dob")))
//!     .add(form_section("Guardian")
//!         .add(input().label("Guardian name").name("gname"))
//!         .add(input().label("Guardian email").name("gemail")))
//!     .add(form_section("Address")
//!         .add(input().label("Street").name("street"))
//!         .add(input().label("City").name("city")))
//!     .add(form_section("Medical")
//!         .subtitle("Allergies, prescriptions, emergency contact")
//!         .add(input().kind(InputType::Textarea).label("Notes").name("med")))
//!     .save_cancel("Create student");
//! ```
//!
//! Sections are **expanded by default** so users never overlook a whole
//! block of fields hidden behind a closed header. Use `.collapsed()` on
//! the sections that should start folded — typically optional / rarely-
//! used blocks (e.g. "Advanced settings", "Custom fields").
//!
//! ## Highlighting sections with errors
//!
//! When a section contains field-level errors, paint its header red via
//! `.danger()` so the user can spot which collapsed section to open:
//!
//! ```ignore
//! let personal = form_section("Personal details");
//! let personal = if errors.any_in(&["first", "last", "dob"]) {
//!     personal.danger().open()   // red header + auto-expand
//! } else {
//!     personal
//! };
//! ```

use crate::core::{wrap, Attr, Child, Component};

/// Header tone for a section. `Neutral` is the default; `Danger` paints
/// the header red so the user knows a collapsed section has errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectionTone { Neutral, Danger }

/// `<ui-collapse>` builder — one foldable section inside a form.
pub struct FormSection {
    title: String,
    subtitle: Option<String>,
    /// Tri-state expansion:
    ///   None       → follow the component default (currently: open)
    ///   Some(true) → explicitly expanded
    ///   Some(false)→ explicitly collapsed
    /// We keep the tri-state so `.open()` remains a compile-safe no-op
    /// for old call sites while `.collapsed()` is the modern opt-out.
    open: Option<bool>,
    disabled: bool,
    tone: SectionTone,
    actions: Vec<Child>,
    children: Vec<Child>,
}

/// Start a new foldable form section. `title` shows in the header.
///
/// **Sections are expanded by default** — call [`FormSection::collapsed`]
/// on optional/rarely-used sections that should start folded.
pub fn form_section(title: impl Into<String>) -> FormSection {
    FormSection {
        title: title.into(),
        subtitle: None,
        open: None,     // ← inherit component default (expanded)
        disabled: false,
        tone: SectionTone::Neutral,
        actions: Vec::new(),
        children: Vec::new(),
    }
}

impl FormSection {
    /// Optional secondary line under the title (e.g. "Optional",
    /// "Auto-filled from ID", "3 errors").
    pub fn subtitle(mut self, s: impl Into<String>) -> Self {
        self.subtitle = Some(s.into()); self
    }
    /// Explicitly start expanded. Sections are **expanded by default**,
    /// so calling this is only necessary if you previously wrote
    /// `.collapsed()` on the same section and want to override it, or
    /// if you like being explicit at the call site.
    pub fn open(mut self) -> Self { self.open = Some(true); self }
    /// Start collapsed — for optional / rarely-used sections
    /// ("Advanced settings", "Custom fields", "Notes for internal use").
    ///
    /// ```ignore
    /// form_section("Advanced").collapsed()
    ///     .add(input().label("Legacy code").name("legacy_id"))
    /// ```
    pub fn collapsed(mut self) -> Self { self.open = Some(false); self }
    /// Grey out the header and disable toggling. Useful for sections
    /// that are locked until an upstream field is filled in.
    pub fn disabled(mut self) -> Self { self.disabled = true; self }
    /// Paint the header red — call this when the section contains
    /// field-level errors, so users can locate them at a glance.
    /// Usually paired with `.open()` so the errors are immediately
    /// visible.
    pub fn danger(mut self) -> Self { self.tone = SectionTone::Danger; self }
    /// Explicit tone setter for symmetry with other components.
    pub fn tone(mut self, t: SectionTone) -> Self { self.tone = t; self }

    /// Add one field/child into the section body.
    pub fn add(mut self, c: impl Component + 'static) -> Self {
        self.children.push(Box::new(c)); self
    }
    /// Add many children at once.
    pub fn children<I, C>(mut self, iter: I) -> Self
    where I: IntoIterator<Item = C>, C: Component + 'static {
        for c in iter { self.children.push(Box::new(c)); } self
    }

    /// Add a child to the header's `actions` slot (top-right — e.g. an
    /// error-count badge, a "Reset section" button).
    pub fn action(mut self, c: impl Component + 'static) -> Self {
        self.actions.push(Box::new(c)); self
    }
}

impl Component for FormSection {
    fn render(&self) -> String {
        let mut attrs: Vec<Attr> = vec![Attr::kv("title", self.title.as_str())];
        if let Some(ref s) = self.subtitle { attrs.push(Attr::kv("subtitle", s.as_str())); }
        // Emit `collapsed` when the author explicitly opts out. When
        // `open` is None we simply omit the attribute, so the component
        // default (expanded) takes effect. `.open()` maps to explicitly
        // rendering the `open` flag for anyone reading the raw HTML,
        // though the component would default to open anyway.
        match self.open {
            Some(true)  => attrs.push(Attr::flag("open")),
            Some(false) => attrs.push(Attr::flag("collapsed")),
            None        => {}
        }
        if self.disabled { attrs.push(Attr::flag("disabled")); }
        if matches!(self.tone, SectionTone::Danger) {
            attrs.push(Attr::kv("tone", "danger"));
        }

        let mut body = String::new();
        for a in &self.actions {
            body.push_str(r#"<span slot="actions">"#);
            body.push_str(&a.render());
            body.push_str("</span>");
        }
        for c in &self.children {
            body.push_str(&c.render());
        }
        wrap("ui-collapse", &attrs, &body)
    }
}
