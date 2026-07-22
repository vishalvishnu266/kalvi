//! Tenant home: OS-style menu launcher at `/web/{tenant}/`.

use askama::Template;
use axum::{response::Response, Extension};

use crate::http::TenantScope;
use crate::middleware::auth::SessionUser;
use crate::web::error::{render, WebError};
use crate::web::layout::{nav_items, NavContext, NavItem};

#[derive(Template)]
#[template(path = "dashboard.html")]
struct MenuPage<'a> {
    nav: &'a NavContext,
    nav_items: &'static [NavItem],
    tiles: &'static [Tile],
}

/// A single launcher tile on the home screen.
pub struct Tile {
    pub href: &'static str,
    pub label: &'static str,
    pub description: &'static str,
    pub icon: &'static str,
    pub gradient: &'static str,
}

fn tiles() -> &'static [Tile] {
    &[
        Tile { href: "students",       label: "Students",       description: "Admissions & profiles",   icon: "graduation-cap", gradient: "from-brand-500 to-indigo-600" },
        Tile { href: "attendance",     label: "Attendance",     description: "Daily marking & reports", icon: "calendar-check", gradient: "from-emerald-500 to-teal-600" },
        Tile { href: "timetable",      label: "Timetable",      description: "Classes & periods",       icon: "calendar-days",  gradient: "from-sky-500 to-blue-600" },
        Tile { href: "fees",           label: "Fees",           description: "Invoices & payments",     icon: "wallet",         gradient: "from-amber-500 to-orange-600" },
        Tile { href: "examinations",   label: "Exams",          description: "Grades & report cards",   icon: "clipboard-check", gradient: "from-fuchsia-500 to-pink-600" },
        Tile { href: "academic",       label: "Academic",       description: "Curriculum & subjects",   icon: "book-open",      gradient: "from-violet-500 to-purple-600" },
        Tile { href: "staff",          label: "Staff",          description: "Employees & roles",       icon: "briefcase",      gradient: "from-zinc-600 to-zinc-800" },
        Tile { href: "payroll",        label: "Payroll",        description: "Salaries & payslips",     icon: "banknote",       gradient: "from-lime-500 to-emerald-600" },
        Tile { href: "guardians",      label: "Guardians",      description: "Parents & contacts",      icon: "users",          gradient: "from-rose-500 to-pink-600" },
        Tile { href: "communication",  label: "Communication",  description: "Notices & messages",      icon: "megaphone",      gradient: "from-orange-500 to-red-600" },
        Tile { href: "library",        label: "Library",        description: "Books & loans",           icon: "library",        gradient: "from-teal-500 to-cyan-600" },
        Tile { href: "transport",      label: "Transport",      description: "Routes & stops",          icon: "bus",            gradient: "from-yellow-500 to-amber-600" },
        Tile { href: "hostel",         label: "Hostel",         description: "Rooms & allocations",     icon: "bed-double",     gradient: "from-indigo-500 to-blue-700" },
        Tile { href: "inventory",      label: "Inventory",      description: "Assets & stock",          icon: "package",        gradient: "from-stone-500 to-stone-700" },
        Tile { href: "health",         label: "Health",         description: "Medical & incidents",     icon: "heart-pulse",    gradient: "from-red-500 to-rose-600" },
        Tile { href: "discipline",     label: "Discipline",     description: "Incidents & merits",      icon: "shield-alert",   gradient: "from-orange-600 to-red-700" },
        Tile { href: "documents",      label: "Documents",      description: "Files & certificates",    icon: "file-text",      gradient: "from-slate-500 to-slate-700" },
        Tile { href: "audit",          label: "Audit",          description: "Change history",          icon: "history",        gradient: "from-neutral-500 to-neutral-700" },
        Tile { href: "settings",       label: "Settings",       description: "Tenant configuration",    icon: "settings",       gradient: "from-zinc-500 to-zinc-700" },
    ]
}

pub async fn index(
    scope: TenantScope,
    Extension(session): Extension<SessionUser>,
) -> Result<Response, WebError> {
    let nav = NavContext::new(
        session.display.clone(),
        scope.tenant.as_str().to_string(),
        "dashboard", "Home",
    );
    render(&MenuPage { nav: &nav, nav_items: nav_items(), tiles: tiles() })
}
