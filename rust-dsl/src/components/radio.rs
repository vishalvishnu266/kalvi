//! `<ui-radio>` + `<ui-radio-group>` typed builders.

use crate::core::{escape_html, wrap, Attr, Component};

pub struct Radio { value: String, label: String, disabled: bool }
pub fn radio(value: impl Into<String>, label: impl Into<String>) -> Radio {
    Radio { value: value.into(), label: label.into(), disabled: false }
}
impl Radio { pub fn disabled(mut self) -> Self { self.disabled = true; self } }
impl Component for Radio {
    fn render(&self) -> String {
        let mut attrs = vec![Attr::kv("value", self.value.as_str())];
        if self.disabled { attrs.push(Attr::flag("disabled")); }
        wrap("ui-radio", &attrs, &escape_html(&self.label))
    }
}

pub struct RadioGroup {
    name: String, value: Option<String>, orientation_horizontal: bool,
    error: Option<String>, invalid: bool,
    options: Vec<Radio>,
}
pub fn radio_group(name: impl Into<String>) -> RadioGroup {
    RadioGroup { name: name.into(), value: None, orientation_horizontal: false,
                 error: None, invalid: false, options: Vec::new() }
}
impl RadioGroup {
    pub fn value(mut self, v: impl Into<String>) -> Self { self.value = Some(v.into()); self }
    pub fn horizontal(mut self)                  -> Self { self.orientation_horizontal = true; self }
    pub fn option(mut self, r: Radio)            -> Self { self.options.push(r); self }
    pub fn options<I: IntoIterator<Item = Radio>>(mut self, iter: I) -> Self {
        self.options.extend(iter); self
    }
    pub fn invalid(mut self) -> Self { self.invalid = true; self }
    pub fn error(mut self, msg: impl Into<String>) -> Self {
        self.invalid = true; self.error = Some(msg.into()); self
    }
    pub fn maybe_error(self, msg: Option<impl Into<String>>) -> Self {
        match msg { Some(m) => self.error(m), None => self }
    }
}
impl Component for RadioGroup {
    fn render(&self) -> String {
        let mut attrs = vec![Attr::kv("name", self.name.as_str())];
        if let Some(ref v) = self.value { attrs.push(Attr::kv("value", v.as_str())); }
        if let Some(ref e) = self.error { attrs.push(Attr::kv("error", e.as_str())); }
        if self.orientation_horizontal { attrs.push(Attr::kv("orientation", "horizontal")); }
        if self.invalid { attrs.push(Attr::flag("invalid")); }
        let body: String = self.options.iter().map(|o| o.render()).collect();
        wrap("ui-radio-group", &attrs, &body)
    }
}
