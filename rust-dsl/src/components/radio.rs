//! `<ui-radio>` + `<ui-radio-group>` typed builders.
//!
//! * `Radio` is fully derived — pure primitive.
//! * `RadioGroup` keeps a hand-written `Component` impl because its body
//!   is a typed `Vec<Radio>` (not `Vec<Child>`), so the derive's
//!   `#[ui(children)]` — which assumes `Vec<Child>` — doesn't apply. The
//!   typed vec is deliberate: it prevents callers from putting arbitrary
//!   components (e.g. a `Button`) into a radio group.

use crate::core::{wrap, Attr, Component};
use lit_ui_macros::UiComponent;

// ── Radio (leaf) ────────────────────────────────────────────────────────

pub fn radio(value: impl Into<String>, label: impl Into<String>) -> Radio {
    let mut r = <Radio as Default>::default();
    r.value = value.into();
    r.label = label.into();
    r
}

#[derive(UiComponent)]
#[ui(tag = "ui-radio", no_ctor)]
pub struct Radio {
    #[ui(attr = "value")]     pub value: String,
    #[ui(slot)]               pub label: String,
    #[ui(flag = "disabled")]  pub disabled: bool,
}

// ── RadioGroup (container over typed Vec<Radio>) ────────────────────────

pub struct RadioGroup {
    name: String,
    value: Option<String>,
    orientation_horizontal: bool,
    error: Option<String>,
    invalid: bool,
    options: Vec<Radio>,
}

pub fn radio_group(name: impl Into<String>) -> RadioGroup {
    RadioGroup {
        name: name.into(), value: None, orientation_horizontal: false,
        error: None, invalid: false, options: Vec::new(),
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_radio() {
        let html = radio("a", "Apples").render();
        assert_eq!(html, r#"<ui-radio value="a">Apples</ui-radio>"#);
    }

    #[test]
    fn radio_disabled() {
        let html = radio("b", "Bananas").disabled().render();
        assert_eq!(html, r#"<ui-radio value="b" disabled>Bananas</ui-radio>"#);
    }

    #[test]
    fn radio_group_defaults() {
        let html = radio_group("fruit").render();
        assert_eq!(html, r#"<ui-radio-group name="fruit"></ui-radio-group>"#);
    }

    #[test]
    fn radio_group_with_options_and_error() {
        let html = radio_group("fruit")
            .value("a")
            .horizontal()
            .option(radio("a", "Apples"))
            .option(radio("b", "Bananas"))
            .error("Pick one")
            .render();
        assert_eq!(
            html,
            r#"<ui-radio-group name="fruit" value="a" error="Pick one" orientation="horizontal" invalid><ui-radio value="a">Apples</ui-radio><ui-radio value="b">Bananas</ui-radio></ui-radio-group>"#
        );
    }
}
