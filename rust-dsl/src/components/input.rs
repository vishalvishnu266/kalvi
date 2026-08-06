//! `<ui-input>` — Vaadin-style typed builder, refactored onto the derive.
//!
//! Key patterns exercised here:
//! * `kind: InputType` renders as `type="…"` via `enum_attr`, matching the
//!   pre-refactor byte-for-byte output.
//! * `icon_leading` / `icon_trailing` accept `impl IntoIconName` (typed or
//!   raw), so they use `no_setter` and hand-written setters below.
//! * `hint` uses `no_setter` so we can offer a plain `.hint()` alongside
//!   `.error()` (which mutates two fields).
//!
//! ```ignore
//! use lit_ui::prelude::*;
//! input().label("Full name").name("fullName").placeholder("e.g. Aarav").required();
//! ```

use crate::components::icon::IntoIconName;
#[allow(unused_imports)]
use crate::core::Component;
use lit_ui_macros::{AttrEnum, UiComponent};

#[derive(AttrEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputType {
    #[attr("text")] #[attr_enum(default)] Text,
    #[attr("password")] Password,
    #[attr("email")]    Email,
    #[attr("number")]   Number,
    #[attr("textarea")] Textarea,
}

#[derive(UiComponent)]
#[ui(tag = "ui-input")]
pub struct Input {
    // Wire order matches the pre-refactor render: `type` first, then
    // label / name / value / placeholder / hint, then icon-leading /
    // icon-trailing, then required / invalid.
    #[ui(enum_attr = "type")]                       pub kind: InputType,
    #[ui(attr = "label")]                           pub label: Option<String>,
    #[ui(attr = "name")]                            pub name: Option<String>,
    #[ui(attr = "value")]                           pub value: Option<String>,
    #[ui(attr = "placeholder")]                     pub placeholder: Option<String>,
    #[ui(attr = "hint", no_setter)]                 pub hint: Option<String>,
    #[ui(attr = "icon-leading",  no_setter)]        pub icon_leading: Option<String>,
    #[ui(attr = "icon-trailing", no_setter)]        pub icon_trailing: Option<String>,
    #[ui(flag = "required")]                        pub required: bool,
    #[ui(flag = "invalid")]                         pub invalid: bool,
}

impl Input {
    pub fn hint(mut self, s: impl Into<String>) -> Self { self.hint = Some(s.into()); self }
    /// Leading icon. Accepts a typed `IconName` or a raw string.
    pub fn icon_leading(mut self, name: impl IntoIconName) -> Self {
        self.icon_leading = Some(name.into_icon_name()); self
    }
    /// Trailing icon. Accepts a typed `IconName` or a raw string.
    pub fn icon_trailing(mut self, name: impl IntoIconName) -> Self {
        self.icon_trailing = Some(name.into_icon_name()); self
    }
    /// Field-level error — sets `invalid` and replaces the hint so it
    /// renders red under the field.
    pub fn error(mut self, msg: impl Into<String>) -> Self {
        self.invalid = true; self.hint = Some(msg.into()); self
    }
    /// `Option<msg>` convenience so server code doesn't need a match.
    pub fn maybe_error(self, msg: Option<impl Into<String>>) -> Self {
        match msg { Some(m) => self.error(m), None => self }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_type_text_only() {
        let html = input().render();
        assert_eq!(html, r#"<ui-input type="text"></ui-input>"#);
    }

    #[test]
    fn full_field() {
        let html = input()
            .label("Email")
            .name("email")
            .kind(InputType::Email)
            .placeholder("you@x.com")
            .required()
            .render();
        assert_eq!(
            html,
            r#"<ui-input type="email" label="Email" name="email" placeholder="you@x.com" required></ui-input>"#
        );
    }

    #[test]
    fn error_sets_invalid_and_replaces_hint() {
        let html = input().label("Email").hint("optional").error("Required").render();
        assert_eq!(
            html,
            r#"<ui-input type="text" label="Email" hint="Required" invalid></ui-input>"#
        );
    }

    #[test]
    fn icons_accepted() {
        let html = input().icon_leading("search").icon_trailing("x").render();
        assert_eq!(
            html,
            r#"<ui-input type="text" icon-leading="search" icon-trailing="x"></ui-input>"#
        );
    }
}
