//! Dashboard home page.

use askama::Template;
use axum::{
    http::HeaderMap,
    response::Response,
    routing::get,
    Router,
};

use crate::http::middleware::TenantScopeState;
use crate::http::{ExtractServices, ExtractTenant};
use crate::web::auth::read_cookie_from_headers;
use crate::web::error::{render, WebError};
use crate::web::layout::{nav_items, NavContext, NavItem};

#[derive(Template)]
#[template(path = "dashboard.html")]
struct DashboardPage<'a> {
    nav: &'a NavContext,
    nav_items: &'static [NavItem],
    kpis: Vec<Kpi>,
}

struct Kpi {
    label: &'static str,
    value: String,
    delta: &'static str,     // e.g. "+2.1%"
    icon: &'static str,      // lucide icon name
    /// Tailwind classes for the icon chip background/text.
    chip_class: &'static str,
    /// Tailwind classes for the delta line.
    delta_class: &'static str,
}

fn chip(intent: &'static str) -> &'static str {
    match intent {
        "success" => "bg-emerald-50 text-emerald-600 dark:bg-emerald-500/10 dark:text-emerald-400",
        "warning" => "bg-amber-50 text-amber-600 dark:bg-amber-500/10 dark:text-amber-400",
        "danger"  => "bg-rose-50 text-rose-600 dark:bg-rose-500/10 dark:text-rose-400",
        "info"    => "bg-sky-50 text-sky-600 dark:bg-sky-500/10 dark:text-sky-400",
        _         => "bg-zinc-100 text-zinc-600 dark:bg-zinc-800 dark:text-zinc-300",
    }
}
fn delta_cls(intent: &'static str) -> &'static str {
    match intent {
        "success" => "text-emerald-600 dark:text-emerald-400",
        "warning" => "text-amber-600 dark:text-amber-400",
        "danger"  => "text-rose-600 dark:text-rose-400",
        "info"    => "text-sky-600 dark:text-sky-400",
        _         => "text-zinc-500 dark:text-zinc-400",
    }
}

fn kpi(label: &'static str, value: String, delta: &'static str, intent: &'static str, icon: &'static str) -> Kpi {
    Kpi { label, value, delta, icon, chip_class: chip(intent), delta_class: delta_cls(intent) }
}

pub fn routes() -> Router<TenantScopeState> {
    Router::new().route("/", get(index))
}

async fn index(
    ExtractTenant(tenant): ExtractTenant,
    ExtractServices(services): ExtractServices,
    headers: HeaderMap,
) -> Result<Response, WebError> {
    // Best-effort counts. Any failure becomes "—" so the shell still renders.
    let students = services.repos.students.list(1, 0).await
        .map(|_| "1,248".to_string())  // placeholder until we add count()
        .unwrap_or_else(|_| "—".into());

    let kpis = vec![
        kpi("Students",         students,          "+3.2% this term",     "info",    "graduation-cap"),
        kpi("Attendance",       "94.7%".into(),    "+0.4% vs last week",  "success", "calendar-check"),
        kpi("Outstanding Dues", "$18,420".into(),  "-5.1% vs last month", "warning", "wallet"),
        kpi("Staff Present",    "82 / 88".into(),  "6 on leave",          "neutral", "briefcase"),
    ];

    let user = read_cookie_from_headers(&headers, "erp_user")
        .unwrap_or_else(|| "Admin".into());
    let nav = NavContext::new(user, tenant.as_str().to_string(), "dashboard", "Dashboard");
    render(&DashboardPage { nav: &nav, nav_items: nav_items(), kpis })
}
