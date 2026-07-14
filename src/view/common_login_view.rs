use crate::view::{render_layout, LayoutContext};

pub struct CommonLoginView;

use std::collections::HashMap;

impl CommonLoginView {
    pub fn render_common_login(field_errors: HashMap<String, String>, general_error: Option<String>) -> String {
        use crate::view::components;

        let error_alert = general_error.as_deref().map(components::alert_error).unwrap_or_default();

        let form_content = format!(
            r#"<turbo-frame id="common-login-form">
                {error_alert}
                <form action="/login" method="POST">
                    {slug_input}
                    {username_input}
                    {password_input}
                    <div class="mt-4">{submit_button}</div>
                </form>
            </turbo-frame>"#,
            error_alert = error_alert,
            slug_input = components::input("Institution Slug", "slug", "text", "e.g. demo-school", true, field_errors.get("slug").map(|s| s.as_str())),
            username_input = components::input("Username", "username", "text", "Enter username", true, field_errors.get("username").map(|s| s.as_str())),
            password_input = components::input("Password", "password", "password", "••••••••", true, field_errors.get("password").map(|s| s.as_str())),
            submit_button = components::button_primary("Access Portal", true)
        );

        let content = format!(
            //language=HTML
            r#"<div class="container min-vh-100 d-flex align-items-center justify-content-center p-4">
                <div class="w-100" style="max-width: 450px;">
                    <div class="text-center mb-5">
                        <div class="d-inline-flex align-items-center justify-content-center bg-primary-subtle text-primary rounded-4 mb-4 shadow-sm" style="width: 72px; height: 72px; font-size: 2rem; font-weight: 800;">
                            P
                        </div>
                        <h2 class="h2 fw-bold text-body-emphasis text-center">Portal Access</h2>
                        <p class="text-secondary fw-medium text-center small">Sign in to your specific institution</p>
                    </div>

                    {card}
                    
                    <div class="mt-4 pt-4 border-top text-center d-flex flex-column gap-2">
                        <a href="/saas/login" class="text-decoration-none small text-secondary hover-primary">Control Plane Login</a>
                        <a href="/" class="text-decoration-none small text-secondary hover-primary transition-all">← Back to home</a>
                    </div>
                </div>
            </div>
            <style>
                .hover-primary:hover {{ color: var(--bs-primary) !important; }}
            </style>"#,
            card = components::card(form_content)
        );

        render_layout(LayoutContext::default(), content)
    }
}
