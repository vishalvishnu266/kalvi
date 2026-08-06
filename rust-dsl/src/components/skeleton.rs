//! `<ui-skeleton>` typed builder — refactored onto `#[derive(UiComponent)]`.
//!
//! Interesting bits vs. `badge`:
//! * `width`/`height` are `Option<String>` — the derive automatically
//!   skips them when `None` (no field-level attribute needed).
//! * `lines: u32` has a default of `1` and only renders when non-default;
//!   `skip_if = "self.lines == 1"` expresses that in one line.
//! * `Shape` uses `AttrEnum` and picks `Line` as the default variant.

use crate::core::Component;
use lit_ui_macros::{AttrEnum, UiComponent};

#[derive(AttrEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    #[attr("line")] #[attr_enum(default)] Line,
    #[attr("rect")] Rect,
    #[attr("circle")] Circle,
}

#[derive(UiComponent)]
#[ui(tag = "ui-skeleton")]
pub struct Skeleton {
    #[ui(enum_attr = "shape")]                          pub shape: Shape,
    #[ui(attr = "width")]                               pub width: Option<String>,
    #[ui(attr = "height")]                              pub height: Option<String>,
    #[ui(attr = "lines", default = "1", skip_if = "self.lines == 1")]
                                                        pub lines: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_single_line_shape() {
        let html = skeleton().render();
        // Default: shape=line (always emitted, since not marked skip_if),
        // width/height omitted (Option::None), lines=1 skipped.
        assert_eq!(html, r#"<ui-skeleton shape="line"></ui-skeleton>"#);
    }

    #[test]
    fn width_and_height_render_when_set() {
        let html = skeleton().width("120px").height("16px").render();
        assert_eq!(
            html,
            r#"<ui-skeleton shape="line" width="120px" height="16px"></ui-skeleton>"#
        );
    }

    #[test]
    fn lines_render_only_when_greater_than_one() {
        let html_one  = skeleton().lines(1).render();
        let html_many = skeleton().lines(3).render();
        assert_eq!(html_one,  r#"<ui-skeleton shape="line"></ui-skeleton>"#);
        assert_eq!(html_many, r#"<ui-skeleton shape="line" lines="3"></ui-skeleton>"#);
    }

    #[test]
    fn shape_variants_render() {
        let rect   = skeleton().shape(Shape::Rect).render();
        let circle = skeleton().shape(Shape::Circle).render();
        assert_eq!(rect,   r#"<ui-skeleton shape="rect"></ui-skeleton>"#);
        assert_eq!(circle, r#"<ui-skeleton shape="circle"></ui-skeleton>"#);
    }
}
