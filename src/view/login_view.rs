use crate::view::{render_layout, LayoutContext};
use crate::model::Tenant;
use std::collections::HashMap;

pub struct LoginView;

impl LoginView {
    pub fn render_login(tenant: &Tenant, field_errors: HashMap<String, String>, general_error: Option<String>) -> String {
        use crate::view::components;
        let error_alert = general_error.as_deref().map(components::alert_error).unwrap_or_default();

        let form_content = format!(
            r###"<turbo-frame id="login-form">
                {error_alert}
                <form action="/web/{}/login" method="POST" class="space-y-4">
                    {user_input}
                    {pass_input}
                    {submit_button}
                </form>
            </turbo-frame>"###,
            tenant.slug,
            error_alert = error_alert,
            user_input = components::input("Username", "username", "text", "Enter username", true, field_errors.get("username").map(|s| s.as_str())),
            pass_input = components::input("Password", "password", "password", "••••••••", true, field_errors.get("password").map(|s| s.as_str())),
            submit_button = components::button_primary("Sign In", true)
        );

        let content = format!(
            //language=HTML
            r###"<div class="min-h-screen flex items-center justify-center p-4 bg-slate-50 dark:bg-slate-950">
                <div class="max-w-md w-full">
                    <div class="text-center mb-8">
                        <h1 class="text-3xl font-extrabold text-slate-900 dark:text-white">{}</h1>
                        <p class="text-slate-500 mt-2">Sign in to your institution</p>
                    </div>
                    {card}
                    <div class="mt-8 text-center text-sm">
                        <a href="/login" class="text-slate-400 hover:text-primary transition-colors">← Different institution</a>
                    </div>
                </div>
            </div>"###,
            tenant.name,
            card = components::card(form_content)
        );

        render_layout(LayoutContext::default(), content)
    }
}
