use crate::middleware::auth::SessionUser;

#[derive(Clone)]
pub struct NavContext {
    pub user_display: String,

    pub tenant_id: String,

    pub active: &'static str,

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

pub struct NavItem {
    pub key: &'static str,
    pub label: &'static str,

    pub href: &'static str,

    pub icon: &'static str,

    pub mobile: bool,

    pub perm: Option<&'static [&'static str]>,
}

/// Framework-only nav: Dashboard + a single Demo entry. Add one
/// `NavItem` per module here as you (re-)build them.
pub fn nav_items() -> &'static [NavItem] {
    use crate::services::perm::*;
    &[
        NavItem {
            key: "dashboard",
            label: "Dashboard",
            href: "",
            icon: "layout-dashboard",
            mobile: true,
            perm: None,
        },
        NavItem {
            key: "demo",
            label: "Demo",
            href: "demo",
            icon: "sparkles",
            mobile: true,
            perm: Some(&[DEMO_VIEW]),
        },
    ]
}

pub fn visible_nav_items(session: &SessionUser) -> Vec<&'static NavItem> {
    nav_items()
        .iter()
        .filter(|i| match i.perm {
            None => true,
            Some(codes) => session.any_of(codes),
        })
        .collect()
}
