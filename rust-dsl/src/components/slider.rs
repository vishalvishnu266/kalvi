//! `<ui-slider>` typed builder — a themed range input primitive.
//!
//! ```ignore
//! use lit_ui::prelude::*;
//! slider().name("volume").min(0).max(100).value(42).show_value();
//! ```

#[allow(unused_imports)]
use crate::core::Component;
use lit_ui_macros::UiComponent;

#[derive(UiComponent)]
#[ui(tag = "ui-slider")]
pub struct Slider {
    #[ui(attr = "name")]                                pub name: Option<String>,
    #[ui(attr = "min",  default = "0")]                 pub min: i64,
    #[ui(attr = "max",  default = "100")]               pub max: i64,
    // `step` defaults to 1; skipped on the wire when 1 to match native default.
    #[ui(attr = "step", default = "1", skip_if = "self.step == 1")]
                                                        pub step: i64,
    #[ui(attr = "value", default = "0")]                pub value: i64,
    #[ui(flag = "show-value")]                          pub show_value: bool,
    #[ui(flag = "disabled")]                            pub disabled: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_render_min_max_value() {
        // step=1 is skipped, show-value/disabled omitted.
        let html = slider().render();
        assert_eq!(
            html,
            r#"<ui-slider min="0" max="100" value="0"></ui-slider>"#
        );
    }

    #[test]
    fn full_config() {
        let html = slider()
            .name("volume").min(0).max(100).step(5).value(42).show_value().disabled().render();
        assert_eq!(
            html,
            r#"<ui-slider name="volume" min="0" max="100" step="5" value="42" show-value disabled></ui-slider>"#
        );
    }
}
