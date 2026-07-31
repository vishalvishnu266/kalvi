//! `<ui-checkbox>` typed builder.

use crate::core::{escape_html, wrap, Attr, Component};

pub struct Checkbox {
    label: String, name: Option<String>, value: Option<String>,
    checked: bool, indeterminate: bool, disabled: bool, required: bool,
}
pub fn checkbox(label: impl Into<String>) -> Checkbox {
    Checkbox {
        label: label.into(), name: None, value: None,
        checked: false, indeterminate: false, disabled: false, required: false,
    }
}
impl Checkbox {
    pub fn name(mut self, s: impl Into<String>)  -> Self { self.name  = Some(s.into()); self }
    pub fn value(mut self, s: impl Into<String>) -> Self { self.value = Some(s.into()); self }
    pub fn checked(mut self)       -> Self { self.checked = true; self }
    pub fn indeterminate(mut self) -> Self { self.indeterminate = true; self }
    pub fn disabled(mut self)      -> Self { self.disabled = true; self }
    pub fn required(mut self)      -> Self { self.required = true; self }
}
impl Component for Checkbox {
    fn render(&self) -> String {
        let mut attrs = Vec::new();
        if let Some(ref n) = self.name  { attrs.push(Attr::kv("name",  n.as_str())); }
        if let Some(ref v) = self.value { attrs.push(Attr::kv("value", v.as_str())); }
        if self.checked       { attrs.push(Attr::flag("checked")); }
        if self.indeterminate { attrs.push(Attr::flag("indeterminate")); }
        if self.disabled      { attrs.push(Attr::flag("disabled")); }
        if self.required      { attrs.push(Attr::flag("required")); }
        wrap("ui-checkbox", &attrs, &escape_html(&self.label))
    }
}
