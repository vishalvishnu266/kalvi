use crate::view::{render_layout, LayoutContext};
use std::collections::HashMap;

pub struct OnboardingView;

impl OnboardingView {
    pub fn render_form(field_errors: HashMap<String, String>, general_error: Option<String>) -> String {
        use crate::view::components;
        let error_alert = general_error.as_deref().map(components::alert_error).unwrap_or_default();

        let form_content = format!(
            r###"<turbo-frame id="registration-form">
                {error_alert}
                <form action="/registration" method="POST" class="space-y-6">
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                        {name_input}
                        {tenant_input}
                    </div>
                    
                    <div class="relative py-4">
                        <div class="absolute inset-0 flex items-center"><span class="w-full border-t dark:border-slate-800"></span></div>
                        <div class="relative flex justify-center text-xs uppercase"><span class="bg-white dark:bg-slate-950 px-2 text-slate-500 font-bold">Admin Credentials</span></div>
                    </div>

                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                        {user_input}
                        {pass_input}
                    </div>

                    {submit_button}
                </form>
            </turbo-frame>"###,
            error_alert = error_alert,
            name_input = components::input("Institution Name", "name", "text", "City High School", true, field_errors.get("name").map(|s| s.as_str())),
            tenant_input = components::input("URL Name (Tenant)", "tenant", "text", "city-high", true, field_errors.get("tenant").map(|s| s.as_str())),
            user_input = components::input("Admin Username", "admin_username", "text", "admin", true, field_errors.get("admin_username").map(|s| s.as_str())),
            pass_input = components::input("Admin Password", "admin_password", "password", "••••••••", true, field_errors.get("admin_password").map(|s| s.as_str())),
            submit_button = components::button_primary("Create My Institution", true)
        );

        let content = format!(
            //language=HTML
            r###"<div class="min-h-screen flex items-center justify-center p-4 bg-slate-50 dark:bg-slate-950">
                <div class="max-w-lg w-full">
                    <div class="text-center mb-8">
                        <h1 class="text-3xl font-extrabold text-slate-900 dark:text-white">Get Started</h1>
                        <p class="text-slate-500 mt-2">Create your isolated institution workspace</p>
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

    pub fn render_success(tenant_name: &str, tenant_slug: &str) -> String {
        let content = format!(
            //language=HTML
            r###"<div class="min-h-screen flex items-center justify-center p-4 bg-slate-50 dark:bg-slate-950 text-center">
                <div class="max-w-md w-full">
                    <div class="bg-emerald-500 text-white w-20 h-20 rounded-full flex items-center justify-center mx-auto mb-6 shadow-lg shadow-emerald-500/20">
                        <svg class="w-10 h-10" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M5 13l4 4L19 7" />
                        </svg>
                    </div>
                    <h1 class="text-3xl font-bold mb-2">{} Ready!</h1>
                    <p class="text-slate-500 mb-8">Your institution database has been initialized and secured.</p>
                    <a href="/web/{}/login" class="inline-block bg-primary text-white font-bold py-4 px-10 rounded-xl shadow-lg shadow-primary/20 hover:bg-primary-600 transition-all">
                        Login to Portal
                    </a>
                </div>
            </div>"###,
            tenant_name, tenant_slug
        );

        render_layout(LayoutContext::default(), content)
    }
}
