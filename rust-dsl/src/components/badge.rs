//! `<ui-badge>` typed builder — refactored to use `#[derive(UiComponent)]`
//! and `#[derive(AttrEnum)]`. Compare with the pre-refactor `checkbox.rs`
//! to see how much ceremony these two derives eliminate.
//!
//! The `AttrEnum` derive turns `#[attr("...")]` on each variant into a
//! `Tone::as_str(self) -> &'static str` method. The `UiComponent` derive
//! reads `#[ui(...)]` on each field and generates:
//! * the setter (`.tone(t)`, `.dot()`),
//! * `Default` (via per-field `default = "..."` or the type's `Default`),
//! * a free constructor (`badge()` → `Badge::default()`),
//! * `impl Component` — i.e. the whole `render()` body.
//!
//! `skip_if = "self.tone == Tone::Neutral"` reproduces the original
//! "don't emit `tone` when it's the default" behaviour byte-for-byte.

use crate::core::Component;
use lit_ui_macros::{AttrEnum, UiComponent};

#[derive(AttrEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tone {
    #[attr("neutral")] #[attr_enum(default)] Neutral,
    #[attr("brand")]   Brand,
    #[attr("success")] Success,
    #[attr("warning")] Warning,
    #[attr("danger")]  Danger,
    #[attr("info")]    Info,
}

/// The label is a required constructor arg in the old API, so we keep a
/// convenience free function that takes it. The derive still generates a
/// `badge()` no-arg constructor, but downstream code should prefer the
/// `badge(label)` form for readability.
pub fn badge(label: impl Into<String>) -> Badge {
    let mut b = <Badge as Default>::default();
    b.label = label.into();
    b
}

// `#[ui(no_ctor)]` because the module supplies a custom `badge(label)`
// constructor above that takes the required label directly.
#[derive(UiComponent)]
#[ui(tag = "ui-badge", no_ctor)]
pub struct Badge {
    #[ui(slot)]                                                                 pub label: String,
    #[ui(enum_attr = "tone", skip_if = "self.tone == Tone::Neutral")]           pub tone: Tone,
    #[ui(flag = "dot")]                                                         pub dot: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    // Golden-file snapshots. These match the byte-exact output of the
    // pre-refactor `impl Component` so any regression is caught immediately.

    #[test]
    fn default_neutral_no_dot() {
        // Old render: no `tone`, no `dot`, escaped label.
        let html = badge("Live").render();
        assert_eq!(html, r#"<ui-badge>Live</ui-badge>"#);
    }

    #[test]
    fn brand_tone_reflected() {
        let html = badge("New").tone(Tone::Brand).render();
        assert_eq!(html, r#"<ui-badge tone="brand">New</ui-badge>"#);
    }

    #[test]
    fn dot_flag_reflected() {
        let html = badge("Alerts").dot().render();
        assert_eq!(html, r#"<ui-badge dot>Alerts</ui-badge>"#);
    }

    #[test]
    fn tone_and_dot_together() {
        let html = badge("Errors").tone(Tone::Danger).dot().render();
        assert_eq!(html, r#"<ui-badge tone="danger" dot>Errors</ui-badge>"#);
    }

    #[test]
    fn label_is_html_escaped() {
        let html = badge("<b>x</b>").render();
        assert_eq!(html, r#"<ui-badge>&lt;b&gt;x&lt;/b&gt;</ui-badge>"#);
    }
}
