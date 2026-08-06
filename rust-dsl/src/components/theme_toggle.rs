//! `<ui-theme-toggle>` typed builder — a one-click light/dark switcher.
//!
//! Zero attributes today. We expose a Rust struct anyway so:
//! * call sites read `theme_toggle()` instead of a `Node::raw` string,
//! * any future attributes (labels, custom icons, side-preference) land
//!   as `.something(x)` chain calls without touching consumers.
//!
//! ```ignore
//! use lit_ui::prelude::*;
//! theme_toggle();  // drop anywhere, no config
//! ```

#[allow(unused_imports)]
use crate::core::Component;
use lit_ui_macros::UiComponent;

#[derive(UiComponent)]
#[ui(tag = "ui-theme-toggle")]
pub struct ThemeToggle {
    // Deliberately empty. See the module doc for the rationale.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_empty_tag() {
        let html = theme_toggle().render();
        assert_eq!(html, r#"<ui-theme-toggle></ui-theme-toggle>"#);
    }
}
