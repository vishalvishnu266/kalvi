//! `<ui-select>` typed builder.

use crate::core::{escape_html, wrap, Attr, Component};

pub struct SelectOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}
impl SelectOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self { value: value.into(), label: label.into(), disabled: false }
    }
    pub fn disabled(mut self) -> Self { self.disabled = true; self }
}

pub struct Select {
    label: Option<String>,
    name: Option<String>,
    value: Option<String>,
    placeholder: Option<String>,
    hint: Option<String>,   // reused for error message when invalid = true
    searchable: bool,
    clearable: bool,
    multiple: bool,
    /// When true, the drawer shows an "Add …" row for search queries that
    /// don't match an existing option. Implies `searchable` on the web
    /// component side. Use this for tag-input / "pick or type new" flows —
    /// replaces the retired standalone `combobox()` builder.
    allow_new: bool,
    required: bool,
    invalid: bool,
    options: Vec<SelectOption>,
}
pub fn select() -> Select {
    Select {
        label: None, name: None, value: None, placeholder: None, hint: None,
        searchable: false, clearable: false, multiple: false,
        allow_new: false, required: false, invalid: false,
        options: Vec::new(),
    }
}
impl Select {
    pub fn label(mut self, s: impl Into<String>)       -> Self { self.label = Some(s.into()); self }
    pub fn name(mut self, s: impl Into<String>)        -> Self { self.name  = Some(s.into()); self }
    pub fn value(mut self, s: impl Into<String>)       -> Self { self.value = Some(s.into()); self }
    pub fn placeholder(mut self, s: impl Into<String>) -> Self { self.placeholder = Some(s.into()); self }
    pub fn hint(mut self, s: impl Into<String>)        -> Self { self.hint = Some(s.into()); self }
    pub fn searchable(mut self) -> Self { self.searchable = true; self }
    pub fn clearable(mut self)  -> Self { self.clearable  = true; self }
    pub fn multiple(mut self)   -> Self { self.multiple   = true; self }
    /// Allow users to add a value that isn't in the options list. Adds a
    /// search box automatically. Replaces the retired `combobox()`.
    pub fn allow_new(mut self)  -> Self { self.allow_new  = true; self }
    pub fn required(mut self)   -> Self { self.required   = true; self }
    pub fn invalid(mut self)    -> Self { self.invalid    = true; self }
    pub fn option(mut self, o: SelectOption) -> Self { self.options.push(o); self }
    pub fn options<I: IntoIterator<Item = SelectOption>>(mut self, iter: I) -> Self {
        self.options.extend(iter); self
    }

    /// Field-level error — sets `invalid` and replaces the hint so it
    /// renders red under the field.
    pub fn error(mut self, msg: impl Into<String>) -> Self {
        self.invalid = true; self.hint = Some(msg.into()); self
    }
    pub fn maybe_error(self, msg: Option<impl Into<String>>) -> Self {
        match msg { Some(m) => self.error(m), None => self }
    }
}
impl Component for Select {
    fn render(&self) -> String {
        let mut attrs = Vec::new();
        if let Some(ref v) = self.label       { attrs.push(Attr::kv("label", v.as_str())); }
        if let Some(ref v) = self.name        { attrs.push(Attr::kv("name",  v.as_str())); }
        if let Some(ref v) = self.value       { attrs.push(Attr::kv("value", v.as_str())); }
        if let Some(ref v) = self.placeholder { attrs.push(Attr::kv("placeholder", v.as_str())); }
        if let Some(ref v) = self.hint        { attrs.push(Attr::kv("hint", v.as_str())); }
        if self.searchable { attrs.push(Attr::flag("searchable")); }
        if self.clearable  { attrs.push(Attr::flag("clearable")); }
        if self.multiple   { attrs.push(Attr::flag("multiple")); }
        if self.allow_new  { attrs.push(Attr::flag("allow-new")); }
        if self.required   { attrs.push(Attr::flag("required")); }
        if self.invalid    { attrs.push(Attr::flag("invalid")); }

        let mut body = String::new();
        for o in &self.options {
            let dis = if o.disabled { " disabled" } else { "" };
            body.push_str(&format!(r#"<option value="{}"{}>{}</option>"#,
                escape_html(&o.value), dis, escape_html(&o.label)));
        }
        wrap("ui-select", &attrs, &body)
    }
}
