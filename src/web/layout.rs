//! Layout helpers shared by every page template.
//!
//! Every `*.html` template that extends `base.html` receives a `nav` value
//! (a [`NavContext`]) so the sidebar/bottom-bar can highlight the active
//! item without each template repeating boilerplate.
//!
//! ## RBAC filtering
//!
//! Both the sidebar/bottom-bar (via [`NavItem`]) and dashboard launcher
//! tiles carry an optional permission code. The [`visible_nav_items`]
//! helper (and dashboard's own tile filter) evaluate them against the
//! caller's [`crate::middleware::auth::SessionUser`] and return only
//! entries the user is allowed to see. Handlers should use those helpers
//! rather than the raw lists so parents / students / teachers see only
//! their applicable screens.

use crate::middleware::auth::SessionUser;

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
///
/// `href` is a **tenant-relative path** — templates prefix it with the
/// current tenant id (e.g. `href = ""` → `/acme/`, `href = "students"` →
/// `/acme/students`). Keeping `NavItem` tenant-agnostic means we don't have
/// to rebuild the nav list per request.
pub struct NavItem {
    pub key: &'static str,
    pub label: &'static str,
    /// Tenant-relative path, without a leading slash. `""` == tenant root.
    pub href: &'static str,
    /// Lucide icon name — templates render as `<i data-lucide="{icon}"></i>`.
    pub icon: &'static str,
    /// Whether this item should also appear on the mobile bottom bar. Space
    /// is limited to ~5 items on mobile.
    pub mobile: bool,
    /// Optional RBAC gate. If `Some(code)`, the item is hidden from the
    /// nav unless the current session holds *any* of the codes (comma-free
    /// list). `None` means always visible (dashboard, "more", etc.).
    ///
    /// A slice is used so items that satisfy either a full-view or
    /// row-scoped variant (e.g. `["fees.view", "fees.view_own"]`) can share
    /// a single entry.
    pub perm: Option<&'static [&'static str]>,
}

/// The canonical navigation list. Kept in one place so sidebar + bottom-bar
/// stay in sync. Every `href` is **tenant-relative** — templates render it
/// as `/{{ nav.tenant_id }}/{{ item.href }}`.
pub fn nav_items() -> &'static [NavItem] {
    use crate::services::perm::*;
    &[
        NavItem { key: "dashboard",  label: "Dashboard",  href: "",           icon: "layout-dashboard", mobile: true,  perm: None },
        NavItem { key: "students",   label: "Students",   href: "students",   icon: "graduation-cap",   mobile: true,  perm: Some(&[STUDENTS_VIEW, STUDENTS_VIEW_OWN]) },
        NavItem { key: "staff",      label: "Staff",      href: "staff",      icon: "briefcase",        mobile: false, perm: Some(&[STAFF_VIEW]) },
        NavItem { key: "academic",   label: "Academic",   href: "academic",   icon: "book-open",        mobile: false, perm: Some(&[ACADEMIC_VIEW]) },
        NavItem { key: "attendance", label: "Attendance", href: "attendance", icon: "calendar-check",   mobile: true,  perm: Some(&[ATTENDANCE_VIEW, ATTENDANCE_VIEW_OWN, ATTENDANCE_MARK]) },
        NavItem { key: "fees",       label: "Fees",       href: "fees",       icon: "wallet",           mobile: true,  perm: Some(&[FEES_VIEW, FEES_VIEW_OWN, FEES_COLLECT, FEES_PAY]) },
        NavItem { key: "library",    label: "Library",    href: "library",    icon: "library",          mobile: false, perm: Some(&[LIBRARY_VIEW]) },
        NavItem { key: "more",       label: "More",       href: "more",       icon: "menu",             mobile: true,  perm: None },
    ]
}

/// Filter the canonical [`nav_items`] list to just the entries the current
/// user is allowed to see. Items with `perm = None` are always kept; items
/// with `perm = Some(codes)` are kept if the session holds any of them.
///
/// Templates receive the returned `Vec<&'static NavItem>` and iterate it
/// exactly the same way as the raw slice — no HTML changes required.
pub fn visible_nav_items(session: &SessionUser) -> Vec<&'static NavItem> {
    nav_items()
        .iter()
        .filter(|i| match i.perm {
            None => true,
            Some(codes) => session.any_of(codes),
        })
        .collect()
}
