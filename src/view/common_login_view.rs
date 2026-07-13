use crate::view::{render_layout, LayoutContext};

pub fn render_common_login(error: Option<String>) -> String {
    use crate::view::components;

    let error_alert = error.map(|err| components::alert(&err, true)).unwrap_or_default();

    let form_content = format!(
        r#"{error_alert}
        <form action="/login" method="POST" class="space-y-4" data-turbo="false">
            {slug_input}
            {username_input}
            {password_input}
            {submit_button}
        </form>"#,
        error_alert = error_alert,
        slug_input = components::input("Institution Slug", "slug", "text", "e.g. demo-school", true),
        username_input = components::input("Username", "username", "text", "Enter username", true),
        password_input = components::input("Password", "password", "password", "••••••••", true),
        submit_button = components::button_primary("Access Portal", true)
    );

    let content = format!(
        //language=HTML
        r#"<div class="min-h-screen flex items-center justify-center p-4 md:p-6">
            <div class="max-w-md w-full">
                <div class="flex flex-col items-center mb-8">
                    <div class="w-16 h-16 bg-primary/10 text-primary rounded-2xl flex items-center justify-center font-bold text-3xl mb-4 shadow-inner">
                        P
                    </div>
                    <h2 class="text-3xl font-bold text-slate-900 dark:text-white text-center">Portal Access</h2>
                    <p class="text-slate-500 dark:text-slate-400 font-medium text-center text-sm">Sign in to your specific institution</p>
                </div>

                {card}
                
                <div class="mt-8 pt-6 border-t dark:border-slate-800 text-center flex flex-col gap-3">
                    <a href="/saas/login" class="text-sm text-slate-500 hover:text-primary transition-colors">Control Plane Login</a>
                    <a href="/" class="text-sm text-slate-500 hover:text-primary transition-colors">← Back to home</a>
                </div>
            </div>
        </div>"#,
        card = components::card(form_content)
    );

    render_layout(LayoutContext::default(), content)
}
