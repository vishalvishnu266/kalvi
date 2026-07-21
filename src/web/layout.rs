//! Layout helpers shared by every page template.
//!
//! Every `*.html` template that extends `base.html` receives a `nav` value
//! (a [`NavContext`]) so the sidebar/bottom-bar can highlight the active
//! item without each template repeating boilerplate.

/// Metadata used by `base.html` (and the sidebar / bottom-nav partials).
#[derive(Clone)]
pub struct NavContext {
    /// Currently signed-in username, shown in the top-bar and user menu.
    pub user_display: String,
    /// Tenant id (shown in the top-bar as a small pill).
    pub tenant_id: String,
    /// Key of the currently-active nav item (matches `key` in [`nav_items`]).
    /// Templates use it to add an `aria-current="page"` and a highlight style.
    pub active: &'static str,
    /// Page title, rendered in the top-bar and `<title>`.
    pub page_title: String,
}

impl NavContext {
    pub fn new(
        user_display: impl Into<String>,
        tenant_id: impl Into<String>,
        active: &'static str,
        page_title: impl Into<String>,
    ) -> Self {
        Self {
            user_display: user_display.into(),
            tenant_id: tenant_id.into(),
            active,
            page_title: page_title.into(),
        }
    }
}

/// A single sidebar / bottom-bar entry.
pub struct NavItem {
    pub key: &'static str,
    pub label: &'static str,
    pub href: &'static str,
    /// Lucide icon name — templates render as `<i data-lucide="{icon}"></i>`.
    pub icon: &'static str,
    /// Whether this item should also appear on the mobile bottom bar. Space
    /// is limited to ~5 items on mobile.
    pub mobile: bool,
}

/// The canonical navigation list. Kept in one place so sidebar + bottom-bar
/// stay in sync.
pub fn nav_items() -> &'static [NavItem] {
    &[
        NavItem { key: "dashboard", label: "Dashboard", href: "/",          icon: "layout-dashboard", mobile: true  },
        NavItem { key: "students",  label: "Students",  href: "/students",  icon: "graduation-cap",   mobile: true  },
        NavItem { key: "staff",     label: "Staff",     href: "/staff",     icon: "briefcase",        mobile: false },
        NavItem { key: "academic",  label: "Academic",  href: "/academic",  icon: "book-open",        mobile: false },
        NavItem { key: "attendance",label: "Attendance",href: "/attendance",icon: "calendar-check",   mobile: true  },
        NavItem { key: "fees",      label: "Fees",      href: "/fees",      icon: "wallet",           mobile: true  },
        NavItem { key: "library",   label: "Library",   href: "/library",   icon: "library",          mobile: false },
        NavItem { key: "more",      label: "More",      href: "/more",      icon: "menu",             mobile: true  },
    ]
}
