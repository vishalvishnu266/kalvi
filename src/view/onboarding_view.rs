use crate::view::{render_layout, LayoutContext};

pub struct OnboardingView;

impl OnboardingView {
    pub fn render_form(error: Option<String>) -> String {
        use crate::view::components;
        
        let error_alert = error.as_deref().map(components::alert_error).unwrap_or_default();

        let form_content = format!(
            r#"{error_alert}
            <form action="/registration" method="POST" data-turbo="false">
                <div class="row g-3">
                    <div class="col-md-6">{name_input}</div>
                    <div class="col-md-6">{slug_input}</div>
                </div>
                
                <div class="position-relative py-4 text-center">
                    <hr class="text-secondary opacity-25">
                    <span class="position-absolute top-50 start-50 translate-middle bg-body px-3 small text-uppercase fw-bold text-secondary">Admin Credentials</span>
                </div>

                <div class="row g-3 mb-4">
                    <div class="col-md-6">{user_input}</div>
                    <div class="col-md-6">{pass_input}</div>
                </div>

                {submit_button}
            </form>"#,
            error_alert = error_alert,
            name_input = components::input("Institution Name", "name", "text", "e.g. City High", true),
            slug_input = components::input("Slug", "slug", "text", "demo-school", true),
            user_input = components::input("Admin Username", "admin_username", "text", "admin", true),
            pass_input = components::input("Admin Password", "admin_password", "password", "••••••••", true),
            submit_button = components::button_primary("Initialize Institution", true)
        );

        let content = format!(
            //language=HTML
            r#"<div class="container min-vh-100 d-flex align-items-center justify-content-center p-4">
                <div class="w-100" style="max-width: 600px;">
                    <div class="text-center mb-5">
                        <div class="d-inline-flex align-items-center justify-content-center bg-primary-subtle text-primary rounded-4 mb-4 shadow-sm" style="width: 72px; height: 72px; font-size: 2rem; font-weight: 800;">
                            O
                        </div>
                        <h2 class="h2 fw-bold text-body-emphasis">New Institution</h2>
                        <p class="text-secondary fw-medium text-center">Set up your isolated ERP workspace</p>
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
            card = components::card(form_content)
        );

        render_layout(LayoutContext::default(), content)
    }

    pub fn render_success(slug: &str, name: &str) -> String {
        let content = format!(
            //language=HTML
            r#"<div class="container py-5">
                <div class="card shadow border-0 mx-auto text-center p-4 p-md-5" style="max-width: 500px;">
                    <div class="d-inline-flex align-items-center justify-content-center bg-success-subtle text-success rounded-circle mx-auto mb-4" style="width: 80px; height: 80px;">
                        <svg xmlns="http://www.w3.org/2000/svg" width="40" height="40" fill="currentColor" viewBox="0 0 16 16">
                            <path d="M12.736 3.97a.733.733 0 0 1 1.047 0c.286.289.29.756.01 1.05L7.88 12.01a.733.733 0 0 1-1.065.02L3.217 8.384a.757.757 0 0 1 0-1.06.733.733 0 0 1 1.047 0l3.052 3.093 5.42-6.446z"/>
                        </svg>
                    </div>
                    <h2 class="h2 fw-bold mb-3 text-body-emphasis">Institution Ready!</h2>
                    <p class="text-secondary mb-5 px-md-3">
                        <strong>{name}</strong> has been successfully initialized. You can now log in to your tenant dashboard.
                    </p>
                    <a href="/web/{slug}/login" class="btn btn-primary btn-lg py-3 shadow">
                        Go to Dashboard Login
                    </a>
                </div>
            </div>"#,
            slug = slug,
            name = name
        );

        render_layout(LayoutContext::default(), content)
    }
}
