//! `<ui-datepicker>` typed builder — refactored onto `#[derive(UiComponent)]`.
//!
//! `hint` uses `no_setter` so the hand-written `.hint()` can coexist with
//! the two-field `.error()` below.

#[allow(unused_imports)]
use crate::core::Component;
use lit_ui_macros::UiComponent;

#[derive(UiComponent)]
#[ui(tag = "ui-datepicker")]
pub struct Datepicker {
    #[ui(attr = "label")]                  pub label: Option<String>,
    #[ui(attr = "name")]                   pub name: Option<String>,
    #[ui(attr = "value")]                  pub value: Option<String>,
    #[ui(attr = "placeholder")]            pub placeholder: Option<String>,
    #[ui(attr = "hint", no_setter)]        pub hint: Option<String>,
    #[ui(flag = "invalid")]                pub invalid: bool,
}

impl Datepicker {
    pub fn hint(mut self, s: impl Into<String>) -> Self { self.hint = Some(s.into()); self }
    pub fn error(mut self, msg: impl Into<String>) -> Self {
        self.invalid = true; self.hint = Some(msg.into()); self
    }
    pub fn maybe_error(self, msg: Option<impl Into<String>>) -> Self {
        match msg { Some(m) => self.error(m), None => self }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_render_empty_tag() {
        let html = datepicker().render();
        assert_eq!(html, r#"<ui-datepicker></ui-datepicker>"#);
    }

    #[test]
    fn full_field() {
        let html = datepicker()
            .label("DOB").name("dob").value("2020-01-01").placeholder("YYYY-MM-DD").render();
        assert_eq!(
            html,
            r#"<ui-datepicker label="DOB" name="dob" value="2020-01-01" placeholder="YYYY-MM-DD"></ui-datepicker>"#
        );
    }

    #[test]
    fn error_replaces_hint() {
        let html = datepicker().hint("optional").error("Required").render();
        assert_eq!(html, r#"<ui-datepicker hint="Required" invalid></ui-datepicker>"#);
    }
}
