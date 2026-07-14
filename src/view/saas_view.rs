use crate::view::{render_layout, LayoutContext};
use std::collections::HashMap;

pub struct SaasView;

impl SaasView {
    pub fn render_onboard(field_errors: HashMap<String, String>, general_error: Option<String>) -> String {
        use crate::view::components;
        let error_alert = general_error.as_deref().map(components::alert_error).unwrap_or_default();

        let form_content = format!(
            r###"<turbo-frame id="saas-form">
                {error_alert}
                <form action="/saas/onboard" method="POST" class="space-y-4">
                    {user_input}
                    {name_input}
                    {pass_input}
                    {submit_button}
                </form>
            </turbo-frame>"###,
            error_alert = error_alert,
            user_input = components::input("Admin Username", "username", "text", "admin", true, field_errors.get("username").map(|s| s.as_str())),
            name_input = components::input("Full Name", "full_name", "text", "System Admin", true, field_errors.get("full_name").map(|s| s.as_str())),
            pass_input = components::input("Password", "password", "password", "••••••••", true, field_errors.get("password").map(|s| s.as_str())),
            submit_button = components::button_primary("Initialize Platform", true)
        );

        let content = format!(
            //language=HTML
            r###"<div class="min-h-screen flex items-center justify-center p-4 bg-slate-50 dark:bg-slate-950">
                <div class="max-w-md w-full">
                    <div class="text-center mb-8">
                        <h1 class="text-3xl font-extrabold text-slate-900 dark:text-white">Control Plane</h1>
                        <p class="text-slate-500 mt-2">Initialize your ERP platform</p>
                    </div>
                    {card}
                </div>
            </div>"###,
            card = components::card(form_content)
        );

        render_layout(LayoutContext::default(), content)
    }

    pub fn render_login(field_errors: HashMap<String, String>, general_error: Option<String>) -> String {
        use crate::view::components;
        let error_alert = general_error.as_deref().map(components::alert_error).unwrap_or_default();

        let form_content = format!(
            r###"<turbo-frame id="saas-form">
                {error_alert}
                <form action="/saas/login" method="POST" class="space-y-4">
                    {user_input}
                    {pass_input}
                    {submit_button}
                </form>
            </turbo-frame>"###,
            error_alert = error_alert,
            user_input = components::input("Username", "username", "text", "admin", true, field_errors.get("username").map(|s| s.as_str())),
            pass_input = components::input("Password", "password", "password", "••••••••", true, field_errors.get("password").map(|s| s.as_str())),
            submit_button = components::button_primary("Sign In to Control Plane", true)
        );

        let content = format!(
            //language=HTML
            r###"<div class="min-h-screen flex items-center justify-center p-4 bg-slate-50 dark:bg-slate-950">
                <div class="max-w-md w-full">
                    <div class="text-center mb-8">
                        <h1 class="text-3xl font-extrabold text-slate-900 dark:text-white">Admin Login</h1>
                        <p class="text-slate-500 mt-2">Platform Control Plane</p>
                    </div>
                    {card}
                    <div class="mt-8 text-center text-sm">
                        <a href="/" class="text-slate-400 hover:text-primary transition-colors">← Back to home</a>
                    </div>
                </div>
            </div>"###,
            card = components::card(form_content)
        );

        render_layout(LayoutContext::default(), content)
    }
}
