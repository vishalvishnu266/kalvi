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

/// Central catalogue of icons **that actually exist** in the underlying
/// `<ui-icon>` component (see `lit-components/components/ui-icon.js`,
/// the `PATHS` map).
///
/// Every constant here is verified to render an SVG — any name not in
/// the JS map silently falls back to a "help" question mark. So we only
/// expose the ones we've hand-checked.
///
/// The string values use **camelCase** because that's how the JS map is
/// keyed (`chevronRight`, not `chevron-right`).
///
/// ## To add a new icon
/// 1. Add its SVG path to `PATHS` in `ui-icon.js`.
/// 2. Add a matching `pub const NAME: IconName = IconName("jsKey");` here.
/// 3. Keep the two in sync — the JS side is the source of truth.
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
    pub const BELL:     IconName = IconName("bell");     // notifications / late

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
    pub const CARD:      IconName = IconName("card");     // payments / fees
    pub const WALLET:    IconName = IconName("wallet");
    pub const CHART:     IconName = IconName("chart");
    pub const GRID:      IconName = IconName("grid");     // dashboard
    pub const ACTIVITY:  IconName = IconName("activity"); // timeline default
    pub const MAIL:      IconName = IconName("mail");
    pub const CLOCK:     IconName = IconName("clock");
    pub const FILE:      IconName = IconName("file");
    pub const STAR:      IconName = IconName("star");

    // Legacy alias so old code compiles — `DASHBOARD` maps to the closest
    // available icon (the grid).
    pub const DASHBOARD: IconName = GRID;
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
