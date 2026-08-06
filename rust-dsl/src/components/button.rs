//! `<ui-button>` — Vaadin-style typed builder, refactored onto the derive.
//!
//! Notable per-field choices:
//! * `variant`, `size`, `kind` use `enum_attr` so they render via the
//!   enum's `as_str()` (produced by `#[derive(AttrEnum)]`).
//! * `icon` uses `no_setter` so we can hand-write a setter that accepts
//!   anything implementing `IntoIconName` (typed constants OR strings).
//! * `label` + `children` render in that order in the body, matching the
//!   pre-refactor output byte-for-byte.
//!
//! ```ignore
//! use lit_ui::prelude::*;
//!
//! let html = button()
//!     .label("Save")
//!     .variant(Variant::Primary)
//!     .size(Size::Md)
//!     .icon(Icons::CHECK)
//!     .render();
//! ```

use crate::components::icon::IntoIconName;
#[allow(unused_imports)]
use crate::core::{Child, Component};
use lit_ui_macros::{AttrEnum, UiComponent};

/// Visual variant of a button.
#[derive(AttrEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    #[attr("primary")] #[attr_enum(default)] Primary,
    #[attr("secondary")] Secondary,
    #[attr("ghost")]     Ghost,
    #[attr("danger")]    Danger,
}

/// Button size.
#[derive(AttrEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Size {
    #[attr("sm")] Sm,
    #[attr("md")] #[attr_enum(default)] Md,
    #[attr("lg")] Lg,
}

/// Native `type` attribute on the underlying `<button>`. Defaults to
/// `Button` to match the HTML default (no accidental form submits).
#[derive(AttrEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonType {
    #[attr("button")] #[attr_enum(default)] Button,
    #[attr("submit")] Submit,
    #[attr("reset")]  Reset,
}

/// `<ui-button>` builder.
#[derive(UiComponent)]
#[ui(tag = "ui-button")]
pub struct Button {
    #[ui(slot)]                                  pub label: String,
    #[ui(enum_attr = "variant")]                 pub variant: Variant,
    #[ui(enum_attr = "size")]                    pub size: Size,
    #[ui(enum_attr = "type")]                    pub kind: ButtonType,
    #[ui(attr = "icon", no_setter)]              pub icon: Option<String>,
    #[ui(flag = "full")]                         pub full: bool,
    #[ui(flag = "disabled")]                     pub disabled: bool,
    #[ui(children)]                              pub children: Vec<Child>,
}

impl Button {
    /// Attach an icon. Accepts a typed [`crate::components::icon::IconName`]
    /// constant (e.g. `Icons::CHECK`) or a raw string.
    pub fn icon(mut self, name: impl IntoIconName) -> Self {
        self.icon = Some(name.into_icon_name()); self
    }
    /// Shorthand for `.kind(ButtonType::Submit)` — reads well in chains.
    pub fn submit(self) -> Self { self.kind(ButtonType::Submit) }
    /// Shorthand for `.kind(ButtonType::Reset)`.
    pub fn reset(self)  -> Self { self.kind(ButtonType::Reset) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults() {
        let html = button().label("Save").render();
        assert_eq!(
            html,
            r#"<ui-button variant="primary" size="md" type="button">Save</ui-button>"#
        );
    }

    #[test]
    fn all_attrs() {
        let html = button()
            .label("Delete")
            .variant(Variant::Danger)
            .size(Size::Lg)
            .icon("trash")
            .full()
            .disabled()
            .submit()
            .render();
        assert_eq!(
            html,
            r#"<ui-button variant="danger" size="lg" type="submit" icon="trash" full disabled>Delete</ui-button>"#
        );
    }

    #[test]
    fn label_escaped() {
        let html = button().label("<x>").render();
        assert_eq!(
            html,
            r#"<ui-button variant="primary" size="md" type="button">&lt;x&gt;</ui-button>"#
        );
    }
}
