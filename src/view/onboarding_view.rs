use crate::view::{render_layout, LayoutContext};

pub fn render_form(error: Option<String>) -> String {
    let error_alert = match error {
        Some(err) => format!(
            //language=HTML
            r#"<div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded relative mb-4" role="alert">
                <span class="block sm:inline">{err}</span>
            </div>"#,
            err = err
        ),
        None => "".to_string(),
    };

    let content = format!(
        //language=HTML
        r#"<div class="min-h-screen flex items-center justify-center p-4 md:p-6">
            <div class="max-w-lg w-full p-8 bg-white dark:bg-slate-900 rounded-3xl shadow-2xl border dark:border-slate-800">
                <div class="flex flex-col items-center mb-8">
                    <div class="w-16 h-16 bg-primary/10 text-primary rounded-2xl flex items-center justify-center font-bold text-3xl mb-4 shadow-inner">
                        O
                    </div>
                    <h2 class="text-3xl font-bold text-slate-900 dark:text-white">New Institution</h2>
                    <p class="text-slate-500 dark:text-slate-400 font-medium text-center">Set up your isolated ERP workspace</p>
                </div>

                {error_alert}

                <form action="/registration" method="POST" class="space-y-6">
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                        <div>
                            <label class="block text-xs uppercase tracking-wider font-bold text-slate-500 dark:text-slate-400 mb-2">Institution Name</label>
                            <input type="text" name="name" required class="w-full px-4 py-3 rounded-2xl border border-slate-200 dark:border-slate-800 dark:bg-slate-950 dark:text-white focus:ring-2 focus:ring-primary outline-none transition-all">
                        </div>
                        <div>
                            <label class="block text-xs uppercase tracking-wider font-bold text-slate-500 dark:text-slate-400 mb-2">Slug</label>
                            <input type="text" name="slug" required placeholder="demo-school" class="w-full px-4 py-3 rounded-2xl border border-slate-200 dark:border-slate-800 dark:bg-slate-950 dark:text-white focus:ring-2 focus:ring-primary outline-none transition-all">
                        </div>
                    </div>
                    
                    <div class="relative py-4">
                        <div class="absolute inset-0 flex items-center"><span class="w-full border-t dark:border-slate-800"></span></div>
                        <div class="relative flex justify-center text-xs uppercase"><span class="bg-white dark:bg-slate-900 px-2 text-slate-500 font-bold">Admin Credentials</span></div>
                    </div>

                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                        <div>
                            <label class="block text-xs uppercase tracking-wider font-bold text-slate-500 dark:text-slate-400 mb-2">Admin Username</label>
                            <input type="text" name="admin_username" required class="w-full px-4 py-3 rounded-2xl border border-slate-200 dark:border-slate-800 dark:bg-slate-950 dark:text-white focus:ring-2 focus:ring-primary outline-none transition-all">
                        </div>
                        <div>
                            <label class="block text-xs uppercase tracking-wider font-bold text-slate-500 dark:text-slate-400 mb-2">Admin Password</label>
                            <input type="password" name="admin_password" required class="w-full px-4 py-3 rounded-2xl border border-slate-200 dark:border-slate-800 dark:bg-slate-950 dark:text-white focus:ring-2 focus:ring-primary outline-none transition-all">
                        </div>
                    </div>

                    <button type="submit" class="w-full bg-primary hover:bg-primary-600 text-white font-bold py-4 rounded-2xl shadow-xl shadow-primary/20 transition-all transform hover:-translate-y-1 active:scale-95">
                        Initialize Institution
                    </button>
                </form>

                <div class="mt-8 text-center">
                    <a href="/" class="text-sm text-slate-500 hover:text-primary transition-colors">← Back to home</a>
                </div>
            </div>
        </div>"#,
        error_alert = error_alert
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
            <a href="/{slug}/login" class="inline-block bg-primary hover:bg-primary-600 text-white font-bold py-4 px-10 rounded-2xl shadow-xl shadow-primary/20 transition-all transform hover:-translate-y-1 active:scale-95">
                Go to Dashboard Login
            </a>
        </div>"#,
        slug = slug,
        name = name
    );

    render_layout(LayoutContext::default(), content)
}
