//! `<ui-avatar>` typed builder — refactored onto `#[derive(UiComponent)]`.

#[allow(unused_imports)]
use crate::core::Component;
use lit_ui_macros::{AttrEnum, UiComponent};

#[derive(AttrEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum AvatarSize {
    #[attr("sm")] Sm,
    #[attr("md")] #[attr_enum(default)] Md,
    #[attr("lg")] Lg,
    #[attr("xl")] Xl,
}

/// Custom `avatar(name)` constructor — the name is required.
pub fn avatar(name: impl Into<String>) -> Avatar {
    let mut a = <Avatar as Default>::default();
    a.name = name.into();
    a
}

#[derive(UiComponent)]
#[ui(tag = "ui-avatar", no_ctor)]
pub struct Avatar {
    #[ui(attr = "name")]           pub name: String,
    #[ui(attr = "src")]            pub src: Option<String>,
    #[ui(enum_attr = "size")]      pub size: AvatarSize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_md_no_src() {
        let html = avatar("Ada").render();
        assert_eq!(html, r#"<ui-avatar name="Ada" size="md"></ui-avatar>"#);
    }

    #[test]
    fn with_src() {
        let html = avatar("Ada").src("/u/1.png").render();
        assert_eq!(
            html,
            r#"<ui-avatar name="Ada" src="/u/1.png" size="md"></ui-avatar>"#
        );
    }

    #[test]
    fn size_override() {
        let html = avatar("Ada").size(AvatarSize::Lg).render();
        assert_eq!(html, r#"<ui-avatar name="Ada" size="lg"></ui-avatar>"#);
    }
}
