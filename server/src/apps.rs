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
    /// Human label — shown as tooltip on the activity bar (hover),
    /// as caption under the mobile tab icons, and used for copilot
    /// fuzzy matching.
    pub label: &'static str,
    /// The route this app opens. Must be registered in `main.rs`.
    pub route: &'static str,
    /// If true, appears in the mobile bottom tab bar; false = overflow
    /// into a "more" bucket. Keep at most 5 primary apps so the bar
    /// stays scannable on a phone (usual native-app convention).
    pub primary: bool,
}

/// The registered apps. Order is the display order in the activity bar.
///
/// Marked `primary` = top 5 apps that appear in the mobile tab bar.
/// The rest overflow into a "more" sheet — implemented later once the
/// list actually grows past 5 primaries.
pub fn all() -> &'static [App] {
    &[
        App { id: "home",      icon: "home",      label: "Home",      route: "/",          primary: true  },
        App { id: "dashboard", icon: "chart",     label: "Dashboard", route: "/dashboard", primary: true  },
        App { id: "users",     icon: "users",     label: "Users",     route: "/users",     primary: true  },
        App { id: "reports",   icon: "clipboard", label: "Reports",   route: "/reports",   primary: true  },
        App { id: "admin",     icon: "grid",      label: "Admin",     route: "/admin",     primary: true  },
        App { id: "settings",  icon: "settings",  label: "Settings",  route: "/settings",  primary: false },
    ]
}

/// The apps shown in the mobile bottom tab bar (respects `primary`).
///
/// Truncated to a hard maximum of 5 so the bar stays legible on the
/// narrowest common device (≥320px viewport).
pub fn mobile_primary() -> Vec<&'static App> {
    all().iter().filter(|a| a.primary).take(5).collect()
}
