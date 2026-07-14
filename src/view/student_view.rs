use crate::view::{render_layout, LayoutContext};
use crate::model::Tenant;
use std::collections::HashMap;

pub struct StudentView;

impl StudentView {
    pub fn render_add_form(tenant: &Tenant, field_errors: HashMap<String, String>, general_error: Option<String>) -> String {
        use crate::view::components;
        
        let error_alert = general_error.as_deref().map(components::alert_error).unwrap_or_default();

        let form_content = format!(
            r#"<turbo-frame id="add-student-form">
                {error_alert}
                <form action="/web/{slug}/students/add" method="POST">
                    <div class="row g-3 mb-4">
                        <div class="col-md-6">{fn_input}</div>
                        <div class="col-md-6">{ln_input}</div>
                    </div>
                    <div class="row g-3 mb-4">
                        <div class="col-md-6">{email_input}</div>
                        <div class="col-md-6">{enroll_input}</div>
                    </div>
                    {submit_button}
                </form>
            </turbo-frame>"#,
            slug = tenant.slug,
            error_alert = error_alert,
            fn_input = components::input("First Name", "first_name", "text", "John", true, field_errors.get("first_name").map(|s| s.as_str())),
            ln_input = components::input("Last Name", "last_name", "text", "Doe", true, field_errors.get("last_name").map(|s| s.as_str())),
            email_input = components::input("Email", "email", "email", "john@example.com", false, field_errors.get("email").map(|s| s.as_str())),
            enroll_input = components::input("Enrollment #", "enrollment_number", "text", "STU-001", false, field_errors.get("enrollment_number").map(|s| s.as_str())),
            submit_button = components::button_primary("Register Student", true)
        );

        let content = format!(
            //language=HTML
            r#"<nav class="navbar border-bottom bg-body sticky-top z-3 py-3">
                <div class="container-xl d-flex align-items-center">
                    <a href="/web/{slug}/dashboard" class="btn btn-link text-secondary p-2 me-3 hover-primary">
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
                        </svg>
                    </a>
                    <h1 class="h5 fw-bold mb-0">Add New Student</h1>
                </div>
            </nav>

            <main class="container py-4 py-md-5" style="max-width: 800px;">
                <h2 class="h3 fw-bold mb-4 px-1">Student Enrollment</h2>
                {card}
            </main>
            <style>
                .hover-primary:hover {{ color: var(--bs-primary) !important; }}
            </style>"#,
            slug = tenant.slug,
            card = components::card(form_content)
        );

        render_layout(LayoutContext::for_tenant(tenant, "Add Student"), content)
    }

    pub fn render_success_alert(tenant: &Tenant, name: &str) -> String {
        format!(
            r#"<turbo-frame id="add-student-form">
                <div class="alert alert-success d-flex align-items-center gap-3 border-0 shadow-sm py-4 px-5 mb-4">
                    <div class="bg-success text-white rounded-circle d-flex align-items-center justify-content-center" style="width: 48px; height: 48px;">
                        <svg width="24" height="24" fill="currentColor" viewBox="0 0 16 16">
                            <path d="M12.736 3.97a.733.733 0 0 1 1.047 0c.286.289.29.756.01 1.05L7.88 12.01a.733.733 0 0 1-1.065.02L3.217 8.384a.757.757 0 0 1 0-1.06.733.733 0 0 1 1.047 0l3.052 3.093 5.42-6.446z"/>
                        </svg>
                    </div>
                    <div>
                        <h4 class="h5 fw-bold mb-1">Student Registered!</h4>
                        <p class="mb-0 text-secondary small"><strong>{}</strong> has been added to the system.</p>
                    </div>
                </div>
                <div class="d-flex gap-3 mt-4">
                    <a href="/web/{}/dashboard" class="btn btn-outline-secondary px-4">Back to Dashboard</a>
                    <a href="/web/{}/students/add" class="btn btn-primary px-4 shadow-sm" data-turbo-frame="add-student-form">Add Another Student</a>
                </div>
            </turbo-frame>"#,
            name, tenant.slug, tenant.slug
        )
    }
}
