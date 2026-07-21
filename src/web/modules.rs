//! Placeholder screens for ERP modules that haven't been fully implemented
//! yet.
//!
//! Every launcher tile on the home menu needs to route somewhere so the UI
//! feels complete. This module renders a friendly "Coming soon" page for
//! each module using a single shared template — [`templates/modules/stub.html`].
//!
//! When a module gets its real implementation, simply stop registering it
//! here (or remove the specific `.route()`) and mount the real router in
//! its place inside [`crate::web::build_web_router`].

use askama::Template;
use axum::{
    http::HeaderMap,
    response::Response,
    routing::get,
    Router,
};

use crate::http::middleware::TenantScopeState;
use crate::http::ExtractTenant;
use crate::web::auth::read_cookie_from_headers;
use crate::web::error::{render, WebError};
use crate::web::layout::{nav_items, NavContext, NavItem};

/// Metadata for a placeholder module screen.
struct ModuleStub {
    /// URL path (tenant-relative, no leading slash), also the NavContext
    /// `active` key so the sidebar highlight lands on the right item.
    key: &'static str,
    /// Human title shown on the page and in the top-bar.
    title: &'static str,
    /// Short marketing-y description of what the module will do.
    tagline: &'static str,
    /// Lucide icon name.
    icon: &'static str,
    /// Tailwind gradient for the hero icon (matches the tile on the home).
    gradient: &'static str,
    /// Bullet-list of features that will land in this module.
    features: &'static [&'static str],
}

/// Everything the stub template needs.
#[derive(Template)]
#[template(path = "modules/stub.html")]
struct StubPage<'a> {
    nav: &'a NavContext,
    nav_items: &'static [NavItem],
    module: &'a ModuleStub,
}

/// All modules that don't yet have a real implementation. The `students`
/// module is intentionally NOT listed — it has its own real screens.
fn stubs() -> &'static [ModuleStub] {
    &[
        ModuleStub {
            key: "attendance",   title: "Attendance",   icon: "calendar-check", gradient: "from-emerald-500 to-teal-600",
            tagline: "Daily attendance marking, leave tracking, and reports at your fingertips.",
            features: &["Class-wise daily marking", "Bulk import from biometric devices", "Monthly attendance reports", "Parent notifications for absentees"],
        },
        ModuleStub {
            key: "timetable",    title: "Timetable",    icon: "calendar-days",  gradient: "from-sky-500 to-blue-600",
            tagline: "Drag-and-drop timetable builder with conflict detection.",
            features: &["Class + teacher timetables", "Room and lab allocation", "Substitution management", "Printable weekly views"],
        },
        ModuleStub {
            key: "fees",         title: "Fees",         icon: "wallet",         gradient: "from-amber-500 to-orange-600",
            tagline: "Invoices, payments, receipts and dues in one clean ledger.",
            features: &["Fee heads and structures", "Recurring invoices per term", "Online payments with reconciliation", "Dues aging and reminders"],
        },
        ModuleStub {
            key: "examinations", title: "Exams",        icon: "clipboard-check", gradient: "from-fuchsia-500 to-pink-600",
            tagline: "Design assessments, capture marks, and generate report cards.",
            features: &["Exam scheduling", "Grade schemes and rubrics", "Mark entry with validation", "Auto-generated report cards"],
        },
        ModuleStub {
            key: "academic",     title: "Academic",     icon: "book-open",      gradient: "from-violet-500 to-purple-600",
            tagline: "Curriculum, subjects, and academic year setup.",
            features: &["Academic year and terms", "Grades, sections, subjects", "Curriculum mapping", "Learning outcomes"],
        },
        ModuleStub {
            key: "staff",        title: "Staff",        icon: "briefcase",      gradient: "from-zinc-600 to-zinc-800",
            tagline: "Employee directory, roles, and HR essentials.",
            features: &["Employee profiles", "Roles and permissions", "Leaves and holidays", "Contract & document management"],
        },
        ModuleStub {
            key: "payroll",      title: "Payroll",      icon: "banknote",       gradient: "from-lime-500 to-emerald-600",
            tagline: "Salary structures, payslips, and statutory filings.",
            features: &["Salary components", "Monthly payroll run", "Payslips and bank files", "Statutory deductions"],
        },
        ModuleStub {
            key: "guardians",    title: "Guardians",    icon: "users",          gradient: "from-rose-500 to-pink-600",
            tagline: "Parent / guardian contacts linked to students.",
            features: &["Guardian profiles", "Multiple guardians per student", "Contact preferences", "Portal invitations"],
        },
        ModuleStub {
            key: "communication",title: "Communication",icon: "megaphone",      gradient: "from-orange-500 to-red-600",
            tagline: "Notices, SMS, email, and push notifications.",
            features: &["Broadcast notices", "Targeted messages by class/section", "SMS + email + push channels", "Delivery receipts"],
        },
        ModuleStub {
            key: "library",      title: "Library",      icon: "library",        gradient: "from-teal-500 to-cyan-600",
            tagline: "Catalogue, issue and return books with fines.",
            features: &["Book catalogue with ISBN", "Barcode issue / return", "Fines and reservations", "Reading history"],
        },
        ModuleStub {
            key: "transport",    title: "Transport",    icon: "bus",            gradient: "from-yellow-500 to-amber-600",
            tagline: "Routes, stops, vehicles, and student assignments.",
            features: &["Route and stop management", "Vehicle & driver assignment", "Live tracking (optional)", "Route fee billing"],
        },
        ModuleStub {
            key: "hostel",       title: "Hostel",       icon: "bed-double",     gradient: "from-indigo-500 to-blue-700",
            tagline: "Rooms, wardens, and boarder allocations.",
            features: &["Blocks / rooms / beds", "Boarder allocation", "Warden schedules", "Hostel fees"],
        },
        ModuleStub {
            key: "inventory",    title: "Inventory",    icon: "package",        gradient: "from-stone-500 to-stone-700",
            tagline: "Assets, stock, and stores management.",
            features: &["Item master", "Stock in/out", "Store locations", "Asset audits"],
        },
        ModuleStub {
            key: "health",       title: "Health",       icon: "heart-pulse",    gradient: "from-red-500 to-rose-600",
            tagline: "Medical records, incidents, and check-ups.",
            features: &["Student medical profiles", "Sick-bay visits", "Vaccination records", "Emergency contacts"],
        },
        ModuleStub {
            key: "discipline",   title: "Discipline",   icon: "shield-alert",   gradient: "from-orange-600 to-red-700",
            tagline: "Incidents, merits, and disciplinary workflows.",
            features: &["Incident logging", "Merit / demerit points", "Parent notifications", "Escalation workflows"],
        },
        ModuleStub {
            key: "documents",    title: "Documents",    icon: "file-text",      gradient: "from-slate-500 to-slate-700",
            tagline: "Certificates, files, and paperwork.",
            features: &["Document templates", "Bulk certificate generation", "Signed PDFs", "Document retention policies"],
        },
        ModuleStub {
            key: "audit",        title: "Audit",        icon: "history",        gradient: "from-neutral-500 to-neutral-700",
            tagline: "Immutable change history across every module.",
            features: &["Actor, timestamp, and diff for every change", "Search & filter by entity", "Export for compliance", "Retention controls"],
        },
        ModuleStub {
            key: "settings",     title: "Settings",     icon: "settings",       gradient: "from-zinc-500 to-zinc-700",
            tagline: "Tenant-wide configuration and preferences.",
            features: &["Branding & logo", "Notification defaults", "Roles & permissions", "Integrations"],
        },
    ]
}

/// Register a `GET` route for every stubbed module. Returns a router meant
/// to be mounted alongside real module routers under `/{tenant}/…`.
pub fn routes() -> Router<TenantScopeState> {
    let mut r = Router::new();
    for m in stubs() {
        let key = m.key;
        let path = format!("/{}", key);
        r = r.route(&path, get(move |t: ExtractTenant, h: HeaderMap| async move {
            render_stub(key, t, h).await
        }));
    }
    r
}

/// Look up the stub metadata for a `key` and render the shared template.
async fn render_stub(
    key: &'static str,
    ExtractTenant(tenant): ExtractTenant,
    headers: HeaderMap,
) -> Result<Response, WebError> {
    let module = stubs().iter().find(|m| m.key == key)
        .ok_or_else(|| WebError::bad("unknown module"))?;

    let user = read_cookie_from_headers(&headers, "erp_user")
        .unwrap_or_else(|| "Admin".into());
    let nav = NavContext::new(user, tenant.as_str().to_string(), key, module.title);
    render(&StubPage { nav: &nav, nav_items: nav_items(), module })
}
