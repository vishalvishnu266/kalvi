//! `<ui-combobox>` typed builder.

use crate::core::{escape_html, wrap, Attr, Component};

pub struct ComboOption { pub value: String, pub label: String }
impl ComboOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self { value: value.into(), label: label.into() }
    }
}

pub struct Combobox {
    label: Option<String>,
    name: Option<String>,
    value: Option<String>,
    placeholder: Option<String>,
    hint: Option<String>,   // reused for error message when invalid = true
    required: bool,
    invalid: bool,
    allow_new: bool,
    options: Vec<ComboOption>,
}
pub fn combobox() -> Combobox {
    Combobox { label: None, name: None, value: None, placeholder: None, hint: None,
               required: false, invalid: false, allow_new: false, options: Vec::new() }
}
impl Combobox {
    pub fn label(mut self, s: impl Into<String>)       -> Self { self.label = Some(s.into()); self }
    pub fn name(mut self, s: impl Into<String>)        -> Self { self.name = Some(s.into()); self }
    pub fn value(mut self, s: impl Into<String>)       -> Self { self.value = Some(s.into()); self }
    pub fn placeholder(mut self, s: impl Into<String>) -> Self { self.placeholder = Some(s.into()); self }
    pub fn hint(mut self, s: impl Into<String>)        -> Self { self.hint = Some(s.into()); self }
    pub fn required(mut self)  -> Self { self.required  = true; self }
    pub fn invalid(mut self)   -> Self { self.invalid   = true; self }
    pub fn allow_new(mut self) -> Self { self.allow_new = true; self }
    pub fn option(mut self, o: ComboOption) -> Self { self.options.push(o); self }
    pub fn options<I: IntoIterator<Item = ComboOption>>(mut self, iter: I) -> Self {
        self.options.extend(iter); self
    }

    /// Field-level error — sets `invalid` and replaces the hint with the
    /// error message so it renders red under the field.
    pub fn error(mut self, msg: impl Into<String>) -> Self {
        self.invalid = true; self.hint = Some(msg.into()); self
    }
    pub fn maybe_error(self, msg: Option<impl Into<String>>) -> Self {
        match msg { Some(m) => self.error(m), None => self }
    }
}
impl Component for Combobox {
    fn render(&self) -> String {
        let mut attrs = Vec::new();
        if let Some(ref v) = self.label       { attrs.push(Attr::kv("label",       v.as_str())); }
        if let Some(ref v) = self.name        { attrs.push(Attr::kv("name",        v.as_str())); }
        if let Some(ref v) = self.value       { attrs.push(Attr::kv("value",       v.as_str())); }
        if let Some(ref v) = self.placeholder { attrs.push(Attr::kv("placeholder", v.as_str())); }
        if let Some(ref v) = self.hint        { attrs.push(Attr::kv("hint",        v.as_str())); }
        if self.required  { attrs.push(Attr::flag("required")); }
        if self.invalid   { attrs.push(Attr::flag("invalid")); }
        if self.allow_new { attrs.push(Attr::flag("allow-new")); }
        let body: String = self.options.iter()
            .map(|o| format!(r#"<option value="{}">{}</option>"#, escape_html(&o.value), escape_html(&o.label)))
            .collect();
        wrap("ui-combobox", &attrs, &body)
    }
}
