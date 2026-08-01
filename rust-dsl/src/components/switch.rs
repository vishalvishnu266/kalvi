//! `<ui-switch>` typed builder.

use crate::core::{escape_html, wrap, Attr, Component};

pub struct Switch {
    label: String, name: Option<String>, value: Option<String>,
    error: Option<String>,
    checked: bool, disabled: bool, invalid: bool,
}
pub fn switch(label: impl Into<String>) -> Switch {
    Switch { label: label.into(), name: None, value: None, error: None,
             checked: false, disabled: false, invalid: false }
}
impl Switch {
    pub fn name(mut self, n: impl Into<String>)  -> Self { self.name  = Some(n.into()); self }
    pub fn value(mut self, v: impl Into<String>) -> Self { self.value = Some(v.into()); self }
    pub fn checked(mut self)  -> Self { self.checked  = true; self }
    pub fn disabled(mut self) -> Self { self.disabled = true; self }
    pub fn invalid(mut self)  -> Self { self.invalid  = true; self }

    /// Field-level error — sets `invalid` and passes the message via the
    /// `error` attribute. Rare on a switch, but sometimes needed for
    /// "you must enable X to save" style rules.
    pub fn error(mut self, msg: impl Into<String>) -> Self {
        self.invalid = true; self.error = Some(msg.into()); self
    }
    pub fn maybe_error(self, msg: Option<impl Into<String>>) -> Self {
        match msg { Some(m) => self.error(m), None => self }
    }
}
impl Component for Switch {
    fn render(&self) -> String {
        let mut attrs = Vec::new();
        if let Some(ref n) = self.name  { attrs.push(Attr::kv("name",  n.as_str())); }
        if let Some(ref v) = self.value { attrs.push(Attr::kv("value", v.as_str())); }
        if let Some(ref e) = self.error { attrs.push(Attr::kv("error", e.as_str())); }
        if self.checked  { attrs.push(Attr::flag("checked")); }
        if self.disabled { attrs.push(Attr::flag("disabled")); }
        if self.invalid  { attrs.push(Attr::flag("invalid")); }
        wrap("ui-switch", &attrs, &escape_html(&self.label))
    }
}
