use askama::Template;
use axum::{response::Response, Extension};

use crate::http::TenantScope;
use crate::middleware::auth::SessionUser;
use crate::web::error::{render, WebError};
use crate::web::layout::{visible_nav_items, NavContext, NavItem};

struct ModuleStub {
    key: &'static str,
    title: &'static str,
    tagline: &'static str,
    icon: &'static str,
    gradient: &'static str,
    features: &'static [&'static str],
}

#[derive(Template)]
#[template(path = "modules/stub.html")]
struct StubPage<'a> {
    nav: &'a NavContext,
    nav_items: Vec<&'static NavItem>,
    module: &'a ModuleStub,
}

fn stubs() -> &'static [ModuleStub] {
    &[
        ModuleStub { key: "attendance",   title: "Attendance",   icon: "calendar-check", gradient: "from-emerald-500 to-teal-600",
            tagline: "Daily attendance marking, leave tracking, and reports at your fingertips.",
            features: &["Class-wise daily marking", "Bulk import from biometric devices", "Monthly attendance reports", "Parent notifications for absentees"] },
        ModuleStub { key: "timetable",    title: "Timetable",    icon: "calendar-days",  gradient: "from-sky-500 to-blue-600",
            tagline: "Drag-and-drop timetable builder with conflict detection.",
            features: &["Class + teacher timetables", "Room and lab allocation", "Substitution management", "Printable weekly views"] },
        ModuleStub { key: "fees",         title: "Fees",         icon: "wallet",         gradient: "from-amber-500 to-orange-600",
            tagline: "Invoices, payments, receipts and dues in one clean ledger.",
            features: &["Fee heads and structures", "Recurring invoices per term", "Online payments with reconciliation", "Dues aging and reminders"] },
        ModuleStub { key: "examinations", title: "Exams",        icon: "clipboard-check", gradient: "from-fuchsia-500 to-pink-600",
            tagline: "Design assessments, capture marks, and generate report cards.",
            features: &["Exam scheduling", "Grade schemes and rubrics", "Mark entry with validation", "Auto-generated report cards"] },
        ModuleStub { key: "academic",     title: "Academic",     icon: "book-open",      gradient: "from-violet-500 to-purple-600",
            tagline: "Curriculum, subjects, and academic year setup.",
            features: &["Academic year and terms", "Grades, sections, subjects", "Curriculum mapping", "Learning outcomes"] },
        ModuleStub { key: "payroll",      title: "Payroll",      icon: "banknote",       gradient: "from-lime-500 to-emerald-600",
            tagline: "Salary structures, payslips, and statutory filings.",
            features: &["Salary components", "Monthly payroll run", "Payslips and bank files", "Statutory deductions"] },
        ModuleStub { key: "communication",title: "Communication",icon: "megaphone",      gradient: "from-orange-500 to-red-600",
            tagline: "Notices, SMS, email, and push notifications.",
            features: &["Broadcast notices", "Targeted messages by class/section", "SMS + email + push channels", "Delivery receipts"] },
        ModuleStub { key: "library",      title: "Library",      icon: "library",        gradient: "from-teal-500 to-cyan-600",
            tagline: "Catalogue, issue and return books with fines.",
            features: &["Book catalogue with ISBN", "Barcode issue / return", "Fines and reservations", "Reading history"] },
        ModuleStub { key: "transport",    title: "Transport",    icon: "bus",            gradient: "from-yellow-500 to-amber-600",
            tagline: "Routes, stops, vehicles, and student assignments.",
            features: &["Route and stop management", "Vehicle & driver assignment", "Live tracking (optional)", "Route fee billing"] },
        ModuleStub { key: "hostel",       title: "Hostel",       icon: "bed-double",     gradient: "from-indigo-500 to-blue-700",
            tagline: "Rooms, wardens, and boarder allocations.",
            features: &["Blocks / rooms / beds", "Boarder allocation", "Warden schedules", "Hostel fees"] },
        ModuleStub { key: "inventory",    title: "Inventory",    icon: "package",        gradient: "from-stone-500 to-stone-700",
            tagline: "Assets, stock, and stores management.",
            features: &["Item master", "Stock in/out", "Store locations", "Asset audits"] },
        ModuleStub { key: "health",       title: "Health",       icon: "heart-pulse",    gradient: "from-red-500 to-rose-600",
            tagline: "Medical records, incidents, and check-ups.",
            features: &["Student medical profiles", "Sick-bay visits", "Vaccination records", "Emergency contacts"] },
        ModuleStub { key: "discipline",   title: "Discipline",   icon: "shield-alert",   gradient: "from-orange-600 to-red-700",
            tagline: "Incidents, merits, and disciplinary workflows.",
            features: &["Incident logging", "Merit / demerit points", "Parent notifications", "Escalation workflows"] },
        ModuleStub { key: "documents",    title: "Documents",    icon: "file-text",      gradient: "from-slate-500 to-slate-700",
            tagline: "Certificates, files, and paperwork.",
            features: &["Document templates", "Bulk certificate generation", "Signed PDFs", "Document retention policies"] },
        ModuleStub { key: "audit",        title: "Audit",        icon: "history",        gradient: "from-neutral-500 to-neutral-700",
            tagline: "Immutable change history across every module.",
            features: &["Actor, timestamp, and diff for every change", "Search & filter by entity", "Export for compliance", "Retention controls"] },
        ModuleStub { key: "settings",     title: "Settings",     icon: "settings",       gradient: "from-zinc-500 to-zinc-700",
            tagline: "Tenant-wide configuration and preferences.",
            features: &["Branding & logo", "Notification defaults", "Roles & permissions", "Integrations"] },
    ]
}

async fn render_stub(key: &'static str, scope: TenantScope, session: SessionUser)
    -> Result<Response, WebError>
{
    let module = stubs().iter().find(|m| m.key == key)
        .ok_or_else(|| WebError::bad("unknown module"))?;
    let nav = NavContext::new(
        session.display.clone(),
        scope.tenant.as_str().to_string(),
        key, module.title,
    );
    let nav_items = visible_nav_items(&session);
    render(&StubPage { nav: &nav, nav_items, module })
}

macro_rules! stub_handler {
    ($name:ident, $key:literal) => {
        pub async fn $name(
            scope: TenantScope,
            Extension(session): Extension<SessionUser>,
        ) -> Result<Response, WebError>
        { render_stub($key, scope, session).await }
    };
}

stub_handler!(attendance,    "attendance");
stub_handler!(timetable,     "timetable");
stub_handler!(fees,          "fees");
stub_handler!(examinations,  "examinations");
stub_handler!(academic,      "academic");
stub_handler!(payroll,       "payroll");
stub_handler!(communication, "communication");
stub_handler!(library,       "library");
stub_handler!(transport,     "transport");
stub_handler!(hostel,        "hostel");
stub_handler!(inventory,     "inventory");
stub_handler!(health,        "health");
stub_handler!(discipline,    "discipline");
stub_handler!(documents,     "documents");
stub_handler!(audit,         "audit");
stub_handler!(settings,      "settings");
