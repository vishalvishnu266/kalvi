//! `<ui-color-swatch>` typed builder — a single circular colour dot.
//!
//! Compose many inside a `<ui-cluster>` to build palettes:
//!
//! ```ignore
//! use lit_ui::prelude::*;
//! color_swatch("#4f46e5").selectable().selected();
//! ```

#[allow(unused_imports)]
use crate::core::Component;
use lit_ui_macros::{AttrEnum, UiComponent};

#[derive(AttrEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwatchSize {
    #[attr("sm")] Sm,
    #[attr("md")] #[attr_enum(default)] Md,
    #[attr("lg")] Lg,
}

/// Custom `color_swatch(color)` constructor — the colour is required.
pub fn color_swatch(color: impl Into<String>) -> ColorSwatch {
    let mut s = <ColorSwatch as Default>::default();
    s.color = color.into();
    s
}

#[derive(UiComponent)]
#[ui(tag = "ui-color-swatch", no_ctor)]
pub struct ColorSwatch {
    #[ui(attr = "color")]           pub color: String,
    #[ui(enum_attr = "size")]       pub size: SwatchSize,
    #[ui(flag = "selectable")]      pub selectable: bool,
    #[ui(flag = "selected")]        pub selected: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_render_color_and_size() {
        let html = color_swatch("#4f46e5").render();
        assert_eq!(
            html,
            r#"<ui-color-swatch color="#4f46e5" size="md"></ui-color-swatch>"#
        );
    }

    #[test]
    fn selectable_and_selected() {
        let html = color_swatch("red").size(SwatchSize::Lg).selectable().selected().render();
        assert_eq!(
            html,
            r#"<ui-color-swatch color="red" size="lg" selectable selected></ui-color-swatch>"#
        );
    }

    #[test]
    fn color_html_escaped() {
        // Not something you'd ever write, but proves attribute escaping works.
        let html = color_swatch(r#""&<>"#).render();
        assert_eq!(
            html,
            r#"<ui-color-swatch color="&quot;&amp;&lt;&gt;" size="md"></ui-color-swatch>"#
        );
    }
}
