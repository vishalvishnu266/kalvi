use crate::view::{render_layout, LayoutContext};

pub struct SaasView;

impl SaasView {
    pub fn render_onboard(error: Option<String>) -> String {
        use crate::view::components;
        let error_alert = error.as_deref().map(components::alert_error).unwrap_or_default();

        let form_content = format!(
            r#"{error_alert}
            <form action="/saas/onboard" method="POST" class="space-y-4" data-turbo="false">
                {user_input}
                {name_input}
                {pass_input}
                {submit_button}
            </form>"#,
            error_alert = error_alert,
            user_input = components::input("Username", "username", "text", "admin", true),
            name_input = components::input("Full Name", "full_name", "text", "Your Name", true),
            pass_input = components::input("Password", "password", "password", "••••••••", true),
            submit_button = components::button_primary("Initialize SaaS Admin", true)
        );

        let content = format!(
            //language=HTML
            r#"<div class="min-h-screen flex items-center justify-center p-4 md:p-6">
                <div class="max-w-md w-full">
                    <div class="flex flex-col items-center mb-8">
                        <div class="w-16 h-16 bg-primary/10 text-primary rounded-2xl flex items-center justify-center font-bold text-3xl mb-4 shadow-inner">
                            S
                        </div>
                        <h2 class="text-3xl font-bold text-slate-900 dark:text-white text-center">Control Plane</h2>
                        <p class="text-slate-500 dark:text-slate-400 font-medium text-center">Platform Owner Setup</p>
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
            <form action="/saas/login" method="POST" class="space-y-4" data-turbo="false">
                {user_input}
                {pass_input}
                {submit_button}
            </form>"#,
            error_alert = error_alert,
            user_input = components::input("Username", "username", "text", "Enter username", true),
            pass_input = components::input("Password", "password", "password", "••••••••", true),
            submit_button = components::button_primary("Sign In to Control Plane", true)
        );

        let content = format!(
            //language=HTML
            r#"<div class="min-h-screen flex items-center justify-center p-4 md:p-6">
                <div class="max-w-md w-full">
                    <div class="flex flex-col items-center mb-8 text-center">
                        <div class="w-16 h-16 bg-primary/10 text-primary rounded-2xl flex items-center justify-center font-bold text-3xl mb-4 shadow-inner">
                            S
                        </div>
                        <h2 class="text-3xl font-bold text-slate-900 dark:text-white">Admin Login</h2>
                        <p class="text-slate-500 dark:text-slate-400 font-medium">Platform Control Plane</p>
                    </div>

                    {card}

                    <div class="mt-8 text-center">
                        <a href="/" class="text-sm text-slate-500 hover:text-primary transition-colors">← Back to home</a>
                    </div>
                </div>
            </div>"#,
            card = components::card(form_content)
        );

        render_layout(LayoutContext::default(), content)
    }
}
