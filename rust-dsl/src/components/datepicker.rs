//! `<ui-datepicker>` typed builder.

use crate::core::{wrap, Attr, Component};

pub struct Datepicker {
    label: Option<String>,
    name: Option<String>,
    value: Option<String>,        // ISO YYYY-MM-DD
    placeholder: Option<String>,
}
pub fn datepicker() -> Datepicker {
    Datepicker { label: None, name: None, value: None, placeholder: None }
}
impl Datepicker {
    pub fn label(mut self, s: impl Into<String>)       -> Self { self.label = Some(s.into()); self }
    pub fn name(mut self, s: impl Into<String>)        -> Self { self.name  = Some(s.into()); self }
    pub fn value(mut self, iso: impl Into<String>)     -> Self { self.value = Some(iso.into()); self }
    pub fn placeholder(mut self, s: impl Into<String>) -> Self { self.placeholder = Some(s.into()); self }
}
impl Component for Datepicker {
    fn render(&self) -> String {
        let mut attrs = Vec::new();
        if let Some(ref v) = self.label       { attrs.push(Attr::kv("label",       v.as_str())); }
        if let Some(ref v) = self.name        { attrs.push(Attr::kv("name",        v.as_str())); }
        if let Some(ref v) = self.value       { attrs.push(Attr::kv("value",       v.as_str())); }
        if let Some(ref v) = self.placeholder { attrs.push(Attr::kv("placeholder", v.as_str())); }
        wrap("ui-datepicker", &attrs, "")
    }
}
