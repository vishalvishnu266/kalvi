//! OpenAPI schema.
//!
//! We describe the API **surface** (paths, tags, HTTP verbs, tenant header)
//! rather than every DTO. Deriving `ToSchema` on 100+ existing types would
//! require touching every module; skipping it keeps this module small while
//! still giving clients a discoverable Swagger UI.
//!
//! Endpoints:
//! * `GET  /api/openapi.json` — raw OpenAPI 3 document
//! * `GET  /api/docs`         — Swagger UI

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "School ERP API",
        version = "0.1.0",
        description = "Multi-tenant K-12 School ERP.\n\n\
                       * `/api/admin/*` — control-plane tenant management.\n\
                       * `/api/tenant/{tenant}/*` — per-tenant business API. The tenant \
                         id travels as a path parameter (e.g. \
                         `/api/tenant/acme/people/students`).",
    ),
    tags(
        (name = "admin.tenants", description = "Control-plane tenant management"),
        (name = "auth",          description = "Per-tenant authentication"),
        (name = "academic",      description = "Years, terms, grades, sections, rooms, subjects, class sections"),
        (name = "people",        description = "Students & staff"),
        (name = "guardians",     description = "Guardians and student links"),
        (name = "enrollment",    description = "Class enrollments, transfers, promotions"),
        (name = "attendance",    description = "Student & staff attendance"),
        (name = "timetable",     description = "Periods & timetable slots"),
        (name = "examinations",  description = "Exams, results, report cards"),
        (name = "fees",          description = "Fee structures, invoices, payments, ledger"),
        (name = "payroll",       description = "Salary structures, payslips"),
        (name = "library",       description = "Books & issues"),
        (name = "transport",     description = "Vehicles, routes, student assignments"),
        (name = "hostel",        description = "Hostels, rooms, allocations"),
        (name = "inventory",     description = "Vendors, items, purchase orders"),
        (name = "communication", description = "Announcements, messages, notifications"),
        (name = "health",        description = "Health records & vaccinations"),
        (name = "discipline",    description = "Incident tracking"),
        (name = "documents",     description = "Polymorphic file attachments"),
        (name = "audit",         description = "Read-only audit trail"),
    ),
)]
pub struct ApiDoc;

/// Router that serves `/api/docs` (Swagger UI) and `/api/openapi.json`.
pub fn swagger_router() -> axum::Router {
    SwaggerUi::new("/api/docs")
        .url("/api/openapi.json", ApiDoc::openapi())
        .into()
}
