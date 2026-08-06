//! `<ui-select>` typed builder — refactored onto `#[derive(UiComponent)]`.
//!
//! `SelectOption` is a plain data struct (not a component) because it
//! renders as a native `<option>` inside the select's body, not as
//! another custom element. That means the derive's `#[ui(children)]`
//! doesn't fit — we hand-render the option list in a small `Component`
//! impl override below.
//!
//! Alternative considered: use `#[ui(children)] Vec<Child>` and require
//! callers to `.add(option(...))`. Rejected because it loses the typed
//! `SelectOption` contract (`.value` + `.label` + `.disabled` visible in
//! IDE), and `<option>` elements have no `.render()` of their own.

use crate::core::{escape_html, wrap, Attr, Component};
use lit_ui_macros::UiComponent;

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

// `no_component`: the derive still generates setters + Default, but we
// hand-write `impl Component` below because the body is native `<option>`
// HTML rather than a `Vec<Child>` of components.
#[derive(UiComponent)]
#[ui(tag = "ui-select", no_ctor, no_component)]
pub struct Select {
    #[ui(attr = "label")]                     pub label: Option<String>,
    #[ui(attr = "name")]                      pub name: Option<String>,
    #[ui(attr = "value")]                     pub value: Option<String>,
    #[ui(attr = "placeholder")]               pub placeholder: Option<String>,
    #[ui(attr = "hint", no_setter)]           pub hint: Option<String>,
    #[ui(flag = "searchable")]                pub searchable: bool,
    #[ui(flag = "clearable")]                 pub clearable: bool,
    #[ui(flag = "multiple")]                  pub multiple: bool,
    #[ui(flag = "allow-new")]                 pub allow_new: bool,
    #[ui(flag = "required")]                  pub required: bool,
    #[ui(flag = "invalid")]                   pub invalid: bool,
    #[ui(skip)]                               pub options: Vec<SelectOption>,
}

/// Custom `select()` constructor — nothing required, but we keep the
/// free function so the API matches the pre-refactor call site.
pub fn select() -> Select { <Select as Default>::default() }

impl Select {
    // `.hint()` is hand-written because the field uses `no_setter` (so we
    // can offer a *plain* single-field `.hint(msg)` here) — and because
    // `.error(msg)` also mutates `hint`, we want both under our control.
    pub fn hint(mut self, s: impl Into<String>) -> Self { self.hint = Some(s.into()); self }
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

// Hand-written Component impl (see `#[ui(no_component)]` above). We rebuild
// the attribute list in the same wire order the derive would, then append
// the options as native `<option>` HTML in the body.
impl Component for Select {
    fn render(&self) -> String {
        let mut attrs = Vec::new();
        if let Some(ref v) = self.label       { attrs.push(Attr::kv("label", v.as_str())); }
        if let Some(ref v) = self.name        { attrs.push(Attr::kv("name",  v.as_str())); }
        if let Some(ref v) = self.value       { attrs.push(Attr::kv("value", v.as_str())); }
        if let Some(ref v) = self.placeholder { attrs.push(Attr::kv("placeholder", v.as_str())); }
        if let Some(ref v) = self.hint        { attrs.push(Attr::kv("hint",  v.as_str())); }
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
