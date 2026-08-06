//! `<ui-heading>` typed builder — a simple typographic heading primitive.
//!
//! ```ignore
//! use lit_ui::prelude::*;
//! heading("Section").h2();
//! heading("Notice").h3().tone(HeadingTone::Warning);
//! ```

#[allow(unused_imports)]
use crate::core::Component;
use lit_ui_macros::{AttrEnum, UiComponent};

#[derive(AttrEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeadingLevel {
    #[attr("h1")] #[attr_enum(default)] H1,
    #[attr("h2")] H2,
    #[attr("h3")] H3,
    #[attr("h4")] H4,
}

#[derive(AttrEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeadingTone {
    #[attr("default")] #[attr_enum(default)] Default,
    #[attr("brand")]   Brand,
    #[attr("muted")]   Muted,
    #[attr("success")] Success,
    #[attr("warning")] Warning,
    #[attr("danger")]  Danger,
}

/// Custom `heading(text)` constructor — the text is required.
pub fn heading(text: impl Into<String>) -> Heading {
    let mut h = <Heading as Default>::default();
    h.text = text.into();
    h
}

#[derive(UiComponent)]
#[ui(tag = "ui-heading", no_ctor)]
pub struct Heading {
    #[ui(slot)]                                                             pub text: String,
    #[ui(enum_attr = "level", skip_if = "self.level == HeadingLevel::H1")]  pub level: HeadingLevel,
    #[ui(enum_attr = "tone",  skip_if = "self.tone == HeadingTone::Default")] pub tone: HeadingTone,
}

impl Heading {
    /// Shorthand for `.level(HeadingLevel::H1)` — reads well in chains.
    pub fn h1(self) -> Self { self.level(HeadingLevel::H1) }
    pub fn h2(self) -> Self { self.level(HeadingLevel::H2) }
    pub fn h3(self) -> Self { self.level(HeadingLevel::H3) }
    pub fn h4(self) -> Self { self.level(HeadingLevel::H4) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_h1_no_tone() {
        assert_eq!(heading("Hi").render(), "<ui-heading>Hi</ui-heading>");
    }

    #[test]
    fn levels_render_when_non_default() {
        assert_eq!(heading("a").h2().render(), r#"<ui-heading level="h2">a</ui-heading>"#);
        assert_eq!(heading("a").h3().render(), r#"<ui-heading level="h3">a</ui-heading>"#);
        assert_eq!(heading("a").h4().render(), r#"<ui-heading level="h4">a</ui-heading>"#);
    }

    #[test]
    fn tone_and_level_combine() {
        let html = heading("Warn").h2().tone(HeadingTone::Warning).render();
        assert_eq!(html, r#"<ui-heading level="h2" tone="warning">Warn</ui-heading>"#);
    }

    #[test]
    fn text_is_html_escaped() {
        assert_eq!(
            heading("<x>&y").render(),
            r#"<ui-heading>&lt;x&gt;&amp;y</ui-heading>"#,
        );
    }
}
