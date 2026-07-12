use crate::view::LayoutView::{render_layout, LayoutContext};

pub fn render_common_login(error: Option<String>) -> String {
    let error_alert = match error {
        Some(err) => format!(
            /* html */
            r#"<div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4 text-sm">{err}</div>"#,
            err = err
        ),
        None => "".to_string(),
    };

    let content = format!(
        /* html */
        r#"<div class="min-h-screen flex items-center justify-center p-4 md:p-6">
            <div class="max-w-md w-full p-8 bg-white dark:bg-slate-900 rounded-3xl shadow-2xl border dark:border-slate-800">
                <div class="flex flex-col items-center mb-8">
                    <div class="w-16 h-16 bg-primary/10 text-primary rounded-2xl flex items-center justify-center font-bold text-3xl mb-4 shadow-inner">
                        P
                    </div>
                    <h2 class="text-3xl font-bold text-slate-900 dark:text-white text-center">Portal Access</h2>
                    <p class="text-slate-500 dark:text-slate-400 font-medium text-center text-sm">Sign in to your specific institution</p>
                </div>

                {error_alert}

                <form action="/login" method="POST" class="space-y-4">
                    <div>
                        <label class="block text-xs uppercase tracking-wider font-bold text-slate-500 dark:text-slate-400 mb-2">Institution Slug</label>
                        <input type="text" name="slug" required placeholder="e.g. demo-school" class="w-full px-4 py-3.5 rounded-2xl border border-slate-200 dark:border-slate-800 dark:bg-slate-950 dark:text-white focus:ring-2 focus:ring-primary outline-none transition-all">
                    </div>
                    <div>
                        <label class="block text-xs uppercase tracking-wider font-bold text-slate-500 dark:text-slate-400 mb-2">Username</label>
                        <input type="text" name="username" required class="w-full px-4 py-3.5 rounded-2xl border border-slate-200 dark:border-slate-800 dark:bg-slate-950 dark:text-white focus:ring-2 focus:ring-primary outline-none transition-all">
                    </div>
                    <div>
                        <label class="block text-xs uppercase tracking-wider font-bold text-slate-500 dark:text-slate-400 mb-2">Password</label>
                        <input type="password" name="password" required class="w-full px-4 py-3.5 rounded-2xl border border-slate-200 dark:border-slate-800 dark:bg-slate-950 dark:text-white focus:ring-2 focus:ring-primary outline-none transition-all">
                    </div>
                    <button type="submit" class="w-full bg-primary hover:bg-primary-600 text-white font-bold py-4 rounded-2xl mt-6 shadow-xl shadow-primary/20 transition-all transform hover:-translate-y-1 active:scale-95">
                        Access Portal
                    </button>
                </form>
                
                <div class="mt-8 pt-6 border-t dark:border-slate-800 text-center flex flex-col gap-3">
                    <a href="/saas/login" class="text-sm text-slate-500 hover:text-primary transition-colors">Control Plane Login</a>
                    <a href="/" class="text-sm text-slate-500 hover:text-primary transition-colors">← Back to home</a>
                </div>
            </div>
        </div>"#,
        error_alert = error_alert
    );

    render_layout(LayoutContext::default(), content)
}
