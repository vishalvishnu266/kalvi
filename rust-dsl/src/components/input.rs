//! `<ui-input>` — Vaadin-style typed builder.
//!
//! ```ignore
//! use lit_ui::prelude::*;
//!
//! let html = input()
//!     .label("Full name")
//!     .name("fullName")
//!     .placeholder("e.g. Aarav")
//!     .required()
//!     .render();
//! ```

use crate::core::{wrap, Attr, Component};

/// The `type` attribute of an input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputType {
    Text, Password, Email, Number, Textarea,
}
impl InputType {
    fn as_str(self) -> &'static str {
        match self {
            InputType::Text     => "text",
            InputType::Password => "password",
            InputType::Email    => "email",
            InputType::Number   => "number",
            InputType::Textarea => "textarea",
        }
    }
}

/// `<ui-input>` builder.
pub struct Input {
    label: Option<String>,
    name: Option<String>,
    kind: InputType,
    value: Option<String>,
    placeholder: Option<String>,
    hint: Option<String>,
    required: bool,
    invalid: bool,
}

/// Start building a new input.
pub fn input() -> Input {
    Input {
        label: None, name: None, kind: InputType::Text,
        value: None, placeholder: None, hint: None,
        required: false, invalid: false,
    }
}

impl Input {
    pub fn label(mut self, s: impl Into<String>)       -> Self { self.label       = Some(s.into()); self }
    pub fn name(mut self, s: impl Into<String>)        -> Self { self.name        = Some(s.into()); self }
    pub fn value(mut self, s: impl Into<String>)       -> Self { self.value       = Some(s.into()); self }
    pub fn placeholder(mut self, s: impl Into<String>) -> Self { self.placeholder = Some(s.into()); self }
    pub fn hint(mut self, s: impl Into<String>)        -> Self { self.hint        = Some(s.into()); self }
    pub fn kind(mut self, k: InputType)                -> Self { self.kind        = k;              self }
    pub fn required(mut self)                          -> Self { self.required = true; self }
    pub fn invalid(mut self)                           -> Self { self.invalid  = true; self }

    /// Attach a **field-level error message**. Convention:
    ///   * Sets `invalid` → red border + red text below.
    ///   * Replaces any `hint` so the user sees the actionable error.
    ///   * The underlying `<ui-input>` renders it as the `hint` slot in
    ///     red because `[invalid] .hint { color: var(--color-danger); }`.
    ///
    /// Server-render pattern:
    /// ```ignore
    /// let email = input().label("Email").name("email").kind(InputType::Email).required();
    /// let email = match errors.get("email") {
    ///     Some(msg) => email.error(msg),
    ///     None      => email,
    /// };
    /// ```
    ///
    /// Or even cleaner via [`Input::maybe_error`] below.
    pub fn error(mut self, msg: impl Into<String>) -> Self {
        self.invalid = true;
        self.hint = Some(msg.into());
        self
    }

    /// Convenience — accepts an `Option<&str>` so server code doesn't need
    /// a `match` around every field. `None` = clean state; `Some(msg)` =
    /// same as `.error(msg)`.
    pub fn maybe_error(self, msg: Option<impl Into<String>>) -> Self {
        match msg { Some(m) => self.error(m), None => self }
    }
}

impl Component for Input {
    fn render(&self) -> String {
        let mut attrs = vec![Attr::kv("type", self.kind.as_str())];
        if let Some(ref v) = self.label       { attrs.push(Attr::kv("label",       v.as_str())); }
        if let Some(ref v) = self.name        { attrs.push(Attr::kv("name",        v.as_str())); }
        if let Some(ref v) = self.value       { attrs.push(Attr::kv("value",       v.as_str())); }
        if let Some(ref v) = self.placeholder { attrs.push(Attr::kv("placeholder", v.as_str())); }
        if let Some(ref v) = self.hint        { attrs.push(Attr::kv("hint",        v.as_str())); }
        if self.required { attrs.push(Attr::flag("required")); }
        if self.invalid  { attrs.push(Attr::flag("invalid")); }

        // ui-input has no slotted children — always self-closing style.
        wrap("ui-input", &attrs, "")
    }
}
