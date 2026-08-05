//! # Apps registry — the OS-like "installed apps" list.
//!
//! One flat vec of [`App`] entries. This is the single source of truth
//! for:
//!
//! * The **desktop activity bar** on the left of the shell.
//! * The **mobile bottom tab bar** (first N icons, rest go into "more").
//! * The **copilot's `go to X` fuzzy match** (looked up by `label` or
//!   the `route`'s last segment).
//!
//! Adding a new app is a one-liner in [`all`]. No wiring changes
//! needed — the activity bar, tab bar, and copilot all pick it up on
//! next request.
//!
//! The design is deliberately tiny: `route + icon + label`. If an app
//! ever needs its own sub-nav, per-app commands, or per-app palette,
//! we can extend [`App`] later without breaking today's call sites.

/// One installed "app". Kept as a plain struct so it's easy to grep,
/// easy to serialise if we ever want a JSON manifest, and cheap to
/// clone (all fields are `&'static str`).
#[derive(Debug, Clone, Copy)]
pub struct App {
    /// Stable, machine-friendly id (kebab-case). Handy for `data-app-id`
    /// attributes, analytics, and per-app CSS targeting.
    pub id: &'static str,
    /// The `<ui-icon name="…">` value. Must exist in the icon set
    /// shipped by `lit-components/components/ui-icon.js`.
    pub icon: &'static str,
    /// Human label — shown as caption under the icon on the /apps
    /// launcher page and used for launcher fuzzy matching.
    pub label: &'static str,
    /// The route this app opens. Must be registered in `main.rs`.
    pub route: &'static str,
    /// Brand color for the /apps launcher icon tile. iOS-home style: a
    /// tinted background of this color with the glyph in the saturated
    /// version. Hex, e.g. "#4f46e5".
    pub color: &'static str,
    /// If true, appears in the mobile bottom tab bar; false = overflow
    /// into a "more" bucket. Kept for future use; unused today.
    pub primary: bool,
}

/// The registered apps. Order is the display order in the activity bar.
///
/// Marked `primary` = top 5 apps that appear in the mobile tab bar.
/// The rest overflow into a "more" sheet — implemented later once the
/// list actually grows past 5 primaries.
pub fn all() -> &'static [App] {
    &[
        App { id: "home",      icon: "home",      label: "Home",      route: "/",          color: "#0ea5e9", primary: true  },
        App { id: "dashboard", icon: "chart",     label: "Dashboard", route: "/dashboard", color: "#8b5cf6", primary: true  },
        App { id: "users",     icon: "users",     label: "Users",     route: "/users",     color: "#f59e0b", primary: true  },
        App { id: "reports",   icon: "clipboard", label: "Reports",   route: "/reports",   color: "#10b981", primary: true  },
        App { id: "admin",     icon: "grid",      label: "Admin",     route: "/admin",     color: "#ef4444", primary: true  },
        App { id: "settings",  icon: "settings",  label: "Settings",  route: "/settings",  color: "#64748b", primary: false },
    ]
}

// NOTE: `mobile_primary()` was removed alongside the old bottom app-tab
// bar. The mobile bottom bar now hosts the fixed 4-icon primary bar
// (Apps · Search · AI · Profile), not the app list. If we ever want a
// mobile "pinned apps" strip inside the /apps launcher, restore this
// helper — it filtered by `App::primary` with a hard cap of 5.
