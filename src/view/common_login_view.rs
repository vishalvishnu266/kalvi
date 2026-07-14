use crate::view::{render_layout, LayoutContext};
use std::collections::HashMap;

pub struct CommonLoginView;

impl CommonLoginView {
    pub fn render_common_login(field_errors: HashMap<String, String>, general_error: Option<String>) -> String {
        use crate::view::components;
        let error_alert = general_error.as_deref().map(components::alert_error).unwrap_or_default();

        let form_content = format!(
            r###"<turbo-frame id="login-form">
                {error_alert}
                <form action="/login" method="POST" class="space-y-4">
                    {tenant_input}
                    {user_input}
                    {pass_input}
                    {submit_button}
                </form>
            </turbo-frame>"###,
            error_alert = error_alert,
            tenant_input = components::input("Institution (Tenant)", "tenant", "text", "city-high", true, field_errors.get("tenant").map(|s| s.as_str())),
            user_input = components::input("Username", "username", "text", "admin", true, field_errors.get("username").map(|s| s.as_str())),
            pass_input = components::input("Password", "password", "password", "••••••••", true, field_errors.get("password").map(|s| s.as_str())),
            submit_button = components::button_primary("Sign In to Portal", true)
        );

        let content = format!(
            //language=HTML
            r###"<div class="min-h-screen flex items-center justify-center p-4 bg-slate-50 dark:bg-slate-950">
                <div class="max-w-md w-full">
                    <div class="text-center mb-8">
                        <h1 class="text-3xl font-extrabold text-slate-900 dark:text-white">Portal Login</h1>
                        <p class="text-slate-500 mt-2">Access your institution workspace</p>
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
