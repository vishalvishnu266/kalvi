use crate::view::{render_layout, LayoutContext};
use crate::model::Tenant;

pub struct LoginView;

use std::collections::HashMap;

impl LoginView {
    pub fn render_login(tenant: &Tenant, field_errors: HashMap<String, String>, general_error: Option<String>) -> String {
        use crate::view::components;
        
        let error_alert = general_error.as_deref().map(components::alert_error).unwrap_or_default();

        let form_content = format!(
            r#"<turbo-frame id="login-form">
                {error_alert}
                <form action="/web/{slug}/login" method="POST">
                    {username_input}
                    {password_input}
                    <div class="mt-4">{submit_button}</div>
                </form>
            </turbo-frame>"#,
            error_alert = error_alert,
            slug = tenant.slug,
            username_input = components::input("Username", "username", "text", "Enter your username", true, field_errors.get("username").map(|s| s.as_str())),
            password_input = components::input("Password", "password", "password", "••••••••", true, field_errors.get("password").map(|s| s.as_str())),
            submit_button = components::button_primary("Sign In", true)
        );

        let content = format!(
            //language=HTML
            r#"<div class="container min-vh-100 d-flex align-items-center justify-content-center p-4">
                <div class="w-100" style="max-width: 450px;">
                    <div class="text-center mb-5">
                        <div class="d-inline-flex align-items-center justify-content-center bg-primary-subtle text-primary rounded-4 mb-4 shadow-sm" style="width: 72px; height: 72px; font-size: 2rem; font-weight: 800;">
                            {logo_char}
                        </div>
                        <h2 class="h2 fw-bold text-body-emphasis">{tenant_name}</h2>
                        <p class="text-secondary fw-medium">Please sign in to continue</p>
                    </div>

                    {card}

                    <div class="mt-4 text-center">
                        <a href="/" class="text-decoration-none small text-secondary hover-primary transition-all">← Back to home</a>
                    </div>
                </div>
            </div>
            <style>
                .hover-primary:hover {{ color: var(--bs-primary) !important; }}
            </style>"#,
            logo_char = tenant.name.chars().next().unwrap_or('K'),
            tenant_name = tenant.name,
            card = components::card(form_content)
        );

        render_layout(LayoutContext::for_tenant(tenant, "Login"), content)
    }
}
