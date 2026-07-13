use crate::view::{render_layout, LayoutContext};
use crate::model::Tenant;

pub fn render_login(tenant: &Tenant, error: Option<String>) -> String {
    use crate::view::components;
    
    let error_alert = error.map(|err| components::alert(&err, true)).unwrap_or_default();

    let form_content = format!(
        r#"{error_alert}
        <form action="/web/{slug}/login" method="POST" class="space-y-5" data-turbo="false">
            {username_input}
            {password_input}
            {submit_button}
        </form>"#,
        error_alert = error_alert,
        slug = tenant.slug,
        username_input = components::input("Username", "username", "text", "Enter your username", true),
        password_input = components::input("Password", "password", "password", "••••••••", true),
        submit_button = components::button_primary("Sign In", true)
    );

    let content = format!(
        //language=HTML
        r#"<div class="min-h-screen flex items-center justify-center p-4 md:p-6 relative overflow-hidden">
            <div class="max-w-md w-full relative z-10">
                <div class="flex flex-col items-center mb-8 text-center">
                    <div class="w-16 h-16 bg-primary/10 text-primary rounded-2xl flex items-center justify-center font-bold text-3xl mb-4 shadow-inner">
                        {logo_char}
                    </div>
                    <h2 class="text-3xl font-bold text-slate-900 dark:text-white">{tenant_name}</h2>
                    <p class="text-slate-500 dark:text-slate-400 font-medium">Please sign in to continue</p>
                </div>

                {card}

                <div class="mt-8 text-center">
                    <a href="/" class="text-sm text-slate-500 hover:text-primary transition-colors">← Back to home</a>
                </div>
            </div>
        </div>"#,
        logo_char = tenant.name.chars().next().unwrap_or('K'),
        tenant_name = tenant.name,
        card = components::card(form_content)
    );

    let ctx = LayoutContext {
        title: format!("Login - {}", tenant.name),
        primary_color: tenant.primary_color.clone(),
        dark_mode: tenant.dark_mode,
        tenant_slug: Some(tenant.slug.clone()),
    };

    render_layout(ctx, content)
}
