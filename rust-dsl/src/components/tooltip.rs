//! `<ui-tooltip>` typed builder — refactored onto `#[derive(UiComponent)]`.
//!
//! Uses `#[ui(children)]` to auto-derive `.add()` / `.children()` from the
//! `Vec<Child>` field.

#[allow(unused_imports)]
use crate::core::{Child, Component};
use lit_ui_macros::{AttrEnum, UiComponent};

#[derive(AttrEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Placement {
    #[attr("top")] #[attr_enum(default)] Top,
    #[attr("bottom")] Bottom,
    #[attr("left")]   Left,
    #[attr("right")]  Right,
}

/// Custom `tooltip(text)` constructor — the tooltip text is required.
pub fn tooltip(text: impl Into<String>) -> Tooltip {
    let mut t = <Tooltip as Default>::default();
    t.text = text.into();
    t
}

#[derive(UiComponent)]
#[ui(tag = "ui-tooltip", no_ctor)]
pub struct Tooltip {
    #[ui(attr = "text")]                       pub text: String,
    #[ui(enum_attr = "placement")]             pub placement: Placement,
    #[ui(attr = "delay", default = "250")]     pub delay: u32,
    #[ui(children)]                            pub children: Vec<Child>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults() {
        let html = tooltip("Save").render();
        assert_eq!(
            html,
            r#"<ui-tooltip text="Save" placement="top" delay="250"></ui-tooltip>"#
        );
    }

    #[test]
    fn placement_and_delay() {
        let html = tooltip("Save").placement(Placement::Bottom).delay(500).render();
        assert_eq!(
            html,
            r#"<ui-tooltip text="Save" placement="bottom" delay="500"></ui-tooltip>"#
        );
    }
}
