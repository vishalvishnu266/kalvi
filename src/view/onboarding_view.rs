use crate::view::{render_layout, LayoutContext};

pub struct OnboardingView;

impl OnboardingView {
    pub fn render_form(error: Option<String>) -> String {
        use crate::view::components;
        
        let error_alert = error.map(|err| components::alert(&err, true)).unwrap_or_default();

        let form_content = format!(
            r#"{error_alert}
            <form action="/registration" method="POST" class="space-y-6" data-turbo="false">
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    {name_input}
                    {slug_input}
                </div>
                
                <div class="relative py-4">
                    <div class="absolute inset-0 flex items-center"><span class="w-full border-t dark:border-slate-800"></span></div>
                    <div class="relative flex justify-center text-xs uppercase"><span class="bg-white dark:bg-slate-900 px-2 text-slate-500 font-bold">Admin Credentials</span></div>
                </div>

                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    {user_input}
                    {pass_input}
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
            r#"<div class="min-h-screen flex items-center justify-center p-4 md:p-6">
                <div class="max-w-lg w-full">
                    <div class="flex flex-col items-center mb-8">
                        <div class="w-16 h-16 bg-primary/10 text-primary rounded-2xl flex items-center justify-center font-bold text-3xl mb-4 shadow-inner">
                            O
                        </div>
                        <h2 class="text-3xl font-bold text-slate-900 dark:text-white">New Institution</h2>
                        <p class="text-slate-500 dark:text-slate-400 font-medium text-center">Set up your isolated ERP workspace</p>
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

    pub fn render_success(slug: &str, name: &str) -> String {
        let content = format!(
            //language=HTML
            r#"<div class="max-w-md mx-auto my-12 p-8 bg-white dark:bg-slate-800 rounded-xl shadow-2xl text-center">
                <div class="w-20 h-20 bg-green-100 text-green-600 rounded-full flex items-center justify-center mx-auto mb-6">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-12 w-12" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                    </svg>
                </div>
                <h2 class="text-3xl font-bold mb-4 text-slate-900 dark:text-white">Institution Ready!</h2>
                <p class="text-slate-600 dark:text-slate-400 mb-8">
                    <strong>{name}</strong> has been successfully initialized. You can now log in to your tenant dashboard.
                </p>
                <a href="/web/{slug}/login" class="inline-block bg-primary hover:bg-primary-600 text-white font-bold py-4 px-10 rounded-2xl shadow-xl shadow-primary/20 transition-all transform hover:-translate-y-1 active:scale-95">
                    Go to Dashboard Login
                </a>
            </div>"#,
            slug = slug,
            name = name
        );

        render_layout(LayoutContext::default(), content)
    }
}
