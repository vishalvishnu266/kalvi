//! `<ui-icon>` typed builder + a typed catalogue of well-known icon names.
//!
//! Two ways to use it:
//!
//! ```ignore
//! use lit_ui::prelude::*;
//!
//! // 1. Typed constant — compile-time checked, autocompletes, no typos:
//! button().label("Save").icon(Icons::CHECK);
//!
//! // 2. Raw string — still works for one-off / dynamic names:
//! button().label("Save").icon("check");
//! ```
//!
//! To add a new icon: add one `pub const NAME: IconName = IconName("bs-name");`
//! to the [`Icons`] module below. The string is whatever your underlying icon
//! set (Bootstrap Icons in the Lit web components) expects.

use crate::core::{wrap, Attr, Component};

/// A well-known icon identifier. Wraps a `&'static str` so it costs nothing
/// at runtime but gives you typed autocomplete and rename-safety.
///
/// You can freely pass either an [`IconName`] constant from [`Icons`] or a
/// plain string to any DSL method that accepts an icon — see the
/// [`IntoIconName`] trait.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IconName(pub &'static str);

impl IconName {
    /// The underlying icon-set string (e.g. `"check"`, `"x"`, `"info-circle"`).
    pub fn as_str(self) -> &'static str { self.0 }
}

impl From<IconName> for String {
    fn from(i: IconName) -> String { i.0.to_string() }
}

/// Anything convertible to an icon name — a typed [`IconName`] constant,
/// a `&str`, or a `String`. Lets `.icon(...)` accept all three.
pub trait IntoIconName {
    fn into_icon_name(self) -> String;
}
impl IntoIconName for IconName        { fn into_icon_name(self) -> String { self.0.to_string() } }
impl IntoIconName for &'static str    { fn into_icon_name(self) -> String { self.to_string()  } }
impl IntoIconName for String          { fn into_icon_name(self) -> String { self               } }
impl IntoIconName for &String         { fn into_icon_name(self) -> String { self.clone()       } }

/// Central catalogue of standard icons used across the DSL.
///
/// Add new icons here (one line each) instead of scattering string
/// literals through the codebase.
#[allow(non_snake_case)]
pub mod Icons {
    use super::IconName;

    // ── Actions ──
    pub const CHECK:      IconName = IconName("check");
    pub const X:          IconName = IconName("x");
    pub const PLUS:       IconName = IconName("plus");
    pub const MINUS:      IconName = IconName("minus");
    pub const EDIT:       IconName = IconName("pencil");
    pub const DELETE:     IconName = IconName("trash");
    pub const SAVE:       IconName = IconName("save");
    pub const DOWNLOAD:   IconName = IconName("download");
    pub const UPLOAD:     IconName = IconName("upload");
    pub const SEARCH:     IconName = IconName("search");
    pub const FILTER:     IconName = IconName("filter");
    pub const REFRESH:    IconName = IconName("arrow-clockwise");
    pub const SETTINGS:   IconName = IconName("gear");
    pub const MORE:       IconName = IconName("three-dots");

    // ── Feedback ──
    pub const INFO:       IconName = IconName("info-circle");
    pub const WARNING:    IconName = IconName("exclamation-triangle");
    pub const SUCCESS:    IconName = IconName("check-circle");
    pub const ERROR:      IconName = IconName("x-circle");

    // ── Navigation ──
    pub const ARROW_LEFT:  IconName = IconName("arrow-left");
    pub const ARROW_RIGHT: IconName = IconName("arrow-right");
    pub const ARROW_UP:    IconName = IconName("arrow-up");
    pub const ARROW_DOWN:  IconName = IconName("arrow-down");
    pub const CHEVRON_DOWN: IconName = IconName("chevron-down");
    pub const HOME:       IconName = IconName("house");
    pub const MENU:       IconName = IconName("list");

    // ── Domain ──
    pub const USER:       IconName = IconName("person");
    pub const USERS:      IconName = IconName("people");
    pub const CALENDAR:   IconName = IconName("calendar");
    pub const CLOCK:      IconName = IconName("clock");
    pub const MAIL:       IconName = IconName("envelope");
    pub const FILE:       IconName = IconName("file-earmark");
    pub const DOCS:       IconName = IconName("book");
    pub const DASHBOARD:  IconName = IconName("grid-1x2-fill");
    pub const STAR:       IconName = IconName("star");
}

// ── Component ──

pub struct Icon { name: String, size: u32 }

/// Start building a `<ui-icon>`. Accepts an [`IconName`] constant or a string.
pub fn icon(name: impl IntoIconName) -> Icon {
    Icon { name: name.into_icon_name(), size: 18 }
}

impl Icon {
    pub fn size(mut self, px: u32) -> Self { self.size = px; self }
}

impl Component for Icon {
    fn render(&self) -> String {
        let attrs = [
            Attr::kv("name", self.name.as_str()),
            Attr::kv("size", self.size.to_string()),
        ];
        wrap("ui-icon", &attrs, "")
    }
}
