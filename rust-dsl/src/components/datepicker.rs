//! `<ui-datepicker>` typed builder.

use crate::core::{wrap, Attr, Component};

pub struct Datepicker {
    label: Option<String>,
    name: Option<String>,
    value: Option<String>,        // ISO YYYY-MM-DD
    placeholder: Option<String>,
    hint: Option<String>,
    invalid: bool,
}
pub fn datepicker() -> Datepicker {
    Datepicker { label: None, name: None, value: None, placeholder: None, hint: None, invalid: false }
}
impl Datepicker {
    pub fn label(mut self, s: impl Into<String>)       -> Self { self.label = Some(s.into()); self }
    pub fn name(mut self, s: impl Into<String>)        -> Self { self.name  = Some(s.into()); self }
    pub fn value(mut self, iso: impl Into<String>)     -> Self { self.value = Some(iso.into()); self }
    pub fn placeholder(mut self, s: impl Into<String>) -> Self { self.placeholder = Some(s.into()); self }
    pub fn hint(mut self, s: impl Into<String>)        -> Self { self.hint = Some(s.into()); self }
    pub fn invalid(mut self)                            -> Self { self.invalid = true; self }

    /// Field-level error — sets `invalid` and replaces the hint with the
    /// error message so it renders red under the field.
    pub fn error(mut self, msg: impl Into<String>) -> Self {
        self.invalid = true; self.hint = Some(msg.into()); self
    }
    pub fn maybe_error(self, msg: Option<impl Into<String>>) -> Self {
        match msg { Some(m) => self.error(m), None => self }
    }
}
impl Component for Datepicker {
    fn render(&self) -> String {
        let mut attrs = Vec::new();
        if let Some(ref v) = self.label       { attrs.push(Attr::kv("label",       v.as_str())); }
        if let Some(ref v) = self.name        { attrs.push(Attr::kv("name",        v.as_str())); }
        if let Some(ref v) = self.value       { attrs.push(Attr::kv("value",       v.as_str())); }
        if let Some(ref v) = self.placeholder { attrs.push(Attr::kv("placeholder", v.as_str())); }
        if let Some(ref v) = self.hint        { attrs.push(Attr::kv("hint",        v.as_str())); }
        if self.invalid { attrs.push(Attr::flag("invalid")); }
        wrap("ui-datepicker", &attrs, "")
    }
}
