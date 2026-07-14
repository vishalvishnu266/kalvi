use crate::view::{render_layout, LayoutContext};
use crate::model::Tenant;
use std::collections::HashMap;

pub struct StudentView;

impl StudentView {
    pub fn render_add_form(tenant: &Tenant, field_errors: HashMap<String, String>, general_error: Option<String>) -> String {
        use crate::view::components;
        let error_alert = general_error.as_deref().map(components::alert_error).unwrap_or_default();

        let form_content = format!(
            r###"<turbo-frame id="add-student-form">
                {error_alert}
                <form action="/web/{}/students/add" method="POST" class="space-y-4">
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                        {fn_input}
                        {ln_input}
                    </div>
                    {email_input}
                    {enroll_input}
                    {submit_button}
                </form>
            </turbo-frame>"###,
            tenant.slug,
            error_alert = error_alert,
            fn_input = components::input("First Name", "first_name", "text", "John", true, field_errors.get("first_name").map(|s| s.as_str())),
            ln_input = components::input("Last Name", "last_name", "text", "Doe", true, field_errors.get("last_name").map(|s| s.as_str())),
            email_input = components::input("Email", "email", "email", "john@example.com", false, field_errors.get("email").map(|s| s.as_str())),
            enroll_input = components::input("Enrollment #", "enrollment_number", "text", "STU-001", false, field_errors.get("enrollment_number").map(|s| s.as_str())),
            submit_button = components::button_primary("Enroll Student", true)
        );

        let content = format!(
            //language=HTML
            r###"<div class="min-h-screen flex flex-col items-center justify-center p-4 bg-slate-50 dark:bg-slate-950">
                <div class="max-w-lg w-full">
                    <div class="mb-8 flex items-center gap-4">
                        <a href="/web/{}/dashboard" class="p-2 hover:bg-slate-200 dark:hover:bg-slate-800 rounded-lg transition-colors">
                            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
                            </svg>
                        </a>
                        <h1 class="text-3xl font-extrabold text-slate-900 dark:text-white">Enroll Student</h1>
                    </div>
                    {card}
                </div>
            </div>"###,
            tenant.slug,
            card = components::card(form_content)
        );

        render_layout(LayoutContext::default(), content)
    }
}
