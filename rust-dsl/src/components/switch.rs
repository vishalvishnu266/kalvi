//! `<ui-switch>` typed builder — refactored onto `#[derive(UiComponent)]`.

#[allow(unused_imports)]
use crate::core::Component;
use lit_ui_macros::UiComponent;

pub fn switch(label: impl Into<String>) -> Switch {
    let mut s = <Switch as Default>::default();
    s.label = label.into();
    s
}

#[derive(UiComponent)]
#[ui(tag = "ui-switch", no_ctor)]
pub struct Switch {
    #[ui(slot)]                                pub label: String,
    #[ui(attr = "name")]                       pub name: Option<String>,
    #[ui(attr = "value")]                      pub value: Option<String>,
    #[ui(attr = "error", no_setter)]           pub error: Option<String>,
    #[ui(flag = "checked")]                    pub checked: bool,
    #[ui(flag = "disabled")]                   pub disabled: bool,
    #[ui(flag = "invalid")]                    pub invalid: bool,
}

impl Switch {
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
        let html = switch("Enable").render();
        assert_eq!(html, r#"<ui-switch>Enable</ui-switch>"#);
    }

    #[test]
    fn checked_disabled() {
        let html = switch("Live").checked().disabled().render();
        assert_eq!(html, r#"<ui-switch checked disabled>Live</ui-switch>"#);
    }

    #[test]
    fn error_sets_both() {
        let html = switch("x").error("Must be on").render();
        assert_eq!(
            html,
            r#"<ui-switch error="Must be on" invalid>x</ui-switch>"#
        );
    }
}
