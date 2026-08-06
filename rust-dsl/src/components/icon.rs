//! `<ui-icon>` typed builder + a typed catalogue of well-known icon names.
//!
//! Refactored onto `#[derive(UiComponent)]`. The catalogue and the
//! `IntoIconName` trait are unchanged from the pre-macro version — only
//! the `Icon` builder itself is shrunk down.
//!
//! Two ways to use it:
//! ```ignore
//! use lit_ui::prelude::*;
//! button().label("Save").icon(Icons::CHECK);   // typed
//! button().label("Save").icon("check");        // stringy
//! ```

#[allow(unused_imports)]
use crate::core::Component;
use lit_ui_macros::UiComponent;

/// A well-known icon identifier. Wraps a `&'static str` so it costs nothing
/// at runtime but gives you typed autocomplete and rename-safety.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IconName(pub &'static str);
impl IconName {
    pub fn as_str(self) -> &'static str { self.0 }
}
impl From<IconName> for String {
    fn from(i: IconName) -> String { i.0.to_string() }
}

/// Anything convertible to an icon name — a typed [`IconName`], a `&str`,
/// or a `String`. Lets `.icon(...)` accept all three.
pub trait IntoIconName {
    fn into_icon_name(self) -> String;
}
impl IntoIconName for IconName        { fn into_icon_name(self) -> String { self.0.to_string() } }
impl IntoIconName for &'static str    { fn into_icon_name(self) -> String { self.to_string()  } }
impl IntoIconName for String          { fn into_icon_name(self) -> String { self               } }
impl IntoIconName for &String         { fn into_icon_name(self) -> String { self.clone()       } }

/// Central catalogue of icons **that actually exist** in the underlying
/// `<ui-icon>` component (see `lit-components/components/primitives/ui-icon.js`,
/// the `PATHS` map).
#[allow(non_snake_case)]
pub mod Icons {
    use super::IconName;

    // ── Actions ──
    pub const CHECK:    IconName = IconName("check");
    pub const X:        IconName = IconName("x");
    pub const PLUS:     IconName = IconName("plus");
    pub const MINUS:    IconName = IconName("minus");
    pub const EDIT:     IconName = IconName("edit");
    pub const UPLOAD:   IconName = IconName("upload");
    pub const DOWNLOAD: IconName = IconName("download");
    pub const SAVE:     IconName = IconName("save");
    pub const DELETE:   IconName = IconName("trash");
    pub const SEARCH:   IconName = IconName("search");
    pub const FILTER:   IconName = IconName("filter");
    pub const SETTINGS: IconName = IconName("settings");
    pub const REFRESH:  IconName = IconName("refresh");
    pub const MORE:     IconName = IconName("more");
    pub const MENU:     IconName = IconName("menu");
    // ── Feedback ──
    pub const INFO:     IconName = IconName("info");
    pub const WARNING:  IconName = IconName("warning");
    pub const BELL:     IconName = IconName("bell");
    // ── Navigation ──
    pub const HOME:          IconName = IconName("home");
    pub const CHEVRON_DOWN:  IconName = IconName("chevronDown");
    pub const CHEVRON_UP:    IconName = IconName("chevronUp");
    pub const CHEVRON_LEFT:  IconName = IconName("chevronLeft");
    pub const CHEVRON_RIGHT: IconName = IconName("chevronRight");
    pub const ARROW_LEFT:    IconName = IconName("arrowLeft");
    pub const ARROW_RIGHT:   IconName = IconName("arrowRight");
    pub const ARROW_UP:      IconName = IconName("arrowUp");
    pub const ARROW_DOWN:    IconName = IconName("arrowDown");
    // ── Theme ──
    pub const SUN:      IconName = IconName("sun");
    pub const MOON:     IconName = IconName("moon");
    // ── Domain ──
    pub const USERS:     IconName = IconName("users");
    pub const STUDENT:   IconName = IconName("student");
    pub const CALENDAR:  IconName = IconName("calendar");
    pub const MESSAGE:   IconName = IconName("message");
    pub const BOOKMARK:  IconName = IconName("bookmark");
    pub const LIBRARY:   IconName = IconName("library");
    pub const CLIPBOARD: IconName = IconName("clipboard");
    pub const CARD:      IconName = IconName("card");
    pub const WALLET:    IconName = IconName("wallet");
    pub const CHART:     IconName = IconName("chart");
    pub const GRID:      IconName = IconName("grid");
    pub const ACTIVITY:  IconName = IconName("activity");
    pub const MAIL:      IconName = IconName("mail");
    pub const CLOCK:     IconName = IconName("clock");
    pub const FILE:      IconName = IconName("file");
    pub const STAR:      IconName = IconName("star");
    // Legacy alias.
    pub const DASHBOARD: IconName = GRID;
}

// ── Component ──

/// Custom `icon(name)` constructor: accepts anything `IntoIconName`, so
/// we opt out of the derived free ctor via `#[ui(no_ctor)]`.
pub fn icon(name: impl IntoIconName) -> Icon {
    let mut i = <Icon as Default>::default();
    i.name = name.into_icon_name();
    i
}

#[derive(UiComponent)]
#[ui(tag = "ui-icon", no_ctor)]
pub struct Icon {
    #[ui(attr = "name")]                pub name: String,
    #[ui(attr = "size", default = "18")] pub size: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_size_18() {
        let html = icon(Icons::CHECK).render();
        assert_eq!(html, r#"<ui-icon name="check" size="18"></ui-icon>"#);
    }

    #[test]
    fn accepts_string() {
        let html = icon("info").render();
        assert_eq!(html, r#"<ui-icon name="info" size="18"></ui-icon>"#);
    }

    #[test]
    fn size_override() {
        let html = icon(Icons::SEARCH).size(24).render();
        assert_eq!(html, r#"<ui-icon name="search" size="24"></ui-icon>"#);
    }
}
