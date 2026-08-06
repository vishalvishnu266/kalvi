//! `<ui-daterange>` typed builder — refactored onto `#[derive(UiComponent)]`.

#[allow(unused_imports)]
use crate::core::Component;
use lit_ui_macros::UiComponent;

// Custom constructor preserves the original module-level name `date_range`
// (not `daterange`), so consumer code doesn't need to change.
pub fn date_range() -> DateRange { <DateRange as Default>::default() }

#[derive(UiComponent)]
#[ui(tag = "ui-daterange", no_ctor)]
pub struct DateRange {
    #[ui(attr = "label")]              pub label: Option<String>,
    #[ui(attr = "from")]               pub from: Option<String>,
    #[ui(attr = "to")]                 pub to: Option<String>,
    #[ui(attr = "hint", no_setter)]    pub hint: Option<String>,
    #[ui(flag = "invalid")]            pub invalid: bool,
}

impl DateRange {
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
        let html = date_range().render();
        assert_eq!(html, r#"<ui-daterange></ui-daterange>"#);
    }

    #[test]
    fn full_field() {
        let html = date_range().label("Range").from("2024-01-01").to("2024-12-31").render();
        assert_eq!(
            html,
            r#"<ui-daterange label="Range" from="2024-01-01" to="2024-12-31"></ui-daterange>"#
        );
    }

    #[test]
    fn error_replaces_hint() {
        let html = date_range().hint("optional").error("Required").render();
        assert_eq!(html, r#"<ui-daterange hint="Required" invalid></ui-daterange>"#);
    }
}
