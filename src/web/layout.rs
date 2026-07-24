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

pub fn visible_nav_items(session: &SessionUser) -> Vec<&'static NavItem> {
    nav_items()
        .iter()
        .filter(|i| match i.perm {
            None => true,
            Some(codes) => session.any_of(codes),
        })
        .collect()
}
