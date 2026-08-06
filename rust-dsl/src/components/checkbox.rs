//! `<ui-checkbox>` typed builder — refactored onto `#[derive(UiComponent)]`.
//!
//! `error` uses `#[ui(no_setter)]` so the derive still renders it but
//! doesn't generate a public setter. That leaves `.error(msg)` (defined
//! by hand below) as the only `.error()` on the type — the same two-field
//! mutation the pre-refactor API had.

#[allow(unused_imports)]
use crate::core::Component;
use lit_ui_macros::UiComponent;

/// Custom `checkbox(label)` constructor — the label is required.
pub fn checkbox(label: impl Into<String>) -> Checkbox {
    let mut c = <Checkbox as Default>::default();
    c.label = label.into();
    c
}

#[derive(UiComponent)]
#[ui(tag = "ui-checkbox", no_ctor)]
pub struct Checkbox {
    #[ui(slot)]                                pub label: String,
    #[ui(attr = "name")]                       pub name: Option<String>,
    #[ui(attr = "value")]                      pub value: Option<String>,
    #[ui(attr = "error", no_setter)]           pub error: Option<String>,
    #[ui(flag = "checked")]                    pub checked: bool,
    #[ui(flag = "indeterminate")]              pub indeterminate: bool,
    #[ui(flag = "disabled")]                   pub disabled: bool,
    #[ui(flag = "required")]                   pub required: bool,
    #[ui(flag = "invalid")]                    pub invalid: bool,
}

impl Checkbox {
    /// Field-level error — sets `invalid` and passes the message via the
    /// `error` attribute. Two-field mutation, hand-written on purpose.
    pub fn error(mut self, msg: impl Into<String>) -> Self {
        self.invalid = true; self.error = Some(msg.into()); self
    }
    pub fn maybe_error(self, msg: Option<impl Into<String>>) -> Self {
        match msg { Some(m) => self.error(m), None => self }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_render() {
        let html = checkbox("Agree").render();
        assert_eq!(html, r#"<ui-checkbox>Agree</ui-checkbox>"#);
    }

    #[test]
    fn all_flags() {
        let html = checkbox("x").checked().indeterminate().disabled().required().invalid().render();
        assert_eq!(
            html,
            r#"<ui-checkbox checked indeterminate disabled required invalid>x</ui-checkbox>"#
        );
    }

    #[test]
    fn name_value() {
        let html = checkbox("Agree").name("agree").value("y").render();
        assert_eq!(
            html,
            r#"<ui-checkbox name="agree" value="y">Agree</ui-checkbox>"#
        );
    }

    #[test]
    fn error_sets_invalid_and_error_attr() {
        let html = checkbox("Agree").error("Required").render();
        assert_eq!(
            html,
            r#"<ui-checkbox error="Required" invalid>Agree</ui-checkbox>"#
        );
    }
}
