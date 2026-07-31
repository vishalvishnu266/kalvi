//! `<ui-switch>` typed builder.

use crate::core::{escape_html, wrap, Attr, Component};

pub struct Switch {
    label: String, name: Option<String>, value: Option<String>,
    checked: bool, disabled: bool,
}
pub fn switch(label: impl Into<String>) -> Switch {
    Switch { label: label.into(), name: None, value: None, checked: false, disabled: false }
}
impl Switch {
    pub fn name(mut self, n: impl Into<String>)  -> Self { self.name  = Some(n.into()); self }
    pub fn value(mut self, v: impl Into<String>) -> Self { self.value = Some(v.into()); self }
    pub fn checked(mut self)  -> Self { self.checked  = true; self }
    pub fn disabled(mut self) -> Self { self.disabled = true; self }
}
impl Component for Switch {
    fn render(&self) -> String {
        let mut attrs = Vec::new();
        if let Some(ref n) = self.name  { attrs.push(Attr::kv("name",  n.as_str())); }
        if let Some(ref v) = self.value { attrs.push(Attr::kv("value", v.as_str())); }
        if self.checked  { attrs.push(Attr::flag("checked")); }
        if self.disabled { attrs.push(Attr::flag("disabled")); }
        wrap("ui-switch", &attrs, &escape_html(&self.label))
    }
}
