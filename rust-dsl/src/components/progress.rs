//! `<ui-progress>` typed builder — determinate and indeterminate bars.
//!
//! ```ignore
//! use lit_ui::prelude::*;
//! progress().value(65).tone(ProgressTone::Brand).label("Uploading…");
//! progress().indeterminate();
//! ```

#[allow(unused_imports)]
use crate::core::Component;
use lit_ui_macros::{AttrEnum, UiComponent};

#[derive(AttrEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgressTone {
    #[attr("neutral")] #[attr_enum(default)] Neutral,
    #[attr("brand")]   Brand,
    #[attr("success")] Success,
    #[attr("warning")] Warning,
    #[attr("danger")]  Danger,
}

#[derive(UiComponent)]
#[ui(tag = "ui-progress")]
pub struct Progress {
    #[ui(attr = "value", default = "0")]                pub value: u32,
    #[ui(attr = "max",   default = "100")]              pub max: u32,
    #[ui(enum_attr = "tone", skip_if = "self.tone == ProgressTone::Neutral")]
                                                        pub tone: ProgressTone,
    #[ui(attr = "label")]                               pub label: Option<String>,
    #[ui(flag = "indeterminate")]                       pub indeterminate: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_render_value_max() {
        // Neutral tone is default → skipped. No label, not indeterminate.
        let html = progress().render();
        assert_eq!(
            html,
            r#"<ui-progress value="0" max="100"></ui-progress>"#
        );
    }

    #[test]
    fn determinate_with_tone_and_label() {
        let html = progress().value(65).tone(ProgressTone::Brand).label("Uploading").render();
        assert_eq!(
            html,
            r#"<ui-progress value="65" max="100" tone="brand" label="Uploading"></ui-progress>"#
        );
    }

    #[test]
    fn indeterminate() {
        let html = progress().indeterminate().render();
        assert_eq!(
            html,
            r#"<ui-progress value="0" max="100" indeterminate></ui-progress>"#
        );
    }
}
