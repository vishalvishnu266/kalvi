use crate::view::{render_layout, LayoutContext};

pub struct SaasView;

impl SaasView {
    pub fn render_onboard(error: Option<String>) -> String {
        use crate::view::components;
        let error_alert = error.as_deref().map(components::alert_error).unwrap_or_default();

        let form_content = format!(
            r#"{error_alert}
            <form action="/saas/onboard" method="POST" data-turbo="false">
                {user_input}
                {name_input}
                {pass_input}
                <div class="mt-4">{submit_button}</div>
            </form>"#,
            error_alert = error_alert,
            user_input = components::input("Username", "username", "text", "admin", true),
            name_input = components::input("Full Name", "full_name", "text", "Your Name", true),
            pass_input = components::input("Password", "password", "password", "••••••••", true),
            submit_button = components::button_primary("Initialize SaaS Admin", true)
        );

        let content = format!(
            //language=HTML
            r#"<div class="container min-vh-100 d-flex align-items-center justify-content-center p-4">
                <div class="w-100" style="max-width: 450px;">
                    <div class="text-center mb-5">
                        <div class="d-inline-flex align-items-center justify-content-center bg-primary-subtle text-primary rounded-4 mb-4 shadow-sm" style="width: 72px; height: 72px; font-size: 2rem; font-weight: 800;">
                            S
                        </div>
                        <h2 class="h2 fw-bold text-body-emphasis">Control Plane</h2>
                        <p class="text-secondary fw-medium">Platform Owner Setup</p>
                    </div>

                    {card}
                </div>
            </div>"#,
            card = components::card(form_content)
        );

        render_layout(LayoutContext::default(), content)
    }

    pub fn render_login(error: Option<String>) -> String {
        use crate::view::components;
        let error_alert = error.as_deref().map(components::alert_error).unwrap_or_default();

        let form_content = format!(
            r#"{error_alert}
            <form action="/saas/login" method="POST" data-turbo="false">
                {user_input}
                {pass_input}
                <div class="mt-4">{submit_button}</div>
            </form>"#,
            error_alert = error_alert,
            user_input = components::input("Username", "username", "text", "Enter username", true),
            pass_input = components::input("Password", "password", "password", "••••••••", true),
            submit_button = components::button_primary("Sign In to Control Plane", true)
        );

        let content = format!(
            //language=HTML
            r#"<div class="container min-vh-100 d-flex align-items-center justify-content-center p-4">
                <div class="w-100" style="max-width: 450px;">
                    <div class="text-center mb-5">
                        <div class="d-inline-flex align-items-center justify-content-center bg-primary-subtle text-primary rounded-4 mb-4 shadow-sm" style="width: 72px; height: 72px; font-size: 2rem; font-weight: 800;">
                            S
                        </div>
                        <h2 class="h2 fw-bold text-body-emphasis">Admin Login</h2>
                        <p class="text-secondary fw-medium">Platform Control Plane</p>
                    </div>

                    {card}

                    <div class="mt-4 text-center">
                        <a href="/" class="text-decoration-none small text-secondary hover-primary transition-all">← Back to home</a>
                    </div>
                </div>
            </div>
            <style>
                .hover-primary:hover { color: var(--bs-primary) !important; }
            </style>"#,
            card = components::card(form_content)
        );

        render_layout(LayoutContext::default(), content)
    }
}
